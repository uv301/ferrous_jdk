# Java 标准库架构

本文档描述 FerrousJDK 中 Java 标准库（Java SE Standard Library）的实现策略、架构设计和版本隔离机制。

## 1. 概述

FerrousJDK 的标准库实现由 `ferrous-stdlib` crate 承载，目标是：

- 100% 兼容 Java SE 规范定义的所有类和接口
- native 方法全部使用 Rust 重写，消除 JNI 桥接开销
- 通过特性开关优雅隔离不同 JDK 版本之间的差异

> **规范引用**：
> - [Java SE 8 API Specification](https://docs.oracle.com/javase/8/docs/api/)
> - [JVMS8 Chapter 12: Execution](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-12.html) - 描述了类库职责

## 2. 包结构

```
ferrous-stdlib/
├── src/
│   ├── java/
│   │   ├── lang/           # java.lang 核心包
│   │   │   ├── mod.rs
│   │   │   ├── object.rs
│   │   │   ├── string.rs
│   │   │   ├── class.rs
│   │   │   ├── thread.rs
│   │   │   ├── throwable.rs
│   │   │   └── ...
│   │   ├── util/           # java.util 工具包
│   │   │   ├── mod.rs
│   │   │   ├── arraylist.rs
│   │   │   ├── hashmap.rs
│   │   │   ├── concurrent/
│   │   │   └── ...
│   │   ├── io/             # java.io 输入输出
│   │   ├── nio/           # java.nio 新IO
│   │   ├── net/           # java.net 网络
│   │   ├── security/     # java.security 安全
│   │   ├── math/          # java.math 数学
│   │   └── text/          # java.text 文本处理
│   ├── jdk/               # JDK 专用包
│   │   ├── internal/      # sun.* 内部实现
│   │   ├── tool/          # jdk.* 工具类
│   │   └── ...
│   ├── native/            # Rust native 方法实现
│   │   ├── mod.rs
│   │   ├── java_lang_Object.rs
│   │   ├── java_lang_String.rs
│   │   ├── java_lang_Thread.rs
│   │   └── ...
│   ├── alloc/             # Java 对象分配器
│   ├── extern.rs          # FFI 声明
│   └── lib.rs
├── resources/             # 静态资源
│   └── charsets/         # 字符集数据
└── Cargo.toml
```

## 3. 核心设计原则

### 3.1 Pure Rust 实现

所有标准库类均使用 Rust 实现，包括：

```rust
// java.lang.String 的 Rust 实现 (JLS §3.10.5)
// String 使用 UTF-16 编码，内部 char[] 存储
pub struct JString {
    value: Vec<u16>,      // UTF-16 字符数组 (JLS §3.10.5)
    hash: Cell<Option<u32>>,
    coder: Cell<u8>,       // LATIN1 or UTF16 (压缩字符串)
}

impl JString {
    pub fn new(s: &str) -> Self {
        JString {
            value: s.encode_utf16().collect(),
            hash: Cell::new(None),
            coder: Cell::new(if s.is_ascii() { LATIN1 } else { UTF16 }),
        }
    }
    
    // JLS §3.10.5: charAt
    pub fn char_at(&self, index: usize) -> char {
        let code = self.value[index];
        char::from_u32(code as u32).unwrap()
    }
    
    // JLS §3.10.5: substring
    pub fn substring(&self, begin: usize, end: usize) -> JString {
        JString::new(&self.value[begin..end])
    }
}
```

### 3.2 native 方法 Rust 重写

native 方法不再通过 JNI 调用 C 代码，而是直接在 Rust 中实现：

```rust
// java.lang.Object.registerNatives() 的 Rust 实现
// 遵循 JNI 命名约定
#[no_mangle]
pub extern "system" fn Java_java_lang_Object_registerNatives(
    env: *mut JNIEnv, 
    class: jclass
) -> jint {
    let native_methods = [
        ("hashCode", "()I", hash_code as *const c_void),
        ("getClass", "()Ljava/lang/Class;", get_class as *const c_void),
        ("clone", "()Ljava/lang/Object;", object_clone as *const c_void),
        ("notify", "()V", object_notify as *const c_void),
        ("notifyAll", "()V", object_notify_all as *const c_void),
        ("wait", "(J)V", object_wait as *const c_void),
    ];
    
    let env = unsafe { &mut *env };
    env.register_natives(class, &native_methods);
    0
}

// java.lang.Object.hashCode() 实现
// 返回对象的 identity hash code (JLS §3.10.5)
fn hash_code(_this: &Object) -> i32 {
    let obj = current_thread().current_frame().peek_stack::<Object>();
    obj.identity_hash_code() as i32
}
```

### 3.3 对象分配器

统一的对象分配接口，支持多种分配策略（JVMS8 §2.5.3）：

```rust
// 对象分配器接口
pub trait ObjectAllocator: Send + Sync {
    // §2.5.3: 堆中分配对象
    fn alloc(&self, klass: &Klass, size: usize) -> ObjectRef;
    
    // §2.7: 数组对象分配
    fn alloc_array(&self, klass: &Klass, length: usize) -> ArrayRef;
    
    fn alloc_instance(&self, klass: &Klass) -> ObjectRef;
}

pub struct HeapAllocator {
    gc: Arc<dyn GarbageCollector>,
}

impl ObjectAllocator for HeapAllocator {
    fn alloc(&self, klass: &Klass, size: usize) -> ObjectRef {
        // 线程本地分配缓冲区 (TLAB) 优化
        let tlab = current_thread().tlab();
        if let Some(obj) = tlab.alloc(size) {
            return obj;
        }
        
        // 慢路径：请求新 TLAB 或直接分配 (§2.5.3)
        self.gc.alloc_object(size, klass)
    }
}
```

## 4. 核心包实现

### 4.1 java.lang

语言核心包，包含 Java 语言的基础类型和类（JLS §3）：

| 类 | 实现状态 | Rust 类型 | JLS 关联 |
|---|----------|-----------|----------|
| Object | ✓ 完成 | `crate::java::lang::Object` | §3.4.2 |
| Class | ✓ 完成 | `crate::java::lang::Class` | §3.5 |
| String | ✓ 完成 | `crate::java::lang::JString` | §3.10.5 |
| StringBuilder | ✓ 完成 | `crate::java::lang::StringBuilder` | §3.10.5 |
| Thread | ✓ 完成 | `crate::java::lang::Thread` | JLS §17 |
| ClassLoader | ✓ 完成 | `crate::java::lang::ClassLoader` | JVMS8 §5.3 |
| Throwable | ✓ 完成 | `crate::java::lang::Throwable` | JVMS8 §2.10 |
| System | ✓ 完成 | `crate::java::lang::System` | JLS §3.12 |
| Runtime | ✓ 完成 | `crate::java::lang::Runtime` | JLS §3.12 |

#### System 类实现

```rust
// System 类的关键 native 方法 (JLS §3.12)
pub struct System {}

impl System {
    // System.currentTimeMillis() (JLS §20.18.4)
    // 返回自 UTC 1970-01-01 00:00:00 以来的毫秒数
    #[native]
    pub fn current_time_millis() -> i64 {
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64
    }
    
    // System.arraycopy() (JLS §20.18.1)
    // 数组复制操作
    #[native]
    pub fn arraycopy(src: ObjectRef, src_pos: i32, 
                     dest: ObjectRef, dest_pos: i32, length: i32) {
        // 安全检查和数组复制
        // 可能抛出: ArrayStoreException, IndexOutOfBoundsException, NullPointerException
    }
    
    // System.exit() (JLS §20.18.5)
    // 终止 JVM
    #[native]
    pub fn exit(status: i32) -> ! {
        std::process::exit(status);
    }
}
```

### 4.2 java.util

集合框架和工具类：

| 类/接口 | 实现状态 |
|---------|----------|
| List, ArrayList | ✓ 完成 |
| Map, HashMap | ✓ 完成 |
| Set, HashSet | ✓ 完成 |
| Iterator | ✓ 完成 |
| Collections | ✓ 完成 |
| Arrays | ✓ 完成 |
| ConcurrentHashMap | ✓ 进行中 |

#### HashMap 实现

```rust
pub struct HashMap<K, V> {
    table: AtomicRefCell<Vec<Bucket<K, V>>>,
    size: AtomicUsize,
    load_factor: f64,
}

struct Bucket<K, V> {
    key: Option<K>,
    value: Option<V>,
    hash: u32,
    next: Option<NonNull<Bucket<K, V>>>,
}

impl<K: Eq + Hash, V> HashMap<K, V> {
    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = Self::hash(key);
        let bucket = self.find_bucket(hash)?;
        if bucket.key.as_ref() == Some(key) {
            Some(bucket.value.as_ref().unwrap())
        } else {
            None
        }
    }
    
    pub fn put(&self, key: K, value: V) -> Option<V> {
        // 插入逻辑
    }
}
```

### 4.3 java.io

输入输出流：

```rust
pub trait InputStream {
    fn read(&self) -> Result<i32, IOException>;
    fn read_all(&self, buf: &mut [u8]) -> Result<usize, IOException>;
    fn skip(&self, n: u64) -> Result<u64, IOException>;
    fn available(&self) -> Result<usize, IOException>;
}

pub struct FileInputStream {
    file: File,
    fd: RawFd,
}

pub struct BufferedInputStream<I: InputStream> {
    inner: I,
    buf: Vec<u8>,
    pos: usize,
    count: usize,
}
```

### 4.4 java.nio

新 IO 包，提供缓冲区抽象和通道机制：

```rust
pub trait Buffer {
    fn capacity(&self) -> usize;
    fn position(&self) -> usize;
    fn limit(&self) -> usize;
    fn remaining(&self) -> usize;
    fn has_remaining(&self) -> bool;
}

pub struct ByteBuffer {
    buffer: Vec<u8>,
    capacity: usize,
    position: usize,
    limit: usize,
    mark: Option<usize>,
    order: ByteOrder,
}

impl ByteBuffer {
    pub fn allocate(capacity: usize) -> Self {
        ByteBuffer {
            buffer: vec![0; capacity],
            capacity,
            position: 0,
            limit: capacity,
            mark: None,
            order: ByteOrder::BigEndian,
        }
    }
    
    pub fn put(&mut self, value: u8) -> &mut Self { /* ... */ }
    pub fn get(&mut self) -> u8 { /* ... */ }
    pub fn flip(&mut self) { /* ... */ }
    pub fn compact(&mut self) { /* ... */ }
}
```

## 5. 版本隔离机制

### 5.1 特性开关设计

```toml
# Cargo.toml
[features]
default = ["jdk8"]

# JDK8 基础特性
jdk8 = [
    "ferrous-core/jdk8",
    "ferrous-stdlib-jdk8",
]

# JDK17 新增内容
jdk17 = [
    "ferrous-core/jdk17",
    "ferrous-stdlib-jdk17",
    "ferrous-stdlib-jdk8",  # 继承 JDK8 内容
]

# JDK21 新增内容
jdk21 = [
    "ferrous-core/jdk21",
    "ferrous-stdlib-jdk21",
    "ferrous-stdlib-jdk17",  # 继承 JDK17 内容
    "ferrous-stdlib-jdk8",    # 继承 JDK8 内容
]
```

### 5.2 条件编译

使用 `#[cfg(feature = "xxx")]` 实现条件编译：

```rust
// JDK17+ sealed class 支持
#[cfg(feature = "jdk17")]
pub fn get_permitted_subclasses(&self) -> Vec<Class> {
    self.permitted_subclasses.clone()
}

// JDK8 不支持此方法
#[cfg(not(feature = "jdk17"))]
pub fn get_permitted_subclasses(&self) -> Vec<Class> {
    unreachable!("Not available in JDK8")
}
```

### 5.3 版本差异处理

```rust
// 版本特定的 API 实现
pub mod version_specific {
    #[cfg(feature = "jdk17")]
    pub mod jdk17 {
        use super::super::*;
        
        pub fn sealed_class_check(klass: &Class, subclass: &Class) -> bool {
            klass.permitted().contains(&subclass.name())
        }
    }
    
    #[cfg(feature = "jdk21")]
    pub mod jdk21 {
        use super::super::*;
        
        pub fn virtual_thread_start(thread: &Thread) {
            // 虚拟线程启动逻辑
        }
    }
}
```

## 6. JDK8 到 JDK17 主要差异

| 特性 | JDK8 | JDK17 | 实现方式 |
|------|------|-------|----------|
| 模块系统 (JPMS) | ✗ | ✓ | `jdk17` 特性 |
| record 类型 | ✗ | ✓ | `jdk17` 特性 |
| sealed class | ✗ | ✓ | `jdk17` 特性 |
| pattern matching | ✗ | 部分 | 渐进支持 |
| switch 表达式 | ✗ | ✓ | `jdk17` 特性 |
| text blocks | ✗ | ✓ | `jdk17` 特性 |

## 7. JDK17 到 JDK21 主要差异

| 特性 | JDK17 | JDK21 | 实现方式 |
|------|-------|-------|----------|
| 虚拟线程 | ✗ | ✓ | `jdk21` 特性 |
| 记录模式 | ✗ | ✓ | `jdk21` 特性 |
| switch 模式匹配 | ✗ | ✓ | `jdk21` 特性 |
| 序列化集合 | ✗ | ✓ | `jdk21` 特性 |
| unnamed patterns | ✗ | ✓ | `jdk21` 特性 |

## 8. 性能优化策略

### 8.1 内联优化

高频方法直接内联 Rust 原生实现：

```rust
// String.length() 直接内联
#[inline]
pub fn length(&self) -> usize {
    self.value.len()
}

// Integer.bitCount() 内联实现
#[inline]
pub fn bit_count(i: i32) -> i32 {
    i32::count_ones(i) as i32
}
```

### 8.2 缓存优化

```rust
// String.hashCode 缓存
pub fn hash_code(&self) -> i32 {
    match self.hash.get() {
        Some(h) => h,
        None => {
            let h = self.compute_hash();
            self.hash.set(Some(h));
            h
        }
    }
}
```

### 8.3 懒初始化

```rust
pub struct Class {
    name: String,
    // 懒初始化字段
    #[lazy]
    methods: Vec<Method>,
    
    #[lazy]
    fields: Vec<Field>,
    
    #[lazy]
    annotations: HashMap<String, Vec<Annotation>>,
}
```

## 9. 相关文档

- [整体架构](./overall-arch.md) - FerrousJDK 完整架构概览
- [JVM 核心架构](./jvm-arch.md) - 类加载、运行时、GC 等
- [构建系统架构](./build-arch.md) - 产物对齐和跨平台编译
- [JDK17 适配指南](../dev-guide/jdk17-adapt.md) - 版本差异处理详细说明
