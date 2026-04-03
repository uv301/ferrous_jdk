# FerrousJDK 开发路线图

本目录包含 FerrousJDK 的开发计划和里程碑规划文档。

## 文档索引

| 文档 | 描述 |
|------|------|
| [README.md](./README.md) | 完整路线图，包含 21 个里程碑的详细规划 |

---

本文档记录 FerrousJDK 的详细开发计划和里程碑规划。

## 1. 项目概述

| 项目项 | 说明 |
|--------|------|
| 项目名称 | FerrousJDK |
| 开源协议 | MIT OR Apache-2.0 双协议 |
| 核心定位 | Rust 语言开发、极致性能优先、100% 兼容 JDK 规范 |
| 开发优先级 | JDK8 LTS 全量实现 → JDK17 → JDK21 → 后续 LTS |
| 编译器策略 | 阶段 0-4 复用 ECJ 封装 `javac`；阶段 5 启动自研 Rust 版 Java 编译器 |
| 总周期 | 58 周（7 个阶段，21 个里程碑） |

## 2. 版本与 Tag 规范

### 2.1 版本号格式

```
v{主版本}.{次版本}.{补丁版本}-jdk{lts版本}[-ecj|-rc|-beta]
```

### 2.2 版本标识说明

| 标识 | 说明 | 示例 |
|------|------|------|
| 无 | 正式版 | `v1.0.0-jdk8-lts` |
| `-ecj` | ECJ 编译器阶段 | `v0.4.0-jdk8-ecj-beta` |
| `-beta` | Beta 测试阶段 | `v0.4.3-jdk8-ecj-beta` |
| `-rc` | Release Candidate | `v0.5.3-jdk8-rc` |

### 2.3 Tag 规划

| 里程碑 | Tag | 说明 |
|---------|-----|------|
| 0-1 | `v0.0.1-init` | 仓库初始化完成 |
| 0-2 | `v0.0.2-infra-ready` | 基础设施就绪 |
| 1-1 | `v0.0.3-classfile-parser-ready` | ClassFile 解析器完成 |
| 1-2 | `v0.1.0-jdk8-ecj-mvp` | MVP 完成，可执行 HelloWorld |
| 2-1 | `v0.2.0-jdk8-ecj-bytecode-complete` | 全字节码指令完成 |
| 2-2 | `v0.2.1-jdk8-ecj-thread-lock-complete` | 线程锁机制完成 |
| 2-3 | `v0.2.2-jdk8-ecj-core-complete` | java.lang 核心完成 |
| 3-1 | `v0.3.0-jdk8-ecj-memory-management-ready` | 内存管理就绪 |
| 3-2 | `v0.3.1-jdk8-ecj-gc-core-complete` | GC 核心完成 |
| 3-3 | `v0.3.2-jdk8-ecj-gc-complete` | GC 全功能完成 |
| 4-1 | `v0.4.0-jdk8-ecj-collection-complete` | 集合框架完成 |
| 4-2 | `v0.4.1-jdk8-ecj-io-nio-complete` | IO/NIO 完成 |
| 4-3 | `v0.4.2-jdk8-ecj-net-security-complete` | 网络安全完成 |
| 4-4 | `v0.4.3-jdk8-ecj-beta` | JDK8 Beta 发布 |
| 5-1 | `v0.5.0-jdk8-ecj-jit-baseline` | JIT 基础完成 |
| 5-2 | `v0.5.1-jdk8-ecj-jit-zgc-complete` | JIT ZGC 完成 |
| 5-3 | `v0.5.2-jdk8-javac-complete` | 自研编译器完成（移除 -ecj） |
| 5-4 | `v0.5.3-jdk8-rc` | Release Candidate |
| 6-1 | `v1.0.0-jdk8-lts` | **JDK8 LTS 正式发布** |

## 3. 分阶段里程碑规划

---

## 阶段 0：项目初始化与前置准备

**总周期**：2 周

**当前进度**：✅ 里程碑 0-1 已完成，里程碑 0-2 未开始

### 里程碑 0-1：仓库初始化与规范制定

**周期**：1 周

    **实际进度**：✅ 已完成

| 项目 | 内容 |
|------|------|
| **前置依赖** | 无 |
| **核心目标** | 完成项目基础框架搭建，明确所有开发规范 |

**核心开发子任务**：
1. 创建 GitHub/GitLab 仓库，初始化 `dev`、`jdk8-lts` 核心分支
2. 编写 README.md、CONTRIBUTING.md、CODE_OF_CONDUCT.md
3. 配置 MIT/Apache-2.0 双协议声明
4. 搭建 Rust Workspace 根结构
5. 配置 Rustfmt、Clippy 规则

**配套学习内容**：
1. Rust Workspace 官方规范、Rust API Guidelines
2. 语义化版本 2.0.0 规范、Conventional Commits 提交规范
3. OpenJDK8 整体架构与源码目录结构概览

**交付物**：
- ✅ Git 仓库初始化完成
- ✅ 项目规范文档集
- ✅ 空 Workspace 框架（可通过 `cargo build`）

**验收标准**：
1. 仓库可正常访问，分支保护规则生效
2. 空项目无编译错误、无 Clippy 警告
3. 所有规范文档编写完成

**当前状态**：✅ 已完成
- ✅ Git 仓库初始化完成
- ✅ Workspace 结构创建完成（11 个 crate）
- ✅ 核心 crate 目录结构创建完成
- ✅ 规范文档编写完成（README、CONTRIBUTING、CODE_OF_CONDUCT）
- ✅ Rustfmt、Clippy 配置完成
- ✅ 构建和检查通过

---

### 里程碑 0-2：基础设施与 ECJ 集成

**周期**：1 周

**实际进度**：📋 待开始

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 0-1 验收通过 |
| **核心目标** | 完成 CI/CD 流水线搭建，集成 ECJ 封装 `javac` |

**核心开发子任务**：
1. 配置 GitHub Actions CI/CD 流水线
2. 下载 ECJ 最新稳定版，封装为 `javac` 命令
3. 编写 ECJ 封装的兼容性测试
4. 搭建单元测试框架、集成测试框架
5. 编写项目 Roadmap 文档

**配套学习内容**：
1. GitHub Actions CI/CD 配置规范
2. ECJ 官方文档、javac 命令参数规范
3. JVMS 第 1 章、第 2 章 JVM 整体结构概览

**交付物**：
- ✅ 可用的 CI/CD 流水线
- ✅ 封装的 `javac` 命令
- ✅ 可用的测试框架
- ✅ 完整的项目 Roadmap 文档

**验收标准**：
1. CI/CD 流水线正常运行
2. `javac` 命令可正常编译 `HelloWorld.java`
3. 测试框架可正常运行

**版本控制**：
- 开发分支：`feature/ci-ecj-infra`
- Tag：`v0.0.2-infra-ready`

---

## 阶段 1：最小可用 JVM 核心（MVP）

**总周期**：4 周

**当前进度**：📋 未开始

### 里程碑 1-1：ClassFile 解析器与运行时数据区

**周期**：2 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 阶段 0 所有里程碑验收通过 |
| **核心目标** | 100% 实现 JDK8 ClassFile 规范解析 |

**核心开发子任务**：
1. 实现 ClassFile 魔数、版本号、常量池的完整解析
2. 实现访问标志、字段表、方法表、属性表的完整解析
3. 编写 ClassFile 解析器的全量单元测试
4. 实现最小化运行时数据区：方法区、堆、Java 栈等
5. 实现栈帧结构

**配套学习内容**：
1. JVMS 第 4 章《The class File Format》全文
2. JVMS 第 2 章 2.5 节《Run-Time Data Areas》
3. Rust 二进制解析库 `nom` 官方文档

**交付物**：
- ✅ 完整的 ClassFile 解析器
- ✅ 最小化运行时数据区实现
- ✅ 全量单元测试用例集

**验收标准**：
1. 可正确解析所有合法的 JDK8 ClassFile
2. 单元测试覆盖率 ≥ 90%
3. 运行时数据区可正常初始化、销毁

**版本控制**：
- 开发分支：`feature/classfile-parser-runtime-area`
- Tag：`v0.0.3-classfile-parser-ready`

---

### 里程碑 1-2：类加载器、字节码解释器与 HelloWorld

**周期**：2 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 1-1 验收通过 |
| **核心目标** | 可正常加载并执行 HelloWorld.class |

**核心开发子任务**：
1. 实现启动类加载器，完成类的加载、验证、准备、解析、初始化
2. 实现基础字节码解释器（覆盖约 60% 常用指令）
3. 实现基础异常处理机制
4. 实现最小化 `java` 启动器
5. 编写 HelloWorld 集成测试

**配套学习内容**：
1. JVMS 第 5 章《Loading, Linking, and Initializing》全文
2. JVMS 第 6 章《The Java Virtual Machine Instruction Set》
3. OpenJDK `ClassLoader`、`TemplateInterpreter` 源码概览

**交付物**：
- ✅ 最小化启动类加载器
- ✅ 覆盖核心指令的字节码解释器
- ✅ 可用的 `java` 启动器
- ✅ 可正常执行的 HelloWorld 集成测试

**验收标准**：
1. 可正常加载 HelloWorld.class 并输出结果
2. 类加载流程完全符合 JVMS 规范
3. 已实现的字节码指令单元测试覆盖率 100%

**版本控制**：
- 开发分支：`feature/classloader-interpreter-hello-world`
- Tag：`v0.1.0-jdk8-ecj-mvp`（MVP 完成）

---

## 阶段 2：完整 JVM 规范实现

**总周期**：8 周

**当前进度**：📋 未开始

### 里程碑 2-1：全字节码指令与完整类加载器体系

**周期**：2 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 阶段 1 所有里程碑验收通过 |
| **核心目标** | 100% 覆盖 JDK8 所有字节码指令 |

**核心开发子任务**：
1. 补全字节码解释器，覆盖 JDK8 全部 200+ 字节码指令
2. 实现扩展类加载器、应用程序类加载器
3. 实现完整的双亲委派模型
4. 实现自定义类加载器支持
5. 实现类的完整验证流程

**交付物**：
- ✅ 100% 覆盖 JDK8 字节码指令的解释器
- ✅ 完整的类加载器体系
- ✅ 全量字节码指令单元测试

**验收标准**：
1. 所有字节码指令单元测试 100% 覆盖
2. 双亲委派模型正常工作
3. 类验证流程可正确拦截非法 ClassFile

**版本控制**：
- 开发分支：`feature/full-bytecode-classloader-system`
- Tag：`v0.2.0-jdk8-ecj-bytecode-complete`

---

### 里程碑 2-2：线程模型、锁机制与异常处理

**周期**：3 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 2-1 验收通过 |
| **核心目标** | 实现完整的 JVM 线程模型与 synchronized 锁语义 |

**核心开发子任务**：
1. 实现 JVM 线程模型（创建、启动、休眠、终止、中断）
2. 实现对象头结构（无锁、偏向锁、轻量级锁、重量级锁）
3. 实现 synchronized 关键字语义
4. 实现锁池、等待池（wait/notify/notifyAll）
5. 实现完整的异常处理机制

**交付物**：
- ✅ 完整的 JVM 线程实现
- ✅ synchronized 锁机制
- ✅ wait/notify 机制
- ✅ 多线程并发测试用例集

**验收标准**：
1. 多线程程序可正常创建、启动、终止
2. 锁语义正确，无数据竞争
3. 所有异常类型可正确抛出、捕获

**版本控制**：
- 开发分支：`feature/thread-lock-exception-system`
- Tag：`v0.2.1-jdk8-ecj-thread-lock-complete`

---

### 里程碑 2-3：java.lang 核心类与反射机制

**周期**：3 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 2-2 验收通过 |
| **核心目标** | 完成 java.lang 包所有核心类实现 |

**核心开发子任务**：
1. 实现 java.lang.Object 类（含所有 native 方法）
2. 实现 java.lang.Class 类
3. 实现 java.lang.String 类
4. 实现 java.lang.System、Thread、Throwable、ClassLoader 等核心类
5. 实现基础反射机制

**交付物**：
- ✅ java.lang 包所有核心类完整实现
- ✅ 对应 native 方法的 Rust 重写
- ✅ 基础反射机制
- ✅ 核心类单元测试用例集

**验收标准**：
1. java.lang 核心类所有方法行为与 OpenJDK 完全一致
2. 反射 API 可正常使用
3. native 方法重写完成率 100%
4. 单元测试覆盖率 ≥ 90%

**版本控制**：
- 开发分支：`feature/java-lang-core-reflection`
- Tag：`v0.2.2-jdk8-ecj-core-complete`

---

## 阶段 3：GC 实现与性能优化

**总周期**：8 周

**当前进度**：📋 未开始

### 里程碑 3-1：内存管理与可达性分析

**周期**：2 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 阶段 2 所有里程碑验收通过 |
| **核心目标** | 实现堆内存精细化管理，完成可达性分析算法 |

**核心开发子任务**：
1. 实现堆内存分代划分（Eden、Survivor0、Survivor1、老年代）
2. 实现对象内存分配器（支持 TLAB）
3. 实现根节点枚举算法
4. 实现可达性分析算法
5. 实现 OOM 检测机制

**交付物**：
- ✅ 分代堆内存管理器
- ✅ TLAB 对象分配器
- ✅ 根节点枚举与可达性分析算法
- ✅ OOM 检测机制

**验收标准**：
1. 对象可正常分配、释放，无内存泄漏
2. 可达性分析准确标记存活对象
3. 单元测试覆盖率 ≥ 90%

**版本控制**：
- 开发分支：`feature/memory-management-reachability-analysis`
- Tag：`v0.3.0-jdk8-ecj-memory-management-ready`

---

### 里程碑 3-2：分代垃圾回收器核心实现

**周期**：3 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 3-1 验收通过 |
| **核心目标** | 实现完整分代 GC，完成年轻代复制、老年代标记-整理 |

**核心开发子任务**：
1. 实现 STW 机制
2. 实现年轻代 Minor GC（复制算法）
3. 实现老年代 Full GC（标记-清除-整理）
4. 实现 finalize() 方法执行逻辑
5. 实现 GC 日志机制

**交付物**：
- ✅ 完整的 STW 机制
- ✅ 年轻代 Minor GC
- ✅ 老年代 Full GC
- ✅ GC 日志机制

**验收标准**：
1. Minor GC 可正常回收垃圾对象
2. 晋升逻辑正确
3. Full GC 可正常回收并整理内存
4. STW 机制正确暂停、恢复线程

**版本控制**：
- 开发分支：`feature/generational-gc-core`
- Tag：`v0.3.1-jdk8-ecj-gc-core-complete`

---

### 里程碑 3-3：引用类型、解释器优化与监控工具

**周期**：3 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 3-2 验收通过 |
| **核心目标** | 实现完整引用类型体系，优化解释器性能 |

**核心开发子任务**：
1. 实现软引用、弱引用、虚引用及 ReferenceQueue
2. 优化字节码解释器（栈顶缓存、常量池缓存等）
3. 实现 jps 工具
4. 实现 jstat 工具
5. 实现 JVM 调优参数

**交付物**：
- ✅ 完整引用类型与 ReferenceQueue
- ✅ 优化后的字节码解释器
- ✅ jps、jstat 工具
- ✅ JVM 调优参数

**验收标准**：
1. 软引用、弱引用、虚引用正常工作
2. 解释器性能达到 OpenJDK 解释器的 30% 以上
3. jps、jstat 工具功能与 OpenJDK 完全一致

**版本控制**：
- 开发分支：`feature/reference-type-interpreter-opt-monitor-tools`
- Tag：`v0.3.2-jdk8-ecj-gc-complete`

---

## 阶段 4：完整 Java 标准库实现

**总周期**：12 周

**当前进度**：📋 未开始

### 里程碑 4-1：java.util 核心集合框架

**周期**：3 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 阶段 3 所有里程碑验收通过 |
| **核心目标** | 实现 java.util 包核心集合框架 |

**核心开发子任务**：
1. 实现 List 体系（ArrayList、LinkedList、Vector、Stack）
2. 实现 Set 体系（HashSet、LinkedHashSet、TreeSet）
3. 实现 Map 体系（HashMap、LinkedHashMap、TreeMap、Hashtable）
4. 实现迭代器、Collections、Arrays 工具类
5. 实现 JDK8 新增的 Stream API 基础框架

**交付物**：
- ✅ java.util 包核心集合框架
- ✅ Stream API 基础框架
- ✅ 全量单元测试

**验收标准**：
1. 所有集合类行为与 OpenJDK 完全一致
2. 单元测试覆盖率 ≥ 90%

**版本控制**：
- 开发分支：`feature/java-util-collection-framework`
- Tag：`v0.4.0-jdk8-ecj-collection-complete`

---

### 里程碑 4-2：java.io/java.nio 文件与 IO 体系

**周期**：3 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 4-1 验收通过 |
| **核心目标** | 实现 Java IO/NIO 体系 |

**核心开发子任务**：
1. 实现 java.io 字节流体系
2. 实现 java.io 字符流体系
3. 实现 File、RandomAccessFile
4. 实现 java.nio 缓冲区体系
5. 实现 java.nio.channels 通道体系

**交付物**：
- ✅ java.io 流体系
- ✅ java.nio 缓冲区与通道体系
- ✅ 全量单元测试

**验收标准**：
1. 所有 IO/NIO 类行为与 OpenJDK 完全一致
2. 单元测试覆盖率 ≥ 85%

**版本控制**：
- 开发分支：`feature/java-io-nio-system`
- Tag：`v0.4.1-jdk8-ecj-io-nio-complete`

---

### 里程碑 4-3：java.net/java.security 网络与安全

**周期**：3 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 4-2 验收通过 |
| **核心目标** | 实现 Java 网络编程与基础安全体系 |

**核心开发子任务**：
1. 实现 java.net 核心类（Socket、ServerSocket、URL 等）
2. 实现 HTTP/HTTPS 基础支持
3. 实现 java.security 基础安全体系
4. 实现 Java 加密扩展（JCE）基础框架
5. 实现安全管理器基础框架

**交付物**：
- ✅ java.net 网络编程体系
- ✅ java.security 基础安全体系
- ✅ 常用加密算法
- ✅ 全量单元测试

**验收标准**：
1. 所有网络类行为与 OpenJDK 完全一致
2. 单元测试覆盖率 ≥ 85%

**版本控制**：
- 开发分支：`feature/java-net-security-system`
- Tag：`v0.4.2-jdk8-ecj-net-security-complete`

---

### 里程碑 4-4：JDK8 专属特性、工具链与构建产物对齐

**周期**：3 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 4-3 验收通过 |
| **核心目标** | 完成 JDK8 专属特性，工具链对齐，达到 Beta 状态 |

**核心开发子任务**：
1. 完成 JDK8 专属特性（Stream API、Lambda、Date Time API 等）
2. 实现完整 JNI 接口
3. 实现 jar、javap 工具
4. 完善 ferrous-build 构建系统
5. 集成 jtreg 测试套件

**交付物**：
- ✅ JDK8 专属特性完整实现
- ✅ jar、javap 工具
- ✅ 1:1 对齐 OpenJDK8 的构建产物
- ✅ jtreg 测试套件集成

**验收标准**：
1. JDK8 所有专属特性可正常使用
2. jar、javap 工具功能与 OpenJDK 完全一致
3. jtreg 核心测试用例通过率 ≥ 70%
4. JDK8 核心 API 覆盖率 ≥ 90%

**版本控制**：
- 开发分支：`feature/jdk8-features-toolchain-build-align`
- Tag：`v0.4.3-jdk8-ecj-beta`

---

## 阶段 5：JIT 编译器与自研编译器

**总周期**：16 周

**当前进度**：📋 未开始

### 里程碑 5-1：JIT 即时编译器基础

**周期**：4 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 阶段 4 所有里程碑验收通过 |
| **核心目标** | 实现 JIT 即时编译器基础框架 |

**核心开发子任务**：
1. 确定 JIT 后端方案（Cranelift 优先）
2. 实现热点方法探测机制
3. 实现字节码 → IR 转换
4. 实现基础优化 passes
5. 实现 IR → 机器码生成

**交付物**：
- ✅ JIT 编译器基础框架
- ✅ 热点方法探测机制
- ✅ 字节码 → IR → 机器码完整流程

**验收标准**：
1. 可正常探测热点方法，触发 JIT 编译
2. 编译后的机器码可正常执行
3. 单元测试覆盖率 ≥ 80%

**版本控制**：
- 开发分支：`feature/jit-compiler-baseline`
- Tag：`v0.5.0-jdk8-ecj-jit-baseline`

---

### 里程碑 5-2：JIT 高级优化与 ZGC

**周期**：4 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 5-1 验收通过 |
| **核心目标** | 实现 JIT 高级优化与 ZGC 低延迟垃圾回收 |

**核心开发子任务**：
1. 实现 JIT 高级优化（方法内联、逃逸分析、锁消除等）
2. 实现分层编译机制
3. 实现 OSR（栈上替换）
4. 实现 ZGC 低延迟垃圾回收器
5. 优化 ZGC 的 STW 时间

**交付物**：
- ✅ JIT 高级优化 passes
- ✅ 分层编译与 OSR
- ✅ ZGC 低延迟垃圾回收器

**验收标准**：
1. JIT 编译器峰值性能达到 OpenJDK C2 的 60% 以上
2. ZGC 平均 STW 时间 ≤ 1ms
3. 单元测试覆盖率 ≥ 85%

**版本控制**：
- 开发分支：`feature/jit-advanced-opt-zgc`
- Tag：`v0.5.1-jdk8-ecj-jit-zgc-complete`

---

### 里程碑 5-3：自研 Rust 版 Java 编译器

**周期**：4 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 5-2 验收通过 |
| **核心目标** | 实现自研 Rust 版 Java 编译器，替换 ECJ |

**核心开发子任务**：
1. 实现 Java 词法分析器
2. 实现 Java 语法分析器
3. 实现语义分析器
4. 实现字节码生成器
5. 替换 ECJ，完成 `javac` 命令自研实现

**交付物**：
- ✅ 自研 Java 编译器（位于 `crates/ferrous-javac`）
- ✅ 兼容 javac 所有核心参数的 `javac` 命令
- ✅ 移除所有 ECJ 相关依赖

**验收标准**：
1. 可正常编译所有合法的 Java 8 代码
2. 编译结果与 javac 完全一致
3. 编译器单元测试覆盖率 ≥ 90%
4. **移除所有 tag 的 `-ecj` 标识**

**版本控制**：
- 开发分支：`feature/self-developed-java-compiler`
- Tag：`v0.5.2-jdk8-javac-complete`（移除 -ecj）

---

### 里程碑 5-4：JVM TI、高级特性与 JDK17 适配启动

**周期**：4 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 5-3 验收通过 |
| **核心目标** | 实现 JVM TI 接口，启动 JDK17 适配 |

**核心开发子任务**：
1. 实现 JVM Tool Interface (JVM TI) 完整接口
2. 实现 Java Agent 支持
3. 实现 invokedynamic 指令完整实现
4. 完善 JDK 工具链（jmap、jstack、jconsole、jdb）
5. 启动 JDK17 LTS 适配

**交付物**：
- ✅ JVM TI 接口完整实现
- ✅ Java Agent 支持
- ✅ 完整 JDK 工具链
- ✅ `jdk17-lts` 分支基础框架

**验收标准**：
1. JVM TI 接口可正常工作
2. Java Agent 可正常加载运行
3. JDK17 核心特性差异梳理完成

**版本控制**：
- 开发分支：`feature/jvmti-advanced-features-jdk17-start`
- Tag：`v0.5.3-jdk8-rc`

---

## 阶段 6：生产级发行版与全量兼容

**总周期**：8 周

**当前进度**：📋 未开始

### 里程碑 6-1：跨平台适配与安全机制

**周期**：2 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 阶段 5 所有里程碑验收通过 |
| **核心目标** | 完成 Tier1、Tier2 平台全量适配 |

**核心开发子任务**：
1. 完成 Tier1 平台全量适配（x86_64 Linux、ARM64 Linux）
2. 完成 Tier2 平台全量适配（x86_64 Windows、ARM64 macOS）
3. 实现完整的 Java 安全管理器
4. 完善加密算法体系
5. 实现代码签名、证书验证机制

**交付物**：
- ✅ Tier1、Tier2 平台全量适配
- ✅ 完整的 Java 安全体系
- ✅ 跨平台 CI/CD 流水线

**验收标准**：
1. 所有平台构建产物可正常运行
2. 安全机制正常工作
3. 跨平台 CI/CD 流水线正常运行

---

### 里程碑 6-2：全量兼容性测试与稳定性优化

**周期**：3 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 6-1 验收通过 |
| **核心目标** | 完成全量兼容性测试，达到生产级可用标准 |

**核心开发子任务**：
1. 执行 OpenJDK jtreg 全量测试套件
2. 执行主流框架兼容性测试（Spring Boot、MyBatis 等）
3. 执行主流 JVM 语言兼容性测试（Kotlin、Scala、Groovy）
4. 执行 7*24 小时稳定性压测
5. 优化启动速度、内存占用、峰值性能

**交付物**：
- ✅ 全量兼容性测试报告
- ✅ 性能优化完成
- ✅ 稳定性压测报告

**验收标准**：
1. jtreg 测试套件核心用例通过率 ≥ 95%
2. 所有主流 Java 框架可正常启动运行
3. 7*24 小时压测无崩溃、无内存泄漏

---

### 里程碑 6-3：正式版发布与文档完善

**周期**：3 周

| 项目 | 内容 |
|------|------|
| **前置依赖** | 里程碑 6-2 验收通过 |
| **核心目标** | 发布 FerrousJDK 1.0.0 JDK8 LTS 正式版 |

**核心开发子任务**：
1. 编写完整的用户文档和开发文档
2. 编写发行说明
3. 完成所有平台的正式版构建、签名、打包
4. 发布正式版到 GitHub Release

**交付物**：
- ✅ 完整的用户文档和开发文档
- ✅ 所有支持平台的正式版安装包
- ✅ **FerrousJDK 1.0.0 JDK8 LTS 正式版发布**

**验收标准**：
1. 文档完整、清晰、无歧义
2. 所有平台正式版安装包可正常安装运行
3. 正式版发布完成

**版本控制**：
- Tag：`v1.0.0-jdk8-lts`（正式发布）
- 同步创建 `jdk17-lts` 分支

---

## 阶段 7：长期维护与生态扩展

### 季度里程碑：LTS 版本维护

**核心任务**：
- 维护已发布的 LTS 版本
- 每月发布安全补丁
- 每季度发布功能更新补丁

**验收标准**：
- 无未修复的高危安全漏洞
- 用户反馈的核心 bug 及时修复

### 年度里程碑：新 LTS 版本适配

**核心任务**：
- 跟进 Oracle 发布的新 JDK LTS 版本
- 完成新特性适配
- 发布对应正式版

**验收标准**：
- 新 LTS 版本核心 API 覆盖率 ≥ 90%
- 兼容性测试通过率 ≥ 95%

### 持续迭代里程碑：性能优化与生态扩展

**核心任务**：
- 持续优化 JIT 与 GC 性能
- 适配云原生、容器化场景
- 构建开源社区

---

## 4. 里程碑进度总览

| 阶段 | 里程碑 | Tag | 周期 | 状态 |
|------|--------|-----|------|------|
| **0** | 0-1 | `v0.0.1-init` | 1 周 | ✅ 完成 |
| | 0-2 | `v0.0.2-infra-ready` | 1 周 | 📋 |
| **1** | 1-1 | `v0.0.3-classfile-parser-ready` | 2 周 | 📋 |
| | 1-2 | `v0.1.0-jdk8-ecj-mvp` | 2 周 | 📋 |
| **2** | 2-1 | `v0.2.0-jdk8-ecj-bytecode-complete` | 2 周 | 📋 |
| | 2-2 | `v0.2.1-jdk8-ecj-thread-lock-complete` | 3 周 | 📋 |
| | 2-3 | `v0.2.2-jdk8-ecj-core-complete` | 3 周 | 📋 |
| **3** | 3-1 | `v0.3.0-jdk8-ecj-memory-management-ready` | 2 周 | 📋 |
| | 3-2 | `v0.3.1-jdk8-ecj-gc-core-complete` | 3 周 | 📋 |
| | 3-3 | `v0.3.2-jdk8-ecj-gc-complete` | 3 周 | 📋 |
| **4** | 4-1 | `v0.4.0-jdk8-ecj-collection-complete` | 3 周 | 📋 |
| | 4-2 | `v0.4.1-jdk8-ecj-io-nio-complete` | 3 周 | 📋 |
| | 4-3 | `v0.4.2-jdk8-ecj-net-security-complete` | 3 周 | 📋 |
| | 4-4 | `v0.4.3-jdk8-ecj-beta` | 3 周 | 📋 |
| **5** | 5-1 | `v0.5.0-jdk8-ecj-jit-baseline` | 4 周 | 📋 |
| | 5-2 | `v0.5.1-jdk8-ecj-jit-zgc-complete` | 4 周 | 📋 |
| | 5-3 | `v0.5.2-jdk8-javac-complete` | 4 周 | 📋 |
| | 5-4 | `v0.5.3-jdk8-rc` | 4 周 | 📋 |
| **6** | 6-1 | - | 2 周 | 📋 |
| | 6-2 | - | 3 周 | 📋 |
| | 6-3 | `v1.0.0-jdk8-lts` | 3 周 | 📋 |

**图例**：
- 📋 待开始
- 🔄 进行中
- ✅ 已完成

---

## 5. Crate 实现状态

### 5.1 当前实现状态（截至 2026-04-02）

| Crate | 目录结构 | 实现代码 | 说明 |
|-------|----------|----------|------|
| ferrous_utils | ✅ 完成 | 🔄 占位符 | 基础模块创建，代码待实现 |
| ferrous_core | ✅ 完成 | 🔄 占位符 | 基础模块创建，代码待实现 |
| ferrous_interpreter | ✅ 完成 | 🔄 占位符 | 基础模块创建，代码待实现 |
| ferrous_gc | ✅ 完成 | 🔄 占位符 | 完整目录结构，代码待实现 |
| ferrous_stdlib | ✅ 完成 | 🔄 占位符 | 仅占位符代码 |
| ferrous_tools | ✅ 完成 | 🔄 占位符 | 仅占位符代码 |
| ferrous_javac | ✅ 完成 | 🔄 占位符 | 仅占位符代码 |
| ferrous_jit | ✅ 完成 | 🔄 占位符 | 仅占位符代码 |
| ferrous_jni | ✅ 完成 | 🔄 占位符 | 仅占位符代码 |
| ferrous_jvmti | ✅ 完成 | 🔄 占位符 | 仅占位符代码 |
| ferrous_build | ✅ 完成 | 🔄 占位符 | 仅占位符代码 |

### 5.2 下一步实现计划

**优先级 1（P0）**：
1. 实现 ferrous_utils 基础模块（error、collections、memory、concurrency）
2. 实现 ferrous_core 的 ClassFile 解析器
3. 实现 ferrous_core 的运行时数据区

**优先级 2（P1）**：
1. 实现 ferrous_interpreter 的字节码解释器
2. 实现 ferrous_core 的类加载器
3. 实现 ferrous_stdlib 的 java.lang 核心类

**优先级 3（P2）**：
1. 实现 ferrous_gc 的 Serial GC
2. 实现 ferrous_jit 的 JIT 框架
3. 实现 ferrous_tools 的 java 启动器

---

## 6. 核心技术指标

### 6.1 性能目标

| 指标 | 目标 |
|------|------|
| 启动时间 | 相比 OpenJDK < 90% |
| 峰值性能 | 相比 OpenJDK > 100% |
| GC 暂停 | ZGC < 1ms |
| 内存占用 | 相比 OpenJDK < 80% |

### 6.2 兼容性目标

| 指标 | 目标 |
|------|------|
| jtreg 测试通过率 | ≥ 95% |
| 主流框架兼容性 | 100% |
| JVM 语言兼容性 | 100% |

---

## 7. 相关文档

- [开发环境搭建](../dev-guide/env-setup.md) - 本地开发环境配置
- [模块开发指南](../dev-guide/module-guide.md) - crate 结构和贡献代码
- [native 方法重写指南](../dev-guide/native-rewrite.md) - 标准库 native 方法重写
- [JDK17 适配指南](../dev-guide/jdk17-adapt.md) - 版本差异处理

---

**最后更新**: 2026-04-02

**下次更新计划**: 里程碑 0-1 完成时更新
