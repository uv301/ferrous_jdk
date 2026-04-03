# FerrousJDK

一个用 Rust 语言实现的 JDK 发行版，以极致性能为首要目标。

## 项目状态

**当前阶段**：✅ 里程碑 0-1 完成（`v0.0.1-init`）

| 里程碑 | 状态 | Tag |
|--------|------|-----|
| 0-1 仓库初始化 | ✅ 完成 | `v0.0.1-init` |
| 0-2 基础设施 | 📋 待开始 | `v0.0.2-infra-ready` |

## 核心特性

- **Pure Rust 实现**：使用 Rust 的内存安全、零成本抽象、高性能并发能力
- **极致性能**：native 方法 100% Rust 重写，消除 JNI 桥接开销
- **100% 兼容**：严格遵循 JVMS、JLS 规范，构建产物与 OpenJDK 无缝对齐
- **多版本支持**：优先实现 JDK8 LTS，后续支持 JDK17、JDK21

## 架构概览

```
ferrous-jdk/
├── crates/
│   ├── ferrous-utils/      # 通用工具库
│   ├── ferrous-core/        # JVM 核心
│   ├── ferrous-interpreter/ # 字节码解释器
│   ├── ferrous-gc/         # 垃圾回收器
│   ├── ferrous-jit/         # JIT 编译器
│   ├── ferrous-jni/         # JNI 实现
│   ├── ferrous-jvmti/       # JVMTI 实现
│   ├── ferrous-stdlib/      # Java 标准库
│   ├── ferrous-javac/       # Java 编译器
│   ├── ferrous-tools/       # JDK 工具
│   └── ferrous-build/        # 构建系统
└── docs/                   # 文档
```

## 快速开始

```bash
# 克隆仓库
git clone https://github.com/uv301/ferrous_jdk.git
cd ferrous_jdk

# 构建项目
cargo build

# 运行测试
cargo test
```

## 文档

- [开发路线图](./docs/roadmap/README.md) - 完整的里程碑规划
- [开发环境搭建](./docs/dev-guide/env-setup.md) - 本地开发环境配置
- [架构设计](./docs/architecture/overview.md) - 系统架构设计
- [贡献指南](./CONTRIBUTING.md) - 如何参与贡献

## 路线图进度

| 阶段 | 里程碑 | 状态 |
|------|--------|------|
| 0-1 | 仓库初始化 | ✅ 完成 |
| 0-2 | 基础设施 | 📋 |
| 1-1 | ClassFile 解析器 | 📋 |
| 1-2 | HelloWorld MVP | 📋 |
| ... | ... | 📋 |

详见 [完整路线图](./docs/roadmap/README.md)

## 参与贡献

我们欢迎各种形式的贡献！请阅读 [贡献指南](./CONTRIBUTING.md) 了解如何参与。

所有参与者必须遵守 [行为准则](./CODE_OF_CONDUCT.md)。

## 许可证

本项目采用 MIT OR Apache-2.0 双协议许可证。详见 [LICENSE-MIT](./LICENSE-MIT) 和 [LICENSE-APACHE](./LICENSE-APACHE)。

## 联系方式

- **GitHub Issues**: https://github.com/uv301/ferrous_jdk/issues
- **Discussions**: https://github.com/uv301/ferrous_jdk/discussions

---

**最后更新**: 2026-04-02
