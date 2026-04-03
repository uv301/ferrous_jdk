# 模块开发指南

本文档介绍 FerrousJDK 的 crate 结构、模块组织方式以及如何开发和贡献代码。

## 1. 项目结构概览

```
ferrous-jdk/
├── Cargo.toml              # Workspace 根配置
├── Cargo.lock              # 依赖锁定
├── rustfmt.toml           # 代码格式化配置
├── clippy.toml            # Lint 规则配置
│
├── crates/                  # 核心 crates
│   ├── ferrous-utils/      # 通用工具库
│   ├── ferrous-core/       # JVM 核心
│   ├── ferrous-interpreter/# 解释器
│   ├── ferrous-gc/         # 垃圾回收器
│   ├── ferrous-jit/        # JIT 编译器
│   ├── ferrous-jni/        # JNI 实现
│   ├── ferrous-jvmti/      # JVMTI 实现
│   ├── ferrous-stdlib/     # Java 标准库
│   ├── ferrous-javac/      # Java 编译器
│   ├── ferrous-tools/      # JDK 工具
│   └── ferrous-build/       # 构建系统
│
├── tests/                  # 测试
│   ├── unit/              # 单元测试
│   ├── integration/       # 集成测试
│   └── jtreg/            # jtreg 测试
│
└── docs/                  # 文档
```

## 2. Crate 依赖关系

```
ferrous-utils
     ↑
ferrous-core ←───────────┬────────────────────────┐
     ↑                  │                        │
     │          ferrous-jni                      ferrous-jvmti
     │                  │                        │
     ├──────────────────┼────────────────────────┤
     │                  │                        │
ferrous-interpreter    ferrous-stdlib     ferrous-tools
     ↑                  ↑
     │                  │
     └────────┬─────────┘
              │
         ferrous-jit
              │
              ↓
        ferrous-build
```

### 2.1 依赖层级规则

1. `ferrous-utils` - 无外部强依赖，被所有 crate 依赖
2. `ferrous-core` - 基础层，依赖 `ferrous-utils`
3. 其他 crate - 只依赖下层模块，禁止反向依赖

## 3. 创建新模块

### 3.1 添加新 crate

```bash
# 在 crates/ 目录下创建
cargo new crates/ferrous-new-module --lib
```

### 3.2 配置 Cargo.toml

```toml
# crates/ferrous-new-module/Cargo.toml
[package]
name = "ferrous-new-module"
version = "0.1.0"
edition = "2021"
authors = ["FerrousJDK Team"]
description = "Description of the new module"
license = "MIT OR Apache-2.0"
repository = "https://github.com/uv301/ferrous_jdk"
rust-version = "1.75"

[dependencies]
ferrous-utils = { path = "../ferrous-utils", version = "0.1" }

[features]
default = []
jdk8 = []
jdk17 = ["ferrous-core/jdk17"]
```

### 3.3 注册到 Workspace

更新根 `Cargo.toml`:

```toml
[workspace]
members = [
    "crates/ferrous-utils",
    "crates/ferrous-core",
    "crates/ferrous-new-module",
    # ... other members
]
resolver = "2"
```

## 4. 模块组织规范

### 4.1 目录结构

```
ferrous-example/
├── Cargo.toml
├── src/
│   ├── lib.rs           # 模块入口
│   ├── main.rs          # 可执行入口 (如果有)
│   ├── module1/
│   │   ├── mod.rs
│   │   ├── component.rs
│   │   └── component_test.rs
│   ├── module2/
│   │   ├── mod.rs
│   │   └── ...
│   └── utils/
│       ├── mod.rs
│       └── ...
└── tests/
    └── integration_test.rs
```

### 4.2 lib.rs 示例

```rust
//! Ferrous Example Module
//!
//! 详细描述模块功能和用途

#![cfg_attr(feature = "debug", feature(error_generic_member_access))]

pub mod module1;
pub mod module2;
pub mod utils;

pub use module1::{Component, ComponentBuilder};
pub use module2::Processor;
pub use utils::helper;

// 公开错误类型
mod error;
pub use error::{Error, Result};

// 内部子模块
mod internal;
```

## 5. 特性开关使用

### 5.1 定义特性

```rust
// 在 lib.rs 中使用 cfg 属性
#[cfg(feature = "jdk8")]
mod jdk8_impl;

#[cfg(feature = "jdk17")]
mod jdk17_impl;

#[cfg(feature = "jdk21")]
mod jdk21_impl;
```

### 5.2 条件编译函数

```rust
pub fn version_specific_function() -> &'static str {
    #[cfg(feature = "jdk21")]
    {
        "JDK21 implementation"
    }
    
    #[cfg(feature = "jdk17")]
    {
        "JDK17 implementation"
    }
    
    #[cfg(feature = "jdk8")]
    {
        "JDK8 implementation"
    }
    
    #[cfg(not(any(feature = "jdk8", feature = "jdk17", feature = "jdk21")))]
    {
        compile_error!("Must enable one of: jdk8, jdk17, jdk21")
    }
}
```

### 5.3 特性组合

```toml
# 基础特性
jdk8 = ["ferrous-core/jdk8"]

# 扩展特性
jdk17 = ["ferrous-core/jdk17", "ferrous-stdlib/jdk17"]
jdk21 = ["ferrous-core/jdk21", "ferrous-stdlib/jdk21", "jdk17"]  # 继承 jdk17
```

## 6. 测试规范

> **Rust 规范参考**：[The Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)

### 6.1 单元测试

```rust
// src/module.rs

/// Adds two integers together.
///
/// # Examples
///
/// ```
/// # use ferrous_example::add;
/// assert_eq!(add(1, 2), 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -2), -3);
    }

    #[test]
    #[should_panic]
    fn test_overflow() {
        // 测试溢出行为
    }
}
```

### 6.2 集成测试

```rust
// tests/integration_test.rs

use ferrous_example::*;

#[test]
fn test_full_workflow() {
    let processor = Processor::new();
    let result = processor.process(vec![1, 2, 3]);
    assert!(result.is_ok());
}

#[test]
fn test_error_handling() {
    let processor = Processor::new();
    let result = processor.process(vec![]);
    assert!(result.is_err());
}
```

### 6.3 特性测试

```rust
#[cfg(feature = "jdk17")]
mod jdk17_tests {
    use super::*;

    #[test]
    fn test_jdk17_feature() {
        // JDK17 特定测试
    }
}
```

## 7. 文档规范

> **Rust 规范参考**：
> - [Rust API Guidelines - Documentation](https://rust-lang.github.io/api-guidelines/documenting.html)
> - [rustdoc Book](https://doc.rust-lang.org/rustdoc/)

### 7.1 模块文档

```rust
//! Ferrous Example Module
//!
//! 详细描述模块功能、设计决策和使用方法。
//!
//! # Overview
//!
//! This module provides ...
//!
//! # Panics
//!
//! Functions may panic if ...
//!
//! # Examples
//!
//! ```
//! use ferrous_example::Processor;
//! ```

use std::collections::HashMap;

/// MyStruct 文档
///
/// 结构体的详细描述，包括其用途和行为。
///
/// # Examples
///
/// ```
/// use ferrous_example::MyStruct;
/// let my_struct = MyStruct::new();
/// ```
///
/// # Invariants
///
/// - `field` 必须始终为正数
pub struct MyStruct {
    field: i32,
}

/// Method Documentation
///
/// # Arguments
///
/// * `value` - The input value
///
/// # Returns
///
/// Returns the processed result
///
/// # Panics
///
/// Panics if the input is invalid
///
/// # Examples
///
/// ```
/// let result = my_function(42);
/// ```
pub fn my_function(value: i32) -> Result<i32, Error> {
    // implementation
}
```

### 7.2 运行文档测试

```bash
# 运行文档测试
cargo test --doc

# 查看文档
cargo doc --no-deps --open
```

## 8. 代码风格

### 8.1 格式化规则

参考 `rustfmt.toml`:

```toml
edition = "2021"
max_width = 100
tab_spaces = 4
newline_style = "Auto"
use_small_heuristics = "Default"
```

### 8.2 Clippy 规则

参考 `clippy.toml` 和代码注释:

```rust
// 在需要禁用 lint 的地方使用
#[allow(clippy::too_many_arguments)]
pub fn function_with_many_args(
    a: i32, b: i32, c: i32, d: i32, e: i32, f: i32,
) {
    // implementation
}
```

### 8.3 命名规范

> **Rust 规范参考**：[Rust Naming Conventions](https://rust-lang.github.io/api-guidelines/naming.html)

| 类型 | 规范 | 示例 | Rust 规范 |
|------|------|------|-----------|
| 模块 | snake_case | `byte_code`, `class_loader` | ✓ |
| 结构体 | PascalCase | `ClassFile`, `MethodInfo` | ✓ |
| 函数/方法 | snake_case | `load_class`, `parse_constant` | ✓ |
| 常量 | SCREAMING_SNAKE_CASE | `MAX_METHOD_SIZE`, `JAVA_MAGIC` | ✓ |
| 枚举变体 | PascalCase | `AccessFlag::Public` | ✓ |
| Trait | PascalCase | `GarbageCollector` | ✓ |
| 特性(feature) | snake_case | `jdk8`, `serial_gc` | ✓ |

> **注意**：Java 类型在 Rust 中保持 PascalCase 以符合 JVMS 命名约定。

## 9. 错误处理

### 9.1 错误类型定义

```rust
// 位于 src/error.rs

use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// I/O error
    Io(std::io::Error),
    /// Parse error with location
    Parse { message: String, line: usize, column: usize },
    /// Validation error
    Validation(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Parse { message, line, column } => {
                write!(f, "Parse error at {}:{}: {}", line, column, message)
            }
            Error::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

// 实现 From trait
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
```

### 9.2 使用错误类型

```rust
pub fn parse_class(data: &[u8]) -> Result<Class> {
    if data.len() < 10 {
        return Err(Error::Validation("Data too short".to_string()));
    }
    
    // 解析逻辑
    Ok(class)
}
```

## 10. Unsafe 代码规范

### 10.1 Unsafe 使用场景

FerrousJDK 中 unsafe 代码主要用于：

- JVM 内部指针操作
- 与原生代码互操作
- 性能关键的内存操作
- 对象布局控制

### 10.2 Unsafe 规范

```rust
/// Unsafe documentation block
///
/// # Safety
///
/// - Caller must ensure `ptr` is valid
/// - `ptr` must be properly aligned
/// - Memory must not be aliased
///
/// # Panics
///
/// Panics if safety invariants are violated (debug mode only)
pub unsafe fn raw_ptr_operation(ptr: *mut u8, len: usize) {
    // implementation
}

// 或者封装成安全接口
pub fn safe_operation(ptr: NonNull<u8>, len: usize) {
    // safety check
    assert!(!ptr.as_ptr().is_null());
    assert!(len > 0);
    
    // call unsafe function
    unsafe {
        raw_ptr_operation(ptr.as_ptr(), len);
    }
}
```

## 11. 提交代码

### 11.1 提交流程

```bash
# 1. 创建功能分支
git checkout -b feature/my-feature

# 2. 编写代码
# ... 编辑文件 ...

# 3. 格式化代码
cargo fmt

# 4. 运行检查
cargo clippy --features jdk17

# 5. 运行测试
cargo test --features jdk17

# 6. 提交
git add .
git commit -m "feat(module): add new feature"

# 7. 推送
git push origin feature/my-feature

# 8. 创建 Pull Request
gh pr create --title "feat(module): add new feature" --body "Description"
```

### 11.2 提交信息规范

```
<type>(<scope>): <subject>

<body>

<footer>
```

示例：

```
feat(interpreter): add invokespecial bytecode implementation

Implement invokespecial instruction with superclass constructor call support.

- Add invokespecial handler
- Implement super() call semantics
- Add bytecode verification tests

Closes #123
```

类型：`feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## 12. 代码审查清单

提交 PR 前检查：

- [ ] 代码通过 `cargo fmt`
- [ ] 代码通过 `cargo clippy`
- [ ] 所有测试通过
- [ ] 新功能有文档
- [ ] 公共 API 有注释
- [ ] 危险操作有 safety 文档
- [ ] 提交信息符合规范
