# FerrousJDK 规范索引

本文档提供 FerrousJDK 遵循的关键规范的快速索引。

## 1. Java 规范

### 1.1 Java Language Specification (JLS)

| 章节 | 主题 | 描述 |
|------|------|------|
| [JLS §3](https://docs.oracle.com/javase/specs/jls/se8/html/jls-3.html) | Types, Values, and Variables | Java 类型系统 |
| [JLS §3.4](https://docs.oracle.com/javase/specs/jls/se8/html/jls-3.html#jls-3.4) | Reference Types and Values | 引用类型 |
| [JLS §3.10](https://docs.oracle.com/javase/specs/jls/se8/html/jls-3.html#jls-3.10) | Literals | 字面量 |
| [JLS §3.12](https://docs.oracle.com/javase/specs/jls/se8/html/jls-3.html#jls-3.12) | Variables | 变量 |
| [JLS §15](https://docs.oracle.com/javase/specs/jls/se8/html/jls-15.html) | Expressions | 表达式 |
| [JLS §17](https://docs.oracle.com/javase/specs/jls/se8/html/jls-17.html) | Threads and Locks | 线程和锁 |

### 1.2 Java Virtual Machine Specification (JVMS8)

| 章节 | 主题 | 描述 |
|------|------|------|
| [JVMS8 §2.2](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.2) | Data Types | JVM 数据类型 |
| [JVMS8 §2.3](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.3) | Primitive Types | 原始类型 |
| [JVMS8 §2.4](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.4) | Reference Types | 引用类型 |
| [JVMS8 §2.5](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.5) | Run-Time Data Areas | 运行时数据区 |
| [JVMS8 §2.5.1](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.5.1) | PC Register | 程序计数器 |
| [JVMS8 §2.5.2](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.5.2) | Java Virtual Machine Stacks | JVM 栈 |
| [JVMS8 §2.5.3](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.5.3) | Heap | 堆 |
| [JVMS8 §2.5.4](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.5.4) | Method Area | 方法区 |
| [JVMS8 §2.5.5](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.5.5) | Run-Time Constant Pool | 运行时常量池 |
| [JVMS8 §2.5.6](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.5.6) | Native Method Stacks | 本地方法栈 |
| [JVMS8 §2.6](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.6) | Frames | 栈帧 |
| [JVMS8 §2.6.1](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.6.1) | Local Variables | 局部变量 |
| [JVMS8 §2.6.2](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.6.2) | Operand Stacks | 操作数栈 |
| [JVMS8 §2.6.3](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.6.3) | Dynamic Linking | 动态链接 |
| [JVMS8 §2.7](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.7) | Representation of Objects | 对象表示 |
| [JVMS8 §2.10](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.10) | Exceptions | 异常 |
| [JVMS8 §2.11](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.11) | Instruction Set Summary | 指令集概要 |
| [JVMS8 §4](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html) | The class File Format | Class 文件格式 |
| [JVMS8 §4.1](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.1) | ClassFile Structure | ClassFile 结构 |
| [JVMS8 §4.4](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4) | Constant Pool | 常量池 |
| [JVMS8 §4.5](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.5) | Fields | 字段 |
| [JVMS8 §4.6](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.6) | Methods | 方法 |
| [JVMS8 §4.7](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.7) | Attributes | 属性 |
| [JVMS8 §5](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-5.html) | Loading, Linking, and Initializing | 加载、链接、初始化 |
| [JVMS8 §5.3](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-5.html#jvms-5.3) | Creating and Loading | 创建和加载 |
| [JVMS8 §5.4](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-5.html#jvms-5.4) | Linking | 链接 |
| [JVMS8 §5.5](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-5.html#jvms-5.5) | Initialization | 初始化 |
| [JVMS8 §6](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html) | Instruction Set | 字节码指令集 |
| [JVMS8 §6.5](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5) | Instruction Descriptions | 指令描述 |
| [JVMS8 §12](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-12.html) | Execution | 执行 |

### 1.3 Java SE 8 API

| 包 | 描述 |
|----|------|
| [java.lang](https://docs.oracle.com/javase/8/docs/api/java/lang/package-summary.html) | 语言基础 |
| [java.util](https://docs.oracle.com/javase/8/docs/api/java/util/package-summary.html) | 工具类 |
| [java.io](https://docs.oracle.com/javase/8/docs/api/java/io/package-summary.html) | 输入输出 |
| [java.nio](https://docs.oracle.com/javase/8/docs/api/java/nio/package-summary.html) | 新 IO |
| [java.net](https://docs.oracle.com/javase/8/docs/api/java/net/package-summary.html) | 网络 |
| [javax.sql](https://docs.oracle.com/javase/8/docs/api/javax/sql/package-summary.html) | JDBC |

## 2. Rust 规范

| 资源 | 描述 |
|------|------|
| [Rust Reference](https://doc.rust-lang.org/reference/) | Rust 语言参考 |
| [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) | Rust API 设计指南 |
| [rustdoc Book](https://doc.rust-lang.org/rustdoc/) | 文档工具 |
| [The Rustonomicon](https://doc.rust-lang.org/nomicon/) | unsafe Rust |
| [Cargo Book](https://doc.rust-lang.org/cargo/) | 包管理器 |
| [Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html) | 测试指南 |

## 3. 工具规范

| 工具 | 描述 |
|------|------|
| [JNI Specification](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/) | JNI 接口规范 |
| [JVMTI Specification](https://docs.oracle.com/javase/8/docs/technotes/guides/jvmti/) | JVM TI 规范 |
| [JVM TI Agents](https://docs.oracle.com/javase/8/docs/technotes/guides/jvmti/) | 调试工具接口 |

## 4. 相关项目

| 项目 | 描述 |
|------|------|
| [OpenJDK](https://openjdk.org/) | OpenJDK 源码 |
| [Cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift) | Rust JIT 编译器后端 |
| [rust-jni](https://github.com/jni-rs/jni-rs) | Rust JNI 绑定 |
| [libm](https://github.com/rust-lang/libm) | Rust 数学库 |

---

**最后更新**: 2026-04-01
