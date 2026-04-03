# FerrousJDK 规范索引

本目录包含 FerrousJDK 遵循的关键规范的快速索引。

## 文档索引

| 文档 | 描述 |
|------|------|
| [index.md](./index.md) | 完整的规范索引，包含 Java、Rust、工具规范链接 |

## 快速索引

### Java 规范

- [JLS (Java Language Specification)](./index.md#11-java-language-specification-jls)
- [JVMS (Java Virtual Machine Specification)](./index.md#12-java-virtual-machine-specification-jvms8)
- [Java SE 8 API](./index.md#13-java-se-8-api)

### Rust 规范

- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [rustdoc Book](https://doc.rust-lang.org/rustdoc/)

### 工具规范

- [JNI Specification](./index.md#3-工具规范)
- [JVMTI Specification](./index.md#3-工具规范)

## 核心规范引用

### JVMS8 关键章节

| 章节 | 主题 |
|------|------|
| §2.5 | Run-Time Data Areas |
| §2.6 | Frames |
| §4 | The class File Format |
| §5 | Loading, Linking, and Initializing |
| §6 | Instruction Set |
| §12 | Execution |

### JLS 关键章节

| 章节 | 主题 |
|------|------|
| §3 | Types, Values, and Variables |
| §15 | Expressions |
| §17 | Threads and Locks |

## 相关项目

- [OpenJDK](https://openjdk.org/) - OpenJDK 源码
- [Cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift) - Rust JIT 编译器后端
- [rust-jni](https://github.com/jni-rs/jni-rs) - Rust JNI 绑定
- [libm](https://github.com/rust-lang/libm) - Rust 数学库

---

**最后更新**: 2026-04-02
