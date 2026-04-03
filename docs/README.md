# FerrousJDK 文档

本目录包含 FerrousJDK 的完整项目文档。

## 文档概览

| 目录 | 描述 |
|------|------|
| [architecture/](./architecture/) | 架构设计文档 |
| [changelog/](./changelog/) | 版本变更日志 |
| [dev-guide/](./dev-guide/) | 开发者指南 |
| [reference/](./reference/) | 规范索引 |
| [roadmap/](./roadmap/) | 开发路线图 |
| [user-guide/](./user-guide/) | 用户指南 |

## 快速入门

### 开发者

1. [开发环境搭建](./dev-guide/env-setup.md) - 搭建本地开发环境
2. [模块开发指南](./dev-guide/module-guide.md) - 了解 crate 结构
3. [native 方法重写](./dev-guide/native-rewrite.md) - 标准库 native 方法重写
4. [贡献指南](../CONTRIBUTING.md) - 如何参与贡献

### 用户

1. [快速开始](./user-guide/quick-start.md) - 安装和运行 FerrousJDK
2. [命令参考](./user-guide/command-ref.md) - JDK 工具使用
3. [性能调优](./user-guide/tuning-guide.md) - JVM 性能调优
4. [兼容性说明](./user-guide/compatibility.md) - 与 OpenJDK 的兼容性

## 架构文档

### 核心架构

| 文档 | 描述 |
|------|------|
| [整体架构](./architecture/overall-arch.md) | 项目定位、设计原则、crate 定义 |
| [JVM 核心](./architecture/jvm-arch.md) | 类加载、运行时、GC、解释器、JIT |
| [标准库架构](./architecture/stdlib-arch.md) | java.lang、java.util、java.io、java.nio |
| [构建系统](./architecture/build-arch.md) | 构建流程、产物对齐、跨平台编译 |

## 开发路线图

### 当前阶段

**阶段 0-1：仓库初始化与规范制定**（🔄 85% 进行中）

### 里程碑进度

| 阶段 | 状态 | 说明 |
|------|------|------|
| 0-1 | 🔄 85% | 仓库初始化 |
| 0-2 | 📋 | 基础设施 |
| 1-1 | 📋 | ClassFile 解析器 |
| 1-2 | 📋 | HelloWorld MVP |

详见 [完整路线图](./roadmap/README.md)

## 版本信息

- **当前版本**: Unreleased (v0.1.0-jdk8-ecj-mvp 开发中)
- **最新正式版**: 无
- **下一步发布**: v0.0.1-init

详见 [变更日志](./changelog/CHANGELOG.md)

## 相关链接

- [GitHub 仓库](https://github.com/uv301/ferrous_jdk)
- [贡献指南](../CONTRIBUTING.md)
- [行为准则](../CODE_OF_CONDUCT.md)
- [规范索引](./reference/index.md)

---

**最后更新**: 2026-04-02
