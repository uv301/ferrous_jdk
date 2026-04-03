# JVM 核心架构

本文档详细描述 FerrousJDK 中 JVM 核心组件的架构设计，遵循 [Java Virtual Machine Specification, Java SE 8 Edition](https://docs.oracle.com/javase/specs/jvms/se8/html/)。内容包括类加载系统、运行时数据区、字节码解释器、JIT 编译器和垃圾回收器。

> **术语说明**：本文档严格遵循 JVMS8 规范定义的数据类型和数据结构名称。

## 1. 类加载子系统

### 1.1 类加载器体系

FerrousJDK 实现完整的类加载器层次结构，严格遵循 JVMS8 §5.3 定义的类加载模型。

```
┌─────────────────────────────────────────────────────────────┐
│                   Bootstrap ClassLoader                      │
│         (启动类加载器，加载核心 Java SE 类)                   │
│              加载 java.* 包中的类                            │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Extension ClassLoader                      │
│              (扩展类加载器，JDK9+ 已移除)                     │
│         加载 <java.home>/lib/ext 中的类                      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                 Application ClassLoader                      │
│              (应用类加载器，加载用户 classpath 中的类)         │
│              也称为 System ClassLoader                       │
└─────────────────────────────────────────────────────────────┘
                            │
            ┌───────────────┼───────────────┐
            ▼               ▼               ▼
      Custom L1        Custom L2       Custom Ln
```

> **注意**：根据 JVMS8 §5.3，类加载采用"双亲委派模型"（Parent Delegation Model），
> 每个类加载请求首先委派给父类加载器处理，只有父类无法完成时才由自身加载。

### 1.2 类加载流程

每个类加载请求按照 JVMS8 §5.3 定义的流程处理：

```rust
pub fn load_class(&self, name: &ClassName) -> Result<ClassFile> {
    // 1. 检查类是否已加载 (JVMS8 §5.3.5)
    if let Some(cached) = self.find_loaded_class(name) {
        return Ok(cached);
    }

    // 2. 双亲委派 (JVMS8 §5.3.1)
    if !should_skip_delegation(self) {
        if let Some(parent) = self.get_parent() {
            return parent.load_class(name);
        }
    }

    // 3. 加载类字节码 (JVMS8 §5.3.5)
    let bytes = self.find_class_bytes(name)?;
    let class = self.define_class(&bytes)?;

    // 4. 链接类 (JVMS8 §5.4)
    self.link_class(&class)?;

    Ok(class)
}
```

> **引用**：JVMS8 §5.3 定义了类加载的完整生命周期。

### 1.3 链接阶段

链接过程按照 JVMS8 §5.4 包含四个严格有序的阶段：

| 阶段 | 描述 | 操作 | JVMS8 章节 |
|------|------|------|------------|
| 验证 (Verification) | 确保 Class 文件格式正确且安全 | 文件格式验证、字节码验证、符号引用验证 | §5.4.1 |
| 准备 (Preparation) | 为静态字段分配内存并初始化为零值 | 内存分配、零值初始化 | §5.4.2 |
| 解析 (Resolution) | 将符号引用替换为直接引用 | 类/接口解析、字段解析、方法解析 | §5.4.3 |
| 初始化 (Initialization) | 执行类构造器 `<clinit>` | 执行静态初始化代码 | §5.5 |

> **注意**：解析阶段可以在需要时延迟进行（延迟解析），以提高性能。

### 1.4 Class 文件格式

Class 文件解析器位于 `ferrous-core/src/classfile/`，实现 JVMS8 §4 定义的完整数据结构：

```rust
// JVMS8 §4.1 ClassFile 结构
pub struct ClassFile {
    pub magic: u32,                    // 0xCAFEBABE (JVMS8 §4.1)
    pub minor_version: u16,             // 次版本号
    pub major_version: u16,            // 主版本号 (52 = JDK8)
    pub constant_pool: ConstantPool,    // 常量池 (JVMS8 §4.4)
    pub access_flags: AccessFlags,      // 访问标志 (JVMS8 §4.1)
    pub this_class: u16,               // 当前类索引
    pub super_class: u16,              // 父类索引
    pub interfaces: Vec<u16>,          // 接口索引表
    pub fields: Vec<FieldInfo>,         // 字段表 (JVMS8 §4.5)
    pub methods: Vec<MethodInfo>,       // 方法表 (JVMS8 §4.6)
    pub attributes: Vec<AttributeInfo>, // 属性表 (JVMS8 §4.7)
}
```

> **引用**：详见 JVMS8 Chapter 4: The class File Format

## 2. 运行时数据区

JVMS8 §2.5 定义了 JVM 运行时数据区，所有线程共享部分和线程私有部分如下：

### 2.1 内存布局

```
┌──────────────────────────────────────────────────────────────────┐
│                    JVM Process Memory (JVMS8 §2.5)                │
├──────────────────────────────────────────────────────────────────┤
│                                                                    │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                    Heap (JVMS8 §2.5.3)                      │  │
│  │           所有线程共享的运行时数据区域                        │  │
│  │           用于分配所有类实例和数组                           │  │
│  │  ┌────────────┐ ┌────────────┐ ┌────────────────────────┐  │  │
│  │  │  Young Gen │ │   Old Gen  │ │    Metaspace           │  │  │
│  │  │  ┌──────┐  │ │            │ │  (JDK8+: 方法区实现)   │  │  │
│  │  │  │ Eden │  │ │            │ │                        │  │  │
│  │  │  ├──────┤  │ │            │ │                        │  │  │
│  │  │  │ S0   │  │ │            │ │                        │  │  │
│  │  │  ├──────┤  │ │            │ │                        │  │  │
│  │  │  │ S1   │  │ │            │ │                        │  │  │
│  │  │  └──────┘  │ │            │ │                        │  │  │
│  │  └────────────┘ └────────────┘ └────────────────────────┘  │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                    │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │              Method Area (JVMS8 §2.5.4)                    │  │
│  │           所有线程共享的方法区                              │  │
│  │  ┌──────────────────┐ ┌────────────────────────────────┐    │  │
│  │  │  Run-Time        │ │       Code for Methods          │    │  │
│  │  │  Constant Pool   │ │       and Constructors          │    │  │
│  │  │  (§2.5.5)        │ │                                 │    │  │
│  │  └──────────────────┘ └────────────────────────────────┘    │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                    │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                    Per-Thread Data Areas                     │  │
│  │                    线程私有的运行时数据区域                   │  │
│  │  ┌─────────────────┐ ┌─────────────────┐ ┌──────────────┐   │  │
│  │  │  PC Register    │ │ Java Virtual    │ │ Native Method│   │  │
│  │  │  (JVMS8 §2.5.1)│ │ Machine Stack   │ │ Stack        │   │  │
│  │  │                 │ │ (JVMS8 §2.5.2) │ │ (§2.5.6)     │   │  │
│  │  │  Frame[]        │ │                 │ │              │   │  │
│  │  └─────────────────┘ └─────────────────┘ └──────────────┘   │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                    │
└──────────────────────────────────────────────────────────────────┘
```

> **引用**：详见 JVMS8 §2.5 Run-Time Data Areas
┌──────────────────────────────────────────────────────────────────┐
│                         JVM Process Memory                        │
├──────────────────────────────────────────────────────────────────┤
│                                                                    │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                      Heap (GC Managed)                       │  │
│  │  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌──────────┐  │  │
│  │  │  Young Gen │ │   Old Gen  │ │  Metaspace │ │ Code Cache│ │  │
│  │  │  ┌──────┐  │ │            │ │            │ │ (JIT)    │ │  │
│  │  │  │ Eden │  │ │            │ │            │ │          │ │  │
│  │  │  ├──────┤  │ │            │ │            │ │          │ │  │
│  │  │  │ S0   │  │ │            │ │            │ │          │ │  │
│  │  │  ├──────┤  │ │            │ │            │ │          │ │  │
│  │  │  │ S1   │  │ │            │ │            │ │          │ │  │
│  │  │  └──────┘  │ │            │ │            │ │          │ │  │
│  │  └────────────┘ └────────────┘ └────────────┘ └──────────┘  │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                    │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                    Non-Heap Memory                          │  │
│  │  ┌──────────────────┐ ┌────────────────────────────────┐    │  │
│  │  │   Metaspace      │ │        Direct Buffers           │    │  │
│  │  │  (Class Metadata)│ │    (NIO ByteBuffer Alloc)       │    │  │
│  │  └──────────────────┘ └────────────────────────────────┘    │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                    │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                      Per-Thread Memory                      │  │
│  │  ┌─────────────┐ ┌─────────────┐ ┌────────────────────────┐ │  │
│  │  │ JVM Stack   │ │ Native Stack│ │   PC Register        │ │  │
│  │  │             │ │             │ │                       │ │  │
│  │  │  Frame[]    │ │  (Native)   │ │  (Per-Thread)        │ │  │
│  │  └─────────────┘ └─────────────┘ └────────────────────────┘ │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                    │
└──────────────────────────────────────────────────────────────────┘
```

### 2.2 对象表示

根据 JVMS8 §2.7，对象是动态分配的类实例或数组。FerrousJDK 采用 Hotspot 风格的对象头设计：

```rust
// JVMS8 §2.7 对象表示
#[repr(C)]
pub struct ObjectHeader {
    // Mark Word (64-bit on 64-bit platforms)
    // - hash code OR age + GC bits + lock state
    // - thread ID OR pointer to monitor
    #[cfg(target_pointer_width = "64")]
    pub mark_word: u64,
    
    // Klass Pointer (压缩类指针，32-bit)
    // 指向方法区中类元数据的指针
    pub klass_pointer: u32,
}

// JVM 堆中的对象 (JVMS8 §2.7)
pub struct HeapObject {
    pub header: ObjectHeader,
    // Instance fields (JVMS8 §2.6.1) follow...
    // 按照 JVMS8 §2.6.1，long 和 double 占用两个 slot
}
```

> **注意**：根据 JVMS8 §2.7，数组对象包含一个 length 字段和数组元素。
> 引用类型数组的元素初始化为 null，原始类型数组的元素初始化为对应的默认值。

### 2.3 Frame 和线程模型

根据 JVMS8 §2.6，每个线程拥有私有的运行时数据区域：

```rust
// JVM 线程 (JVMS8 §2.5)
pub struct JVMThread {
    pub id: ThreadId,
    pub pc_register: Option<NativePointer>,  // PC Register (JVMS8 §2.5.1)
    pub java_stack: JVMStack,                  // Java Virtual Machine Stack (§2.5.2)
    pub native_stack: NativeStack,             // Native Method Stack (§2.5.6)
}

// Frame 结构 (JVMS8 §2.6)
pub struct Frame {
    pub method: Arc<Method>,
    pub pc: u32,                              // 下一条要执行的指令偏移
    pub locals: LocalVariableArray,            // 局部变量表 (§2.6.1)
    pub operand_stack: OperandStack,           // 操作数栈 (§2.6.2)
    pub constant_pool: Arc<ConstantPool>,     // 运行时常量池引用 (§2.6.3)
}

// 局部变量表 (JVMS8 §2.6.1)
// 索引从 0 开始，long 和 double 占两个连续索引
pub struct LocalVariableArray {
    slots: Vec<Value>,  // boolean, byte, char, short, int, float, reference, returnAddress
                       // long 和 double 占用 slots[n] 和 slots[n+1]
}

// 操作数栈 (JVMS8 §2.6.2)
// 后进先出 (LIFO) 栈，最大深度在编译时确定
pub struct OperandStack {
    slots: Vec<Value>,
    max_depth: usize,
}
```

> **注意**：根据 JVMS8 §2.6，Frame 在方法调用时创建，方法正常或异常完成时销毁。
> Frame 从 Java Virtual Machine Stack 分配，不需要连续内存。

## 3. 字节码解释器

### 3.1 指令集概述

FerrousJDK 实现完整的 JVMS8 Chapter 6 定义的字节码指令集。根据 JVMS8 §2.11.1，指令格式为：

```
mnemonic [operand1 [operand2 ...]]
```

操作数栈格式（JVMS8 §6）：

```
..., value1, value2 → ..., result
```

表示 value2 在栈顶，value1 在其下方，执行后弹出并压入 result。

### 3.2 模板解释器架构

FerrousJDK 采用模板解释器架构，针对每条字节码指令生成优化的机器码片段。

```
┌─────────────────────────────────────────────────────────────────┐
│                    Interpreter Loop (§6)                         │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  while (true) {                                                  │
│      opcode = *pc++;  // §6.5                                    │
│      dispatch_table[opcode]();                                   │
│  }                                                               │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
                               │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│  iconst_0     │   │  iload_1      │   │  invokevirtual│
│  opcode = 3   │   │  opcode = 27  │   │  opcode = 182 │
│  §iadd §iconst│   │  §iload       │   │  §invokevirtual│
└───────────────┘   └───────────────┘   └───────────────┘
```

> **引用**：详见 JVMS8 Chapter 6: The Java Virtual Machine Instruction Set

### 3.2 指令实现示例

按照 JVMS8 §6 格式描述实现：

```rust
// iadd 指令实现 (JVMS8 §iadd)
// Format: iadd
// Forms: iadd = 96 (0x60)
// Operand Stack: ..., value1, value2 → ..., result
//     value1 和 value2 必须是 int 类型
fn interpret_iadd(thread: &mut JVMThread) {
    let value2 = thread.frame_mut().operand_stack_mut().pop_int();
    let value1 = thread.frame_mut().operand_stack_mut().pop_int();
    thread.frame_mut().operand_stack_mut().push_int(value1.wrapping_add(value2));
}

// invokevirtual 指令实现 (JVMS8 §invokevirtual)
// Format: invokevirtual indexbyte1 indexbyte2
// Forms: invokevirtual = 182 (0xb6)
// Operand Stack: ..., objectref, [arg1, arg2, ...] → ...
fn interpret_invokevirtual(thread: &mut JVMThread, index: u16) {
    let method = thread.constant_pool().resolve_method(index);
    let objectref = thread.frame_mut().operand_stack_mut().pop_ref();
    
    // NullPointerException (§2.6.2)
    if objectref.is_null() {
        throw_null_pointer_exception(thread);
        return;
    }
    
    // 动态分派 (§2.6.3, §2.9)
    let resolved = objectref.klass().lookup_method(
        method.name(),
        method.descriptor()
    );
    
    // 传递参数
    for arg in resolved.param_types().iter().rev() {
        let arg_value = thread.frame_mut().operand_stack_mut().pop_value();
        // 传递给新帧的局部变量
    }
    
    // 创建新帧并跳转
    thread.push_frame(resolved);
}
```

### 3.3 解释器优化技术

| 优化技术 | 描述 | JVMS8 关联 |
|----------|------|------------|
| 栈顶缓存 (TOSCA) | 将栈顶值保持在寄存器中 | 减少内存访问 |
| 常量池缓存 | 缓存频繁访问的常量池项 | 减少索引计算 |
| 热点计数 | 统计方法/循环执行次数 | 触发 JIT 编译 |
| 栈上替换 (OSR) | 在运行时替换解释器帧为 JIT 编译帧 | 无缝优化 (§2.6) |

> **注意**：这些是实现优化，不影响 JVMS 语义正确性。

## 4. JIT 即时编译器

> **说明**：JIT 编译是实现优化，JVMS 规范不涉及具体编译策略。

### 4.1 分层编译

FerrousJDK 实现三层编译架构：

```
┌─────────────────────────────────────────────────────────────────┐
│                Tier 0: Interpreter (§2.13)                        │
│                      执行所有字节码                                │
│                         │                                         │
│              (热点方法计数达到阈值)                                │
│                         ▼                                         │
│            ┌─────────────────────────┐                           │
│            │   Tier 1: C1 Compiler   │                           │
│            │   快速编译，有限优化     │                           │
│            └─────────────────────────┘                           │
│                         │                                         │
│              (热点方法计数达到阈值)                                │
│                         ▼                                         │
│            ┌─────────────────────────┐                           │
│            │   Tier 2: C2 Compiler   │                           │
│            │   激进优化，深度优化     │                           │
│            └─────────────────────────┘                           │
└─────────────────────────────────────────────────────────────────┘
```

> **引用**：JVMS8 §2.13 "Public Design, Private Implementation" 允许实现者自行选择内部优化策略。
┌─────────────────────────────────────────────────────────────────┐
│                    Tier 0: Interpreter                          │
│           (模板解释器，执行所有字节码)                              │
│                         │                                         │
│              (热点方法计数达到阈值)                                │
│                         ▼                                         │
│            ┌─────────────────────────┐                           │
│            │  Tier 1: C1 Compiler    │                           │
│            │   (快速编译，优化有限)     │                           │
│            └─────────────────────────┘                           │
│                         │                                         │
│              (热点方法计数达到阈值)                                │
│                         ▼                                         │
│            ┌─────────────────────────┐                           │
│            │  Tier 2: C2 Compiler    │                           │
│            │   (激进优化，深度优化)     │                           │
│            └─────────────────────────┘                           │
└─────────────────────────────────────────────────────────────────┘
```

### 4.2 IR (中间表示)

FerrousJDK 定义自定义 IR，支持 SSA 形式：

```rust
// IR Builder 示例
fn build_ir(method: &Method) -> Function {
    let mut func = Function::new(method.name());
    let mut builder = IRBuilder::new(&mut func);
    
    // 创建基本块
    let entry = builder.create_block();
    builder.position_at(entry);
    
    // 生成 IR (§2.11 Types and the Java Virtual Machine)
    let param_a = builder.param(0, ValueType::Int);
    let param_b = builder.param(1, ValueType::Int);
    
    let sum = builder.ins().iadd(param_a, param_b);
    builder.ins().return_value(sum);
    
    func
}
```

### 4.3 优化 Passes

| Pass | 描述 | JVMS8 关联 |
|------|------|------------|
| 常量折叠 | 编译期计算常量表达式 | 减少运行时计算 |
| 死代码消除 | 移除不可达代码 | 减少代码体积 |
| 循环展开 | 展开小循环体 | 减少分支开销 |
| 标量替换 | 用标量替代聚合对象 | 减少内存访问 |
| 逃逸分析 | 分析对象作用域 | 栈上分配、锁消除 |
| 内联 | 方法体内联 | 消除调用开销 |
| 虚调用去虚 | CHA/类型推断优化虚调用 | 消除虚调度开销 |

### 4.4 Cranelift 后端

Cranelift 作为默认 JIT 后端：

```rust
use cranelift::prelude::*;

pub fn compile_to_cranelift(ir: &Function) -> CompiledCode {
    let mut context = Context::for_function(ir.clone());
    
    // 转换为 Cranelift IR
    let mut clif_module = Module::new(isa.clone());
    clif_module.define_function(ir.name(), &mut context).unwrap();
    
    // 编译为机器码
    let code = context.compile(&*isa).unwrap();
    
    code
}
```

> **注意**：Cranelift 是 Rust 原生 JIT 编译器后端，符合 FerrousJDK "Pure Rust" 的设计理念。

## 5. 垃圾回收器

> **说明**：JVMS 不规定具体的垃圾回收算法，只要求符合 §2.5.3 定义的堆语义。

### 5.1 GC 架构抽象

```rust
pub trait GarbageCollector: Send + Sync {
    fn name(&self) -> &'static str;
    
    fn collect(&self, cause: GCCause);
    
    // §2.5.3: 堆是垃圾回收管理的区域
    fn alloc_object(&self, size: usize, klass: &Klass) -> ObjectRef;
    
    fn handle_gc_workers(&self) -> &[GCWorker];
}
```

### 5.2 Serial GC

JDK8 默认 GC，采用串行 Stop-the-World 方式：

```
Young Generation GC (§2.5.3 Heap)
┌─────────────────────────────────────────────────────────────┐
│ Before:                                                     │
│  ┌───────┬───────┬───────┐                                  │
│  │ Eden  │  S0   │  S1   │  ──►  Old Generation              │
│  │ A B C │       │       │                                  │
│  └───────┴───────┴───────┘                                  │
│                                                             │
│ After (S0 <-- From Eden):                                   │
│  ┌───────┬───────┬───────┐                                  │
│  │ Empty │ A B C │       │                                  │
│  └───────┴───────┴───────┘                                  │
└─────────────────────────────────────────────────────────────┘
```

> **注意**：Survivor spaces (S0, S1) 也称为 To/From spaces，采用复制算法。

### 5.3 并行 GC (Parallel GC)

多线程并行收集，利用多核优势：

```rust
pub struct ParallelGC {
    workers: Vec<GCWorker>,
    young_heap: YoungGeneration,
    old_heap: OldGeneration,
}

impl ParallelGC {
    pub fn collect(&self, cause: GCCause) {
        // 暂停所有 Java 线程 (§2.5.2 JVM Stack)
        let _guard = self.pause();
        
        // 并行标记
        self.parallel_mark();
        
        // 并行清理
        self.parallel_sweep();
        
        // 恢复 Java 线程
    }
}
```

### 5.4 ZGC

低延迟 GC，暂停时间控制在亚毫秒级（JDK11+，JDK8 不支持）：

- **着色指针**：利用指针位存储标记信息，避免全局屏障
- **并发阶段**：标记、重定位、引用处理均并发执行
- **读屏障**：仅在读取引用时需要额外处理

### 5.5 根集合枚举

```rust
// GC Roots (实现相关，但需符合 §2.5 语义)
pub enum RootSet {
    // §2.5.1 PC Register
    ActiveJavaThreads,
    
    // §2.5.2 JVM Stack 中的局部变量
    ThreadStackRoots,
    
    // §2.5.3 Heap 中的静态字段
    StaticFieldRoots,
    
    // JNI Handles
    JNIHandles,
    
    // §2.5.4 Method Area
    ClassLoaderRoots,
    
    // 监视器对象
    MonitorRoots,
}
```

## 6. 异常处理

### 6.1 异常类型层次 (JVMS8 §2.10)

```
java.lang.Throwable (§2.10)
├── java.lang.Error
│   ├── VirtualMachineError
│   │   ├── StackOverflowError (§2.5.2)
│   │   └── OutOfMemoryError (§2.5.3)
│   └── ...
└── java.lang.Exception
    ├── RuntimeException
    │   ├── NullPointerException
    │   ├── ClassCastException
    │   └── ...
    └── ...
```

> **引用**：JVMS8 §2.10 Exceptions 定义了 JVM 异常处理机制。

### 6.2 异常抛出机制 (JVMS8 §2.10)

```rust
// athrow 指令实现 (§athrow)
pub fn throw_exception(thread: &mut JVMThread, exception: ObjectRef) -> ! {
    // 设置线程异常
    thread.set_exception(exception);
    
    // 查找异常处理器 (§athrow Linking Exceptions)
    if let Some(handler) = thread.current_frame().find_handler(exception.klass()) {
        // 跳转到 handler (§2.6.5 Abrupt Method Invocation Completion)
        thread.jump_to(handler.pc());
    } else {
        // 异常无法处理，传播给调用者 (§2.6.5)
        unwind_stack(thread);
    }
}
```

> **注意**：根据 JVMS8 §2.10，异常分为两类：同步异常（在程序执行过程中在特定字节码处抛出）和异步异常（可在任何时刻抛出，如 Thread.stop）。

## 7. 反射机制

### 7.1 Reflection 组件

反射机制由 Java SE API 提供，FerrousJDK 在 `java.lang.reflect` 包中实现：

```rust
pub struct Reflector {
    field_cache: RwLock<HashMap<(Class, FieldId), AccessibleObject>>,
    method_cache: RwLock<HashMap<(Class, MethodId), Executable>>,
}

impl Reflector {
    // getfield 指令的反射实现 (§getfield)
    pub fn get_field(&self, obj: ObjectRef, field: &Field) -> JValue {
        let offset = field.offset();
        unsafe { obj.get_field_raw(offset) }
    }
    
    // putfield 指令的反射实现 (§putfield)
    pub fn set_field(&self, obj: ObjectRef, field: &Field, value: JValue) {
        let offset = field.offset();
        unsafe { obj.set_field_raw(offset, value) };
    }
    
    // invokevirtual/invokespecial 的反射实现 (§invokevirtual, §invokespecial)
    pub fn invoke_method(&self, method: &Method, obj: ObjectRef, args: &[JValue]) -> JValue {
        if method.is_native() {
            self.invoke_native(method, obj, args)
        } else {
            self.invoke_bytecode(method, obj, args)
        }
    }
}
```

## 8. invokedynamic 机制

### 8.1 动态调用架构 (JVMS8 §6.5, §invokedynamic)

```
invokedynamic 指令 (§invokedynamic)
      │
      ▼
Bootstrapping Method (BSM) (§invokedynamic)
      │
      ├──► ConstantCallSite (§invokedynamic)
      │         │
      │         └──► MethodHandle chain
      │
      ├──► MutableCallSite
      │         │
      │         └──► 可变目标
      │
      └──► VolatileCallSite
                │
                └──► volatile 语义
```

> **引用**：JVMS8 §invokedynamic 定义了 invokedynamic 指令格式和引导机制。

### 8.2 MethodHandle 实现

```rust
// JVMS8 §invokedynamic MethodHandle 类型
pub enum MethodHandleKind {
    GetField(FieldRef),           // getfield 行为
    GetStatic(MethodRef),         // getstatic 行为
    PutField(FieldRef),           // putfield 行为
    PutStatic(MethodRef),         // putstatic 行为
    InvokeVirtual(MethodRef),     // invokevirtual 行为
    InvokeStatic(MethodRef),     // invokestatic 行为
    InvokeSpecial(MethodRef),     // invokespecial 行为
    NewInvokeSpecial(Constructor), // 对象创建
    Bound(Box<MethodHandle>, JValue),  // 绑定参数
    Adapter(Box<MethodHandle>, MethodType), // 类型适配
}

impl MethodHandle {
    pub fn invoke(&self, thread: &mut JVMThread, args: &mut [JValue]) -> JValue {
        match self {
            MethodHandleKind::Bound(m, bound) => {
                let mut all_args = vec![bound.clone()];
                all_args.extend_from_slice(args);
                m.invoke(thread, &mut all_args)
            }
            // ...
        }
    }
}
```

> **注意**：MethodHandle 不执行方法验证，调用者负责确保类型安全。

## 9. 相关文档

- [整体架构](./overall-arch.md) - FerrousJDK 完整架构概览
- [标准库架构](./stdlib-arch.md) - Java 标准库实现策略
- [构建系统架构](./build-arch.md) - 产物对齐和跨平台编译
