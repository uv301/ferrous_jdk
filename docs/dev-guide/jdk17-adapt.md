# JDK17 适配指南

本文档介绍 FerrousJDK 中 JDK17 相对于 JDK8 的主要变化、适配策略和特性开关使用方法。

## 1. JDK8 到 JDK17 概述

### 1.1 主要变化

| 类别 | 变化 |
|------|------|
| 模块系统 | 新增 JPMS (Java Platform Module System) |
| 语言特性 | Sealed Classes, Records, Pattern Matching |
| API 变更 | 多处 API 添加和改进 |
| 运行时 | 新的默认 GC (G1)，改进的 JIT |
| 安全 | 更强的安全默认设置 |

### 1.2 版本检测宏

```rust
// src/version.rs

#[cfg(feature = "jdk17")]
pub const VERSION: &str = "17";

#[cfg(feature = "jdk8")]
pub const VERSION: &str = "8";

pub fn is_jdk17_or_later() -> bool {
    cfg!(feature = "jdk17") || cfg!(feature = "jdk21")
}
```

## 2. 模块系统 (JPMS)

### 2.1 模块定义

```java
// module-info.java
module com.example.myapp {
    requires java.base;
    requires java.sql;
    
    exports com.example.api;
    exports com.example.internal to com.example.impl;
    
    opens com.example.internal to com.example.test;
    
    provides com.example.Service with com.example.impl.ServiceImpl;
    uses com.example.Service;
}
```

### 2.2 Rust 中的模块系统实现

```rust
// src/module_system/mod.rs

#[cfg(feature = "jdk17")]
pub mod jdk17 {
    use super::*;

    pub struct Module {
        name: ModuleName,
        requires: Vec<ModuleRef>,
        exports: Vec<Export>,
        opens: Vec<Open>,
        provides: Vec<Provide>,
        uses: Vec<ClassName>,
    }

    impl Module {
        pub fn parse(module_info: &[u8]) -> Result<Self> {
            // 解析 module-info.class
        }

        pub fn can_read(&self, other: &Module) -> bool {
            // 模块可读性检查
        }

        pub fn is_exported(&self, package: &PackageName) -> bool {
            // 包导出检查
        }

        pub fn is_opened(&self, package: &PackageName) -> bool {
            // 包开放检查 (反射)
        }
    }
}
```

### 2.3 模块路径处理

```rust
#[cfg(feature = "jdk17")]
pub fn resolve_modules(paths: &[PathBuf]) -> Result<ModuleResolver> {
    let mut resolver = ModuleResolver::new();
    
    for path in paths {
        for entry in walkdir(path) {
            if entry.file_name() == "module-info.class" {
                let module = Module::parse(&fs::read(entry.path())?)?;
                resolver.add_module(module);
            }
        }
    }
    
    Ok(resolver)
}
```

## 3. Sealed Classes

### 3.1 Sealed Class 定义

```java
public sealed class Shape permits Circle, Rectangle, Square {
    // ...
}

public final class Circle extends Shape {
    // Circle 是密封类的最终子类
}

public sealed class Rectangle extends Shape permits Square, RoundedRectangle {
    // Rectangle 密封 Rectangle 的子类
}

public non-sealed class Square extends Shape {
    // Square 是非密封的，可以有任意子类
}
```

### 3.2 Rust 实现

```rust
// src/class/sealed.rs

#[cfg(feature = "jdk17")]
#[derive(Debug, Clone)]
pub struct SealedInfo {
    pub permitted_subclasses: Vec<ClassRef>,
    pub is_sealed: bool,
    pub is_non_sealed: bool,
}

impl Class {
    pub fn get_permitted_subclasses(&self) -> &[ClassRef] {
        #[cfg(feature = "jdk17")]
        {
            &self.sealed_info.permitted_subclasses
        }
        
        #[cfg(not(feature = "jdk17"))]
        {
            &[]
        }
    }

    pub fn is_sealed(&self) -> bool {
        #[cfg(feature = "jdk17")]
        {
            self.sealed_info.is_sealed
        }
        
        #[cfg(not(feature = "jdk17"))]
        {
            false
        }
    }

    pub fn check_sealed_compatibility(&self, subclass: &Class) -> Result<()> {
        #[cfg(feature = "jdk17")]
        {
            if self.is_sealed() {
                let permitted = self.get_permitted_subclasses();
                if !permitted.iter().any(|c| c.name() == subclass.name()) {
                    return Err(IncompatibleClassChangeError::new(
                        format!("Class {} cannot extend sealed class {}",
                            subclass.name(), self.name())
                    ));
                }
            }
            Ok(())
        }
        
        #[cfg(not(feature = "jdk17"))]
        {
            Ok(())
        }
    }
}
```

## 4. Records

### 4.1 Record 定义

```java
public record Point(int x, int y) {
    // 自动生成:
    // - private final fields x, y
    // - public accessor methods x(), y()
    // - public equals(), hashCode(), toString()
    // - public constructor
    
    // 可以添加额外方法
    public double distanceFromOrigin() {
        return Math.sqrt(x * x + y * y);
    }
}
```

### 4.2 Record Rust 表示

```rust
// src/class/record.rs

#[cfg(feature = "jdk17")]
#[derive(Debug)]
pub struct RecordComponent {
    pub name: String,
    pub type_: Type,
    pub accessor_index: u16,
}

#[cfg(feature = "jdk17")]
pub struct RecordInfo {
    pub components: Vec<RecordComponent>,
}

#[cfg(feature = "jdk17")]
pub struct RecordClass {
    pub base: Class,
    pub record_info: RecordInfo,
}

impl RecordClass {
    pub fn components(&self) -> &[RecordComponent] {
        &self.record_info.components
    }

    pub fn generate_bytecode(&self) -> Vec<u8> {
        // 生成 record 相关字节码
    }
}
```

### 4.3 Record 字节码生成

```rust
#[cfg(feature = "jdk17")]
fn generate_record_accessors(record: &RecordClass) -> Vec<Method> {
    record.components().iter().map(|comp| {
        Method::new(
            comp.name.clone(),
            format!("(){}", comp.type_.descriptor()),
            AccessFlags::ACC_PUBLIC,
            move |gen| {
                // aload_0
                // getfield comp
                // areturn
            },
        )
    }).collect()
}
```

## 5. Pattern Matching

### 5.1 Type Pattern Matching

```java
// JDK16+ 类型模式
Object obj = "Hello";
if (obj instanceof String s) {
    // s 作用域在这里是 String
    System.out.println(s.length());
}

// JDK16+ instanceof 简化
if (obj instanceof String s && s.length() > 5) {
    System.out.println(s);
}
```

### 5.2 Switch Expression

```java
// JDK14+ switch 表达式
String result = switch (day) {
    case MONDAY, FRIDAY, SUNDAY -> "6";
    case TUESDAY -> "7";
    case THURSDAY, SATURDAY -> "8";
    case WEDNESDAY -> "9";
    default -> "0";
};
```

### 5.3 Rust 实现

```rust
#[cfg(feature = "jdk17")]
pub mod pattern_matching {
    use super::*;

    pub fn resolve_type_pattern(
        expr: &Expr,
        type_pattern: &Pattern,
    ) -> Result<MatchedBinding> {
        let target_type = type_pattern.target_type();
        let expr_type = expr.infer_type()?;
        
        if !expr_type.is_assignable_to(&target_type) {
            return Err(ClassCastException::new());
        }
        
        Ok(MatchedBinding {
            name: type_pattern.binding_name(),
            local_slot: allocate_local(target_type),
        })
    }
}
```

## 6. API 差异处理

### 6.1 新增 API

```rust
#[cfg(feature = "jdk17")]
pub mod stream_take_drop {
    use std::stream::Stream;

    pub fn <T> Stream<T>::takeWhile(predicate: Predicate<? super T>) -> Stream<T> {
        // JDK9+ 添加
    }

    pub fn <T> Stream<T>::dropWhile(predicate: Predicate<? super T>) -> Stream<T> {
        // JDK9+ 添加
    }
}
```

### 6.2 废弃 API

```rust
// 标记废弃
#[deprecated(since = "17", note = "Use ProcessHandle instead")]
pub fn exec(cmd: &[String]) -> Process {
    // JDK17 废弃，但仍可用
}
```

### 6.3 条件编译 API

```rust
// java.lang.System
#[cfg(feature = "jdk17")]
pub fn getProperty(key: &str) -> Option<&str> {
    // JDK17+ 实现
    get_property_impl(key)
}

#[cfg(not(feature = "jdk17"))]
pub fn getProperty(key: &str) -> Option<&str> {
    // JDK8 实现
    get_property_jdk8(key)
}
```

## 7. GC 变化

### 7.1 默认 GC 变化

| 版本 | 默认 GC |
|------|---------|
| JDK8 | Serial GC |
| JDK9-16 | G1 GC |
| JDK17+ | G1 GC |

### 7.2 G1 GC 配置

```bash
# JDK17 推荐 GC 配置
java -XX:+UseG1GC \
     -XX:MaxGCPauseMillis=200 \
     -XX:G1HeapRegionSize=4m \
     -XX:InitiatingHeapOccupancyPercent=45 \
     -jar myapp.jar
```

### 7.3 ZGC 支持 (JDK11+)

```bash
# JDK17 启用 ZGC
java -XX:+UseZGC \
     -XX:MaxGCPauseMillis=10 \
     -XX:+UseNUMA \
     -jar myapp.jar
```

## 8. 安全变化

### 8.1 默认禁用历史算法

```rust
#[cfg(feature = "jdk17")]
pub const DISABLED_ALGORITHMS: &[&str] = &[
    "TLSv1.0",
    "TLSv1.1",
    "3DES_EDE_CBC",
    "MD5",
];

#[cfg(not(feature = "jdk17"))]
pub const DISABLED_ALGORITHMS: &[&str] = &[];
```

### 8.2 Stronger Encapsulation

```bash
# JDK17 强封装默认启用
# --add-opens 可用于打开特定模块

# 打开 java.base/sun.misc 以供反射访问
java --add-opens java.base/sun.misc=ALL-UNNAMED -jar myapp.jar
```

## 9. 特性开关使用

### 9.1 配置示例

```toml
# Cargo.toml
[features]
default = ["jdk8"]

jdk8 = [
    "ferrous-core/jdk8",
    "ferrous-stdlib/jdk8",
]

jdk17 = [
    "ferrous-core/jdk17",
    "ferrous-stdlib/jdk17",
    "jdk8",  # 继承 jdk8
]

jdk21 = [
    "ferrous-core/jdk21",
    "ferrous-stdlib/jdk21",
    "jdk17",  # 继承 jdk17
]
```

### 9.2 条件编译

```rust
#[cfg(feature = "jdk17")]
mod jdk17_features {
    pub use super::*;
    
    // JDK17 特定实现
}

#[cfg(feature = "jdk8")]
mod jdk8_features {
    pub use super::*;
    
    // JDK8 兼容实现
}

#[cfg(all(not(feature = "jdk17"), not(feature = "jdk8")))]
compile_error!("Must enable one of jdk8 or jdk17 feature");
```

## 10. 测试 JDK17 特性

### 10.1 测试 Sealed Classes

```rust
#[cfg(test)]
mod sealed_tests {
    #[test]
    #[cfg(feature = "jdk17")]
    fn test_sealed_class() {
        let sealed_class = Class::parse(sealed_class_bytes());
        assert!(sealed_class.is_sealed());
        assert_eq!(sealed_class.permitted_count(), 3);
    }
}
```

### 10.2 测试 Records

```rust
#[cfg(test)]
mod record_tests {
    #[test]
    #[cfg(feature = "jdk17")]
    fn test_record_creation() {
        let point_class = Class::parse(record_point_bytes());
        assert!(point_class.is_record());
        assert_eq!(point_class.record_components().len(), 2);
    }
}
```

## 11. 迁移检查清单

从 JDK8 迁移到 JDK17：

- [ ] 检查模块依赖关系
- [ ] 更新 requires 和 exports
- [ ] 替换已废弃 API
- [ ] 调整安全策略
- [ ] 测试密封类层级
- [ ] 验证 Record 行为
- [ ] 更新 GC 配置
- [ ] 测试反射调用 (可能需要 --add-opens)
