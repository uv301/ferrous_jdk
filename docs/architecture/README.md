# FerrousJDK 架构文档

本目录包含 FerrousJDK 的详细架构设计文档。

## 文档索引

| 文档 | 描述 |
|------|------|
| [overall-arch.md](./overall-arch.md) | 整体架构设计，包含项目定位、设计原则、核心 crate 定义 |
| [jvm-arch.md](./jvm-arch.md) | JVM 核心架构，包含类加载、运行时数据区、解释器、JIT、GC |
| [stdlib-arch.md](./stdlib-arch.md) | Java 标准库架构，包含 java.lang、java.util、java.io 等 |
| [build-arch.md](./build-arch.md) | 构建系统架构，包含构建流程、产物对齐、跨平台编译 |

## 核心组件

```
ferrous-utils ──────┬──► ferrous-core ──┬──► ferrous-interpreter ──┬──► ferrous-jit ──────► ferrous-build
                   │                  │                          │
                   │                  │                          └──► ferrous-stdlib ──┘
                   │                  │
                   │                  ├──► ferrous-jni
                   │                  │
                   │                  └──► ferrous-jvmti
                   │
ferrous-tools ─────┘
```

## 快速导航

### JVM 核心

- [运行时数据区](./jvm-arch.md#2-运行时数据区)
- [类加载子系统](./jvm-arch.md#1-类加载子系统)
- [字节码解释器](./jvm-arch.md#3-字节码解释器)
- [JIT 编译器](./jvm-arch.md#4-jit-即时编译器)
- [垃圾回收器](./jvm-arch.md#5-垃圾回收器)

### 标准库

- [java.lang 核心类](./stdlib-arch.md#41-javalang)
- [java.util 集合框架](./stdlib-arch.md#42-javautil)
- [java.io 输入输出](./stdlib-arch.md#43-javaio)
- [java.nio 新 IO](./stdlib-arch.md#44-javanio)

### 构建系统

- [构建配置](./build-arch.md#2-构建配置)
- [产物目录结构](./build-arch.md#3-产物目录结构)
- [跨平台构建](./build-arch.md#5-跨平台构建)
- [产物校验](./build-arch.md#6-产物校验)

---

**最后更新**: 2026-04-02
