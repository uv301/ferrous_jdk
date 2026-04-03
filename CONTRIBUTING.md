# FerrousJDK 贡献指南

感谢您对 FerrousJDK 项目的兴趣！我们欢迎各种形式的贡献，包括但不限于代码、文档、测试和问题反馈。

## 1. 行为准则

参与本项目的所有成员必须遵守我们的 [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)。我们致力于为所有参与者提供一个友好、安全和包容的环境。

## 2. 如何贡献

### 2.1 报告问题

如果您发现任何问题或 bug，请通过 GitHub Issues 报告。报告时请包含：

- **环境信息**：FerrousJDK 版本、平台、架构
- **问题描述**：详细描述您遇到的问题
- **复现步骤**：如何复现该问题
- **预期行为**：您期望的行为
- **实际行为**：实际发生的行为
- **相关日志**：任何相关的日志输出

### 2.2 提交代码

#### 分支命名规范

| 分支类型 | 命名规范 | 示例 |
|----------|----------|------|
| 功能开发 | `feature/[里程碑id]-[功能名]` | `feature/1-1-classfile-parser` |
| Bug 修复 | `bugfix/[issue-id]-[问题描述]` | `bugfix/123-fix-null-pointer` |
| 发布准备 | `release/jdk8-lts-v[版本号]` | `release/jdk8-lts-v1.0.0` |
| 紧急热修复 | `hotfix/jdk8-lts-v[版本号]` | `hotfix/jdk8-lts-v1.0.1` |

#### 提交信息规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <subject>

<body>

<footer>
```

**类型 (type)**：
- `feat`：新功能
- `fix`：Bug 修复
- `docs`：文档更新
- `style`：代码格式（不影响功能）
- `refactor`：代码重构
- `perf`：性能优化
- `test`：测试相关
- `chore`：构建/工具/依赖

**范围 (scope)**：
- `core`：JVM 核心
- `interpreter`：字节码解释器
- `gc`：垃圾回收器
- `jit`：JIT 编译器
- `stdlib`：标准库
- `tools`：工具链
- `build`：构建系统
- `docs`：文档

**示例**：

```
feat(interpreter): add invokespecial bytecode implementation

Implement invokespecial instruction with superclass constructor call support.

- Add invokespecial handler
- Implement super() call semantics
- Add bytecode verification tests

Closes #123
```

## 3. 开发流程

### 3.1 环境准备

请参考 [docs/dev-guide/env-setup.md](./docs/dev-guide/env-setup.md) 搭建开发环境。

### 3.2 拉取代码

```bash
# 克隆仓库
git clone https://github.com/uv301/ferrous_jdk.git
cd ferrous-jdk

# 添加上游仓库
git remote add upstream https://github.com/uv301/ferrous_jdk.git

# 检出 dev 分支
git checkout dev
```

### 3.3 创建功能分支

```bash
# 确保基于最新的 dev 分支创建
git checkout dev
git pull upstream dev

# 创建功能分支
git checkout -b feature/1-1-classfile-parser
```

### 3.4 开发与测试

```bash
# 开发构建
cargo build

# 格式化代码
cargo fmt

# 运行检查
cargo clippy

# 运行测试
cargo test

# 仅运行特定 crate 测试
cargo test -p ferrous-core
```

### 3.5 提交代码

```bash
# 暂存更改
git add .

# 提交（遵循 Conventional Commits）
git commit -m "feat(core): add ClassFile parser implementation"

# 推送到您的 fork
git push origin feature/1-1-classfile-parser
```

### 3.6 创建 Pull Request

1. 访问 GitHub 仓库页面
2. 点击 "New Pull Request"
3. 选择您的分支并创建 PR
4. 填写 PR 模板中的所有必填项
5. 等待代码审查

### 3.7 代码审查

- 审查者会在 48 小时内回复
- 请及时响应审查意见
- 所有对话解决后，审查者会合并 PR

## 4. 代码规范

### 4.1 Rust 代码规范

- 遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码
- 所有公共 API 必须有文档注释
- 遵循命名规范（snake_case、PascalCase 等）

### 4.2 文档规范

- 所有公共 API 必须有 rustdoc 文档
- 更新相关文档以反映代码更改
- 新功能必须包含使用示例

### 4.3 测试规范

- 所有新功能必须包含测试
- 测试覆盖率要求：核心模块 ≥ 90%
- 使用 `#[cfg(test)]` 标注单元测试
- 集成测试放在 `tests/` 目录

### 4.4 Commit 规范检查

提交前运行以下检查：

```bash
# 1. 格式化检查
cargo fmt --check

# 2. Clippy 检查
cargo clippy --all-targets --all-features

# 3. 测试
cargo test --all-features

# 4. 文档测试
cargo test --doc
```

## 5. 分支保护规则

`dev`、`jdk8-lts`、`jdk17-lts`、`jdk21-lts` 分支受保护：

- 必须通过 CI/CD 检查
- 必须经过代码审查
- 不允许直接推送
- 必须更新相关文档

## 6. 版本控制

### 6.1 版本号格式

```
v{主版本}.{次版本}.{补丁版本}-jdk{lts版本}[-ecj|-rc|-beta]
```

### 6.2 版本生命周期

| 阶段 | 版本标识 | 说明 |
|------|----------|------|
| 开发版 | 无或 `-ecj` | 功能开发中 |
| Beta 版 | `-beta` | 特性冻结，测试中 |
| RC 版 | `-rc` | 候选发布 |
| 正式版 | 无 | 正式发布 |

## 7. 里程碑开发

FerrousJDK 采用拆分式分阶段开发模式，每个里程碑有明确的交付物和验收标准：

- **阶段 0**：项目初始化（2个里程碑）
- **阶段 1**：最小可用 JVM 核心（2个里程碑）
- **阶段 2**：完整 JVM 规范（3个里程碑）
- **阶段 3**：GC 实现与优化（3个里程碑）
- **阶段 4**：完整 Java 标准库（4个里程碑）
- **阶段 5**：JIT 编译器与自研编译器（4个里程碑）
- **阶段 6**：生产级发行版（3个里程碑）
- **阶段 7**：长期维护

详见 [docs/roadmap/README.md](./docs/roadmap/README.md)。

## 8. 模块贡献建议

### 8.1 新贡献者

适合入门的任务：

| 任务类型 | 难度 | 说明 |
|----------|------|------|
| 文档完善 | 简单 | 完善文档、修复拼写错误 |
| 标准库测试 | 简单 | 编写测试用例 |
| Bug 修复 | 中等 | 修复已知问题 |

### 8.2 高级贡献者

需要经验的任务：

| 任务类型 | 难度 | 需要知识 |
|----------|------|----------|
| JIT 优化 | 困难 | JVM 内部实现 |
| GC 实现 | 困难 | 垃圾回收算法 |
| 字节码验证 | 中等 | 字节码规范 |

## 9. 许可证

通过贡献代码，您同意将您的贡献按照 [MIT OR Apache-2.0](./LICENSE) 双协议许可。

## 10. 联系方式

- **GitHub Issues**: https://github.com/uv301/ferrous_jdk/issues
- **Discussions**: https://github.com/uv301/ferrous_jdk/discussions
- **Discord**: https://discord.gg/ferrous-jdk

## 11. 致谢

感谢所有为 FerrousJDK 做出贡献的开发者！

---

**最后更新**: 2026-04-02
