# FerrousJDK 整体架构设计

## 1. 项目定位

FerrousJDK 是一款基于 Rust 语言开发、**极致性能优先、100%兼容 Java SE 规范、构建产物与 OpenJDK 无缝对齐**的开源 JDK 发行版。

- **核心目标**：用 Rust 的内存安全、零成本抽象、高性能并发能力，打造一款比传统 C/C++ 实现的 OpenJDK 更安全、更快、更易维护的 JDK。
- **版本支持**：优先实现 JDK8 LTS，后续迭代支持 JDK17、JDK21 等主流 LTS 版本。
- **兼容承诺**：所有 API、命令、参数、行为与对应版本 OpenJDK 完全兼容，支持无缝替换。

> **规范遵循**：
> - Java SE 规范：遵循 [JLS (Java Language Specification)](https://docs.oracle.com/javase/specs/jls/se8/html/)
> - JVM 规范：遵循 [JVMS (Java Virtual Machine Specification) SE 8](https://docs.oracle.com/javase/specs/jvms/se8/html/)
> - Rust 规范：遵循 [Rust Reference](https://doc.rust-lang.org/reference/) 和 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## 2. 核心设计原则

### 2.1 模块化职责分离

采用 Rust Workspace 多 crate 架构，每个 crate 职责单一、边界清晰，支持并行开发、独立测试、按需编译。

### 2.2 性能优先

- 所有核心路径优先考虑性能，避免不必要的抽象与开销；
- native 方法 100% Rust 重写，彻底消除 JNI 桥接开销；
- 解释器、GC、JIT 均采用性能最优的工业级实现方案。

### 2.3 安全优先

- 最大化利用 Rust 所有权系统、类型系统在编译期保证内存安全、线程安全；
- 最小化 unsafe 代码使用，所有 unsafe 代码必须有详细注释与安全校验；
- 加密算法复用 Rust 生态经过安全审计的成熟库，不自行实现密码学原语。

### 2.4 兼容性优先

- 严格遵循 Java Virtual Machine Specification (JVMS) 与 Java Language Specification (JLS)；
- 所有 API、命令、参数、配置文件格式与 OpenJDK 完全对齐；
- 通过特性开关隔离不同 LTS 版本的差异，避免代码分支混乱。

### 2.5 可维护性优先

- 代码风格统一，注释完整，核心逻辑必须有设计说明；
- 测试覆盖率要求：核心模块≥90%，非核心模块≥80%；
- 文档与代码同步更新，所有设计决策必须有文档记录。

## 3. 整体架构分层

FerrousJDK 采用分层架构，从下到上分为 6 层，每层仅依赖下层，禁止反向依赖，保证架构清晰、依赖可控。

```
┌─────────────────────────────────────────────────────────────────┐
│                        构建发布层                                  │
│                   (ferrous-build)                                │
├─────────────────────────────────────────────────────────────────┤
│                        工具链层                                   │
│                   (ferrous-tools)                               │
├─────────────────────────────────────────────────────────────────┤
│                        标准库层                                   │
│                   (ferrous-stdlib)                               │
├─────────────────────────────────────────────────────────────────┤
│                        执行引擎层                                 │
│         (ferrous-interpreter / ferrous-jit / ferrous-gc)         │
├─────────────────────────────────────────────────────────────────┤
│                        JVM核心层                                  │
│    (ferrous-core / ferrous-jni / ferrous-jvmti / ferrous-utils) │
├─────────────────────────────────────────────────────────────────┤
│                      基础设施层                                   │
│                      (Rust Runtime)                              │
└─────────────────────────────────────────────────────────────────┘
```

## 4. 核心 Crate 完整定义

### 4.1 ferrous-utils - 全项目通用基础库

**定位**：无外部强依赖的基础工具库，被所有其他 crate 依赖。

**核心模块**：
| 模块 | 职责 |
|------|------|
| error | 统一错误类型、错误链、错误上下文 |
| collections | 高性能自定义集合（IntMap、ObjectPool 等） |
| memory | 内存管理工具、内存池、arena 分配器 |
| sync | 并发工具（SpinLock、RwLock、Atomic 等） |
| system | 跨平台系统 API 抽象 |
| log | 统一日志系统（tracing 接口封装） |
| version | 版本信息管理 |

### 4.2 ferrous-core - JVM 核心规范实现

**定位**：全 LTS 版本通用的 JVM 核心实现，不包含 GC、JIT、解释器。

**核心模块**：
| 模块 | 职责 |
|------|------|
| class_file | Class 文件全结构解析、字节码验证器、 ConstantPool |
| runtime | 运行时数据区（方法区/堆/栈/PC 寄存器）、JVM 线程模型 |
| class_loader | 全类加载器体系、双亲委派模型、类路径解析 |
| linking | 类验证、准备、解析、初始化全流程 |
| exception | 全异常类型、抛出/捕获机制、栈轨迹构建 |
| sync | 对象头结构、监视器锁、wait/notify、Unsafe 实现 |
| reflection | 全反射机制（getField/setField、invokeMethod 等） |
| invokedynamic | JDK7+ 动态调用支持、CallSite、MethodHandle |

### 4.3 ferrous-interpreter - 字节码模板解释器

**定位**：全 LTS 版本通用的解释器实现，采用模板解释器架构。

**核心特性**：
- 主解释器循环
- 全量 JDK 字节码指令实现（invokestatic、invokespecial、invokevirtual、invokeinterface 等）
- 栈顶缓存（Top-of-Stack Caching）优化
- 常量池缓存（Constant Pool Cache）优化
- 热点计数（Hotness Counter）统计
- OSR（On-Stack Replacement）栈上替换基础支持

### 4.4 ferrous-gc - 多策略垃圾回收器

**定位**：支持多种 GC 策略，通过特性开关选择。

**核心模块**：
| 模块 | 职责 |
|------|------|
| traits | GC 抽象 trait（Collector、Barrier、Policy） |
| common | 通用组件（Root 枚举、引用处理、finalizer、STW 机制） |
| serial | 串行 GC 实现（JDK8 默认） |
| parallel | 并行 GC 实现（Parallel Scavenge） |
| zgc | ZGC 低延迟 GC 实现 |
| shenandoah | Shenandoah GC 实现（可选） |

### 4.5 ferrous-jit - JIT 即时编译器 + AOT 工具

**定位**：热点代码即时编译优化，支持 Cranelift 和 LLVM 双后端。

**核心模块**：
| 模块 | 职责 |
|------|------|
| traits | JIT 抽象 trait |
| common | 通用组件（热点探测、分层编译阈值） |
| bytecode-to-ir | 字节码到中间表示的转换 |
| ir | IR 定义、CFG 构建 |
| optimizer | 全量优化 passes（标量替换、循环展开、逃逸分析等） |
| cranelift | Cranelift JIT 后端（Rust 原生，轻量级） |
| llvm | LLVM JIT 后端（深度优化，可选） |
| aot | AOT 提前编译工具 |

### 4.6 ferrous-jni - JNI 接口全量实现

**定位**：100% 兼容 JNI 规范，Rust 原生实现。

**核心特性**：
- 全 JNI 规范函数实现（FindClass、GetMethodID、CallMethod 等）
- native 方法注册与调用
- JVM 与 native 代码桥接
- 消除 JNI 桥接开销
- 与 OpenJDK 完全一致的 JNI 头文件

### 4.7 ferrous-jvmti - JVM Tool Interface 实现

**定位**：完整的 JVM TI 规范实现，支持调试、监控、分析工具。

**核心特性**：
- 全 JVM TI 规范函数实现
- 事件机制（Breakpoint、Exception、MethodEntry 等）
- Java Agent 支持（`-javaagent` 参数）
- 能力管理（CanGet*、CanSet*）
- 与 OpenJDK 完全一致的头文件

### 4.8 ferrous-stdlib - Java 标准库实现

**定位**：Java 标准库全量实现，native 方法 Rust 重写。

**核心包**：
| 包 | 描述 |
|----|------|
| java.lang | 语言基础（Object、String、Class、Thread 等） |
| java.util | 工具集（集合框架、日期时间、并发工具） |
| java.io | 输入输出流 |
| java.nio | 新 IO（ByteBuffer、Channel 等） |
| java.net | 网络编程 |
| java.security | 安全框架 |
| java.math | 大数运算 |
| jdk.* | JDK 专属包 |

**版本差异隔离**：通过特性开关 `jdk8`、`jdk17`、`jdk21` 隔离版本差异。

### 4.9 ferrous-javac - Rust 版 Java 编译器

**定位**：自研 Rust 版 Java 编译器（阶段 5 启用），前期封装 ECJ。

**核心阶段**：
1. 词法分析（Lexer）：源码 → Token 流
2. 语法分析（Parser）：Token 流 → AST
3. 语义分析（Sema）：类型检查、作用域解析
4. 字节码生成（Codegen）：AST → Class 文件

### 4.10 ferrous-tools - JDK 工具链

**定位**：JDK 全量工具实现，100% 兼容 OpenJDK 参数。

**核心工具**：
| 工具 | 描述 |
|------|------|
| java | JVM 启动器 |
| javac | Java 编译器（前期 ECJ 封装） |
| jar | JAR 包管理工具 |
| javap | Class 文件反汇编器 |
| jps | JVM 进程状态工具 |
| jstat | JVM 统计监控工具 |
| jmap | 内存映射分析工具 |
| jstack | 线程栈dump工具 |
| jdb | Java 调试器 |

### 4.11 ferrous-build - 构建系统

**定位**：1:1 生成 OpenJDK 对齐产物。

**核心功能**：
- 构建配置解析（来自外部配置或命令行参数）
- Rust 代码编译（调用 cargo）
- Java 代码编译
- 资源复制（动态库、配置文件、静态资源）
- 产物目录结构构建
- 文件格式校验（校验和、版本信息）
- 跨平台打包（tar.gz、zip）

## 5. 特性开关设计

通过 Cargo 特性开关隔离 LTS 版本差异和可选功能，支持灵活编译。

### 5.1 LTS 版本特性（必选其一）

| 特性 | 描述 | 默认 |
|------|------|------|
| `jdk8` | JDK8 LTS 版本适配 | ✓ |
| `jdk17` | JDK17 LTS 适配（含 JPMS、record 等） | |
| `jdk21` | JDK21 LTS 适配（含虚拟线程等） | |

### 5.2 GC 策略特性（可选）

| 特性 | 描述 | 默认 |
|------|------|------|
| `serial-gc` | 串行 GC 实现 | ✓ |
| `parallel-gc` | 并行 GC 实现 | |
| `zgc` | ZGC 低延迟 GC 实现 | |
| `shenandoah-gc` | Shenandoah GC 实现 | |

### 5.3 JIT 后端特性（可选）

| 特性 | 描述 | 默认 |
|------|------|------|
| `cranelift` | Cranelift JIT 后端 | ✓ |
| `llvm` | LLVM JIT 后端 | |

### 5.4 开发调试特性（可选）

| 特性 | 描述 |
|------|------|
| `debug` | 调试模式（调试日志、断言） |
| `trace` | 全链路追踪模式 |

## 6. 平台支持策略

| 层级 | 平台 | 支持级别 |
|------|------|----------|
| Tier 1 | x86_64 Linux, ARM64 Linux | 核心保障 |
| Tier 2 | x86_64 Windows, ARM64 macOS | 主流支持 |
| Tier 3 | 其他架构/OS | 尽力支持 |

## 7. 目录结构规范

构建产物 1:1 对齐对应版本 OpenJDK 目录结构：

```
<JAVA_HOME>/
├── bin/                    # 可执行文件（java、javac、jar 等）
├── lib/                    # 运行时库
│   ├── jvm.cfg             # JVM 配置
│   ├── server/            # Server JVM 动态库
│   └── ...
├── include/               # C 头文件（JNI 开发）
├── jre/                   # JRE 环境
├── conf/                  # 配置文件
├── release                # 版本信息
└── ...
```

## 8. 版本号规范

```
v{主版本}.{次版本}.{补丁版本}-jdk{lts版本}[-ecj|-rc|-beta]
```

示例：
- `v1.0.0-jdk8` - JDK8 正式版
- `v1.0.0-jdk17-ecj` - JDK17 ECJ 预览版
- `v2.0.0-jdk21-rc` - JDK21 候选发布版

## 9. 分支规范

| 分支类型 | 分支名 | 描述 |
|----------|--------|------|
| 长期受保护 | `dev` | 主开发集成分支 |
| 长期受保护 | `jdk8-lts` | JDK8 LTS 维护分支 |
| 长期受保护 | `jdk17-lts` | JDK17 LTS 维护分支 |
| 长期受保护 | `jdk21-lts` | JDK21 LTS 维护分支 |
| 临时 | `feature/*` | 功能开发分支 |
| 临时 | `bugfix/*` | Bug 修复分支 |
| 临时 | `release/*` | 发布准备分支 |
| 临时 | `hotfix/*` | 热修复分支 |

## 10. 核心技术选型

| 组件 | 选型 | 理由 |
|------|------|------|
| JIT IR | 自研 IR + Cranelift | 平衡开发效率与性能 |
| GC | 多策略并行 | 满足不同场景需求 |
| 解释器 | 模板解释器 | 实现简单、性能优秀 |
| 异步运行时 | tokio（可选） | 成熟稳定的异步生态 |
| 日志 | tracing | 结构化日志、Fluent API |
| 序列化 | serde | 成熟的序列化框架 |
