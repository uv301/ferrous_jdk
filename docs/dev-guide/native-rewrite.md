# native 方法重写指南

本文档介绍 FerrousJDK 中 Java 标准库 native 方法的 Rust 重写策略、规范和最佳实践。

## 1. 概述

FerrousJDK 使用 Rust 重写所有 Java 标准库中的 native 方法，消除传统 JNI 桥接开销，提升性能并增强安全性。

### 1.1 为什么重写 native 方法

| 方面 | 传统 JNI | Rust 重写 |
|------|----------|-----------|
| 性能 | JNI 调用开销 (5-10x) | 零开销函数调用 |
| 安全性 | 需要 unsafe C 代码 | Rust 内存安全保证 |
| 可维护性 | C/Rust 混合代码 | 统一 Rust 代码库 |
| 跨平台 | 各自实现 | 一次编写，到处运行 |

### 1.2 native 方法分类

```
native 方法类型
├── VM Native
│   └── Object.hashCode(), Object.clone() 等
├── Math Native
│   └── Math.sin(), Math.cos() 等
├── Class Native
│   └── Class.getDeclaredFields0() 等
├── Thread Native
│   └── Thread.start0(), Thread.sleep() 等
├── I/O Native
│   └── FileInputStream.read0() 等
└── Network Native
    └── PlainSocketImpl.socketConnect() 等
```

## 2. 重写流程

### 2.1 识别 native 方法

1. 在 OpenJDK 源码中查找 `native` 关键字
2. 记录方法签名
3. 查找对应的 C/C++ 实现

```java
// Java 定义 (java.lang.Object)
public class Object {
    private static native void registerNatives();
    public native int hashCode();
    protected native Object clone();
    public final native Class<?> getClass();
    public final native void notify();
    public final native void notifyAll();
    public final native void wait(long timeout) throws InterruptedException;
}
```

### 2.2 创建 Rust 实现

```rust
// src/native/java_lang_Object.rs

use crate::jni::*;
use crate::runtime::thread::current_thread;

/// registerNatives - 注册 native 方法映射
#[no_mangle]
pub extern "system" fn Java_java_lang_Object_registerNatives(env: *mut JNIEnv, class: jclass) -> jint {
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

/// hashCode - 返回对象哈希码
fn hash_code(env: &mut JNIEnv, this: jobject) -> jint {
    let obj = env.get_object(this);
    obj.identity_hash_code() as jint
}

/// getClass - 返回对象的 Class
fn get_class(env: &mut JNIEnv, this: jobject) -> jclass {
    let obj = env.get_object(this);
    let class = obj.get_class();
    env.add_local_ref(class)
}

/// clone - 克隆对象
fn object_clone(env: &mut JNIEnv, this: jobject) -> jobject {
    let obj = env.get_object(this);
    
    // 检查是否可克隆
    if !obj.get_class().is_cloneable() {
        env.throw_new("java/lang/CloneNotSupportedException", None);
        return null();
    }
    
    let clone = obj.clone();
    env.add_local_ref(clone)
}
```

## 3. JNI 环境封装

### 3.1 JNIEnv 封装

```rust
// src/jni/mod.rs

pub struct JNIEnv<'a> {
    inner: &'a mut JNIInvokeInterface,
    locals: LocalRefs<'a>,
}

impl<'a> JNIEnv<'a> {
    pub fn new(raw: *mut JNIInvokeInterface) -> Self {
        JNIEnv {
            inner: unsafe { &mut *raw },
            locals: LocalRefs::new(),
        }
    }
    
    // 对象操作
    pub fn get_object(&self, obj: jobject) -> ObjectRef { /* ... */ }
    pub fn alloc_object(&mut self, class: jclass) -> Result<jobject> { /* ... */ }
    pub fn new_object(&mut self, class: jclass, sig: &str, args: &[JValue]) -> Result<jobject> { /* ... */ }
    
    // 字段操作
    pub fn get_field<T: FromJValue>(&self, obj: jobject, field_id: jfieldID) -> T { /* ... */ }
    pub fn set_field(&self, obj: jobject, field_id: jfieldID, value: JValue) { /* ... */ }
    
    // 方法调用
    pub fn call_method(&self, obj: jobject, method_id: jmethodID, args: &[JValue]) -> Result<JValue> { /* ... */ }
    pub fn call_static_method(&self, class: jclass, method_id: jmethodID, args: &[JValue]) -> Result<JValue> { /* ... */ }
    
    // 异常处理
    pub fn exception_occurred(&self) -> Option<jthrowable> { /* ... */ }
    pub fn exception_clear(&self) { /* ... */ }
    pub fn throw(&self, exception: jthrowable) -> jint { /* ... */ }
    pub fn throw_new(&self, class_name: &str, msg: Option<&str>) -> jint { /* ... */ }
    
    // 引用管理
    pub fn add_local_ref<T: Into<JObject>>(&mut self, obj: T) -> jobject { /* ... */ }
    pub fn delete_local_ref(&mut self, obj: jobject) { /* ... */ }
}
```

### 3.2 常用 JNI 函数封装

```rust
// JNI 函数速查

// 获取 Java 字符串
pub fn get_string_utf(&self, jstr: jstring) -> String { /* ... */ }

// 创建 Java 字符串
pub fn new_string_utf(&self, s: &str) -> Result<jstring> { /* ... */ }

// 数组操作
pub fn get_array_length(&self, array: jarray) -> jsize { /* ... */ }
pub fn get_object_array_element(&self, array: jobjectArray, index: jsize) -> jobject { /* ... */ }
pub fn set_object_array_element(&self, array: jobjectArray, index: jsize, value: jobject) { /* ... */ }

// 字节数组
pub fn get_byte_array_elements(&self, array: jbyteArray, is_copy: &mut jboolean) -> *mut jbyte { /* ... */ }
pub fn release_byte_array_elements(&self, array: jbyteArray, elems: *mut jbyte, mode: jint) { /* ... */ }
```

## 4. 常见 native 方法实现

### 4.1 System native 方法

```rust
// src/native/java_lang_System.rs

/// currentTimeMillis
fn current_time_millis() -> jlong {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as jlong
}

/// nanoTime
fn nano_time() -> jlong {
    std::time::Instant::now()
        .elapsed()
        .as_nanos() as jlong
}

/// arraycopy
fn array_copy(
    _env: &mut JNIEnv,
    _src: jobject,
    src_pos: jint,
    dest: jobject,
    dest_pos: jint,
    length: jint,
) {
    // 实现数组复制逻辑
    // 包括类型检查和边界检查
}

/// identityHashCode
fn identity_hash_code(_env: &mut JNIEnv, obj: jobject) -> jint {
    let obj_ref = unsafe { &*(obj as *const Object) };
    obj_ref.header().hash_code() as jint
}

/// setIn/setOut/setErr
fn set_stream(_env: &mut JNIEnv, stream: jobject) {
    // 设置标准流
}
```

### 4.2 Math native 方法

```rust
// src/native/java_lang_Math.rs

use std::arch::x86_64::*;

const PI: f64 = 3.141592653589793;
const E: f64 = 2.718281828459045;

/// sin
fn sin(_env: &mut JNIEnv, x: jdouble) -> jdouble {
    unsafe { x.to_bits() }; // 转换为 f64
    libm::sin(x)
}

/// cos
fn cos(_env: &mut JNIEnv, x: jdouble) -> jdouble {
    libm::cos(x)
}

/// tan
fn tan(_env: &mut JNIEnv, x: jdouble) -> jdouble {
    libm::tan(x)
}

/// sqrt
fn sqrt(_env: &mut JNIEnv, x: jdouble) -> jdouble {
    libm::sqrt(x)
}

/// pow
fn pow(_env: &mut JNIEnv, x: jdouble, y: jdouble) -> jdouble {
    libm::pow(x, y)
}

/// log
fn log(_env: &mut JNIEnv, x: jdouble) -> jdouble {
    libm::log(x)
}

/// exp
fn exp(_env: &mut JNIEnv, x: jdouble) -> jdouble {
    libm::exp(x)
}
```

### 4.3 Thread native 方法

```rust
// src/native/java_lang_Thread.rs

use crate::runtime::thread;

/// currentThread
fn current_thread(_env: &mut JNIEnv) -> jobject {
    let thread = thread::current();
    let java_thread = thread.java_thread();
    // 返回 Java Thread 对象引用
    java_thread.as_jobject()
}

/// isAlive
fn is_alive(_env: &mut JNIEnv, thread: jobject) -> jboolean {
    let java_thread = unsafe { &*(thread as *const Thread) };
    (java_thread.is_alive() as jboolean)
}

/// start0
fn start0(_env: &mut JNIEnv, thread: jobject) {
    let java_thread = unsafe { &mut *(thread as *mut Thread) };
    java_thread.start();
}

/// stop
fn stop0(_env: &mut JNIEnv, _thread: jobject, _exception: jobject) {
    // JDK 1.1 deprecated, 空实现
}

/// sleep
fn sleep(_env: &mut JNIEnv, millis: jlong, nanos: jint) {
    let duration = std::time::Duration::from_millis(millis as u64)
        + std::time::Duration::from_nanos(nanos as u64);
    std::thread::sleep(duration);
}

/// yield
fn yield_thread(_env: &mut JNIEnv) {
    std::thread::yield_now();
}

/// setPriority0
fn set_priority0(_env: &mut JNIEnv, thread: jobject, priority: jint) {
    let java_thread = unsafe { &mut *(thread as *mut Thread) };
    java_thread.set_priority(priority);
}

/// holdsLock
fn holds_lock(_env: &mut JNIEnv, obj: jobject) -> jboolean {
    let object = unsafe { &*(obj as *const Object) };
    let current = thread::current();
    current.holds_lock(&object) as jboolean
}
```

### 4.4 Class native 方法

```rust
// src/native/java_lang_Class.rs

/// getSuperclass
fn get_superclass(_env: &mut JNIEnv, class: jclass) -> jclass {
    let class_ref = unsafe { &*(class as *const Class) };
    if let Some(super_class) = class_ref.super_class() {
        super_class.as_jclass()
    } else {
        null()
    }
}

/// isInterface
fn is_interface(_env: &mut JNIEnv, class: jclass) -> jboolean {
    let class_ref = unsafe { &*(class as *const Class) };
    class_ref.is_interface() as jboolean
}

/// getName0
fn get_name0(_env: &mut JNIEnv, class: jclass) -> jstring {
    let class_ref = unsafe { &*(class as *const Class) };
    let name = class_ref.name().replace('/', '.');
    _env.new_string_utf(&name).unwrap_or(null())
}

/// desiredAssertionStatus0
fn desired_assertion_status0(_env: &mut JNIEnv, class: jclass) -> jboolean {
    // 根据类加载器和包确定断言状态
    jboolean::FALSE
}

/// getPrimitiveClass
fn get_primitive_class(_env: &mut JNIEnv, name: jstring) -> jclass {
    // 处理基本类型 (int, long, etc.)
}

/// registerNatives
fn register_natives(_env: &mut JNIEnv, class: jclass) -> jint {
    // 注册所有 Class native 方法
}
```

## 5. 性能优化

### 5.1 内联优化

```rust
// 高频方法标记 #[inline]
#[inline]
pub fn identity_hash_code(obj: &Object) -> usize {
    obj.header().hash_code()
}

// 热点方法使用 #[inline(always)]
#[inline(always)]
pub fn current_time_millis() -> i64 {
    // ...
}
```

### 5.2 缓存优化

```rust
// 缓存常用数据
lazy_static! {
    static ref EPOCH_OFFSET: i64 = {
        // 计算 Java epoch 偏移
    };
}

pub fn current_time_millis() -> i64 {
    let unix_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    
    unix_ms - *EPOCH_OFFSET
}
```

### 5.3 SIMD 优化

```rust
// 使用 SIMD 优化数组操作
#[cfg(target_arch = "x86_64")]
pub fn array_copy_simd(src: &[u8], dest: &mut [u8]) {
    unsafe {
        // 使用 AVX 指令批量复制
    }
}
```

## 6. 错误处理

### 6.1 抛出 Java 异常

```rust
fn may_throw_io_error(env: &mut JNIEnv) -> Result<jobject> {
    let result = do_io_operation()?;
    
    match result {
        Ok(value) => Ok(value),
        Err(e) => {
            env.throw_new("java/io/IOException", Some(&e.to_string()));
            Ok(null())
        }
    }
}
```

### 6.2 异常检查

```rust
fn safe_call(env: &mut JNIEnv) -> jint {
    // 调用前检查
    if env.exception_check() {
        return -1;
    }
    
    let result = unsafe_method_call();
    
    // 调用后检查
    if env.exception_check() {
        return -1;
    }
    
    result
}
```

## 7. 测试 native 方法

### 7.1 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_hash_code() {
        let obj = Object::new();
        let hash1 = identity_hash_code(&obj);
        let hash2 = identity_hash_code(&obj);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_array_copy() {
        let mut src = vec![1, 2, 3, 4, 5];
        let mut dest = vec![0; 5];
        array_copy(&mut src, 0, &mut dest, 0, 3);
        assert_eq!(&dest[..3], &[1, 2, 3]);
    }
}
```

### 7.2 集成测试

```bash
# 运行 Java 测试
cd tests
javac NativeMethodTest.java
java NativeMethodTest
```

## 8. 规范清单

native 方法重写检查清单：

- [ ] 方法签名正确匹配
- [ ] 参数类型和顺序正确
- [ ] 返回值类型正确
- [ ] 异常处理正确
- [ ] 引用管理 (Local/Weak/Global)
- [ ] 线程安全 (如果需要)
- [ ] 性能优化 (如果需要)
- [ ] 文档注释完整
- [ ] 测试覆盖

## 9. 参考资源

- [JNI 规范](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/)
- [OpenJDK 源码](https://github.com/openjdk/jdk)
- [Rust FFI NOM](https://docs.rust-embedded.org/book/)
