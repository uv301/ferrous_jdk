# FerrousJDK 开发指南

本目录包含 FerrousJDK 的开发者文档。

## 文档索引

| 文档 | 描述 |
|------|------|
| [env-setup.md](./env-setup.md) | 本地开发环境搭建，包含 Rust 工具链、IDE 配置、调试配置 |
| [module-guide.md](./module-guide.md) | 模块开发指南，包含 crate 结构、特性开关、测试规范、提交代码规范 |
| [native-rewrite.md](./native-rewrite.md) | native 方法重写指南，包含 JNI 封装、常见 native 方法实现、性能优化 |
| [jdk17-adapt.md](./jdk17-adapt.md) | JDK17 适配指南，包含模块系统、Sealed Classes、Records、Pattern Matching |

## 快速导航

### 环境搭建

- [系统要求](./env-setup.md#1-系统要求)
- [Rust 工具链安装](./env-setup.md#2-基础工具安装)
- [获取源码](./env-setup.md#3-获取源码)
- [IDE 设置](./env-setup.md#4-开发工具配置)
- [构建项目](./env-setup.md#5-构建-ferrousjdk)
- [运行测试](./env-setup.md#6-运行测试)
- [调试配置](./env-setup.md#8-调试配置)

### 模块开发

- [项目结构](./module-guide.md#1-项目结构概览)
- [创建新模块](./module-guide.md#3-创建新模块)
- [特性开关](./module-guide.md#5-特性开关使用)
- [测试规范](./module-guide.md#6-测试规范)
- [文档规范](./module-guide.md#7-文档规范)
- [提交代码](./module-guide.md#11-提交代码)

### native 方法重写

- [重写流程](./native-rewrite.md#2-重写流程)
- [JNI 环境封装](./native-rewrite.md#3-jni-环境封装)
- [常见方法实现](./native-rewrite.md#4-常见-native-方法实现)
- [性能优化](./native-rewrite.md#5-性能优化)
- [错误处理](./native-rewrite.md#6-错误处理)

### JDK17 适配

- [模块系统](./jdk17-adapt.md#2-模块系统-jpms)
- [Sealed Classes](./jdk17-adapt.md#3-sealed-classes)
- [Records](./jdk17-adapt.md#4-records)
- [Pattern Matching](./jdk17-adapt.md#5-pattern-matching)
- [API 差异处理](./jdk17-adapt.md#6-api-差异处理)
- [GC 变化](./jdk17-adapt.md#7-gc-变化)

---

**最后更新**: 2026-04-02
