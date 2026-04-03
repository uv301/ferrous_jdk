# FerrousJDK 变更日志

本文档记录 FerrousJDK 的版本变更历史。

## 版本格式规范

### 版本号格式

```
v{主版本}.{次版本}.{补丁版本}-jdk{lts版本}[-ecj|-rc|-beta]
```

### 版本标识说明

| 标识 | 说明 | 示例 |
|------|------|------|
| 无 | 正式版 | `v1.0.0-jdk8-lts` |
| `-ecj` | ECJ 编译器阶段 | `v0.4.0-jdk8-ecj-beta` |
| `-beta` | Beta 测试阶段 | `v0.4.3-jdk8-ecj-beta` |
| `-rc` | Release Candidate | `v0.5.3-jdk8-rc` |

### 版本生命周期

| 阶段 | 版本标识 | 说明 |
|------|----------|------|
| 开发版 | 无或 `-ecj` | 功能开发中，编译器使用 ECJ 封装 |
| Beta 版 | `-beta` | 特性冻结，进入测试阶段 |
| RC 版 | `-rc` | 候选发布，准备正式发布 |
| 正式版 | 无 | 正式发布，移除所有后缀 |

### Tag 与里程碑对应

| 里程碑 | Tag | 说明 |
|---------|-----|------|
| 0-1 | `v0.0.1-init` | 仓库初始化完成 |
| 0-2 | `v0.0.2-infra-ready` | 基础设施就绪 |
| 1-1 | `v0.0.3-classfile-parser-ready` | ClassFile 解析器完成 |
| 1-2 | `v0.1.0-jdk8-ecj-mvp` | MVP 完成（HelloWorld 可执行） |
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
| 6-3 | `v1.0.0-jdk8-lts` | JDK8 LTS 正式发布 |

---

## [Unreleased] - 开发中

### Added

#### 项目结构

- **Workspace 配置**: 完整的 Cargo Workspace 设置，包含 11 个 crate
- **ferrous-utils**: 基础工具库模块结构（collections, concurrency, error, log, memory, platform）
- **ferrous-core**: JVM 核心框架模块结构（class_file, class_loader, exception, invoke_dynamic, linking, reflection, runtime, sync）
- **ferrous-interpreter**: 字节码解释器模块结构（instructions, interpreter, optimizations, osr）
- **ferrous-gc**: 垃圾回收器模块结构（common, serial, parallel, g1gc, zgc, shenandoah）
- **ferrous-stdlib**: Java 标准库框架
- **ferrous-tools**: JDK 工具链框架
- **ferrous-javac**: Java 编译器框架
- **ferrous-jit**: JIT 编译器框架
- **ferrous-jni**: JNI 实现框架
- **ferrous-jvmti**: JVMTI 实现框架
- **ferrous-build**: 构建系统框架

#### 文档

- **CONTRIBUTING.md**: 贡献指南
- **CODE_OF_CONDUCT.md**: 行为准则
- **docs/roadmap/README.md**: 完整路线图
- **docs/changelog/CHANGELOG.md**: 变更日志

### Changed

### Deprecated

### Removed

### Fixed

### Security

### Known Issues

- 所有 crate 代码均为占位符状态，需要实际实现
- 尚未配置 CI/CD 流水线
- 尚未集成 ECJ 编译器

---

## [v0.5.3-jdk8-rc] - JDK8 Release Candidate

发布日期: 待定

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.5.2-jdk8-javac-complete] - 自研 Java 编译器完成

发布日期: 待定

> **重要变更**: 此版本移除 `-ecj` 标识，标志着 FerrousJDK 使用自研 Rust 版 Java 编译器替代 ECJ 封装。

### Added

#### 编译器

- **ferrous-javac**: 自研 Rust 版 Java 编译器
  - 词法分析器（Lexer）实现
  - 语法分析器（Parser）实现
  - 语义分析器（Sema）实现
  - 字节码生成器（Codegen）实现
  - 完整 Java 8 语法支持

#### JVM TI

- **JVM Tool Interface**: 完整 JVM TI 规范实现
  - 全量事件机制（Breakpoint、Exception、MethodEntry 等）
  - 能力管理（CanGet*、CanSet*）
  - 与 OpenJDK 一致的头文件

#### Agent 支持

- **Java Agent**: `-javaagent` 参数支持
  - 静态 Instrumentation
  - 动态 Instrumentation

#### invokedynamic

- **invokedynamic 指令**: 完整实现
  - CallSite 机制
  - MethodHandle 实现
  - Lambda metafactory 支持

#### 工具链完善

- **jmap**: 内存映射分析工具
- **jstack**: 线程栈 dump 工具
- **jconsole**: JMX 控制台
- **jdb**: Java 调试器

### Changed

- **javac**: 从 ECJ 封装切换到自研编译器
- 移除所有 ECJ 相关依赖
- 所有版本 Tag 移除 `-ecj` 标识

### Deprecated

### Removed

- **ECJ 依赖**: 完全移除 Eclipse Compiler for Java

### Fixed

### Security

---

## [v0.5.1-jdk8-ecj-jit-zgc-complete] - JIT 高级优化与 ZGC 完成

发布日期: 待定

### Added

#### JIT 高级优化

- **方法内联**: 支持热点方法内联
- **逃逸分析**: 对象作用域分析，栈上分配优化
- **锁消除**: 无竞争锁优化
- **循环展开**: 小循环体展开优化
- **常量折叠**: 编译期常量计算

#### 分层编译

- **C1 编译器**: 快速编译，有限优化
- **C2 编译器**: 激进优化，深度优化
- **OSR**: 栈上替换，热循环即时编译

#### ZGC 低延迟 GC

- **着色指针**: 指针位存储标记信息
- **并发阶段**: 标记、重定位、引用处理并发执行
- **读屏障**: 仅读取引用时额外处理
- **STW 控制**: 平均 STW 时间 ≤ 1ms

### Changed

- **JIT 默认启用**: 分层编译默认开启

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.5.0-jdk8-ecj-jit-baseline] - JIT 即时编译器基础

发布日期: 待定

### Added

#### JIT 编译器框架

- **ferrous-jit**: JIT 即时编译器核心框架
  - Cranelift 后端支持
  - LLVM 后端支持（可选）
  - 热点方法探测
  - 字节码 → IR 转换
  - IR → 机器码生成

#### 基础优化

- **常量传播**: 编译期常量传播优化
- **死代码消除**: 移除不可达代码
- **简单循环优化**: 基础循环优化

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.4.3-jdk8-ecj-beta] - JDK8 Beta 发布

发布日期: 待定

### Added

#### JDK8 专属特性

- **Stream API**: 完整实现
  - Filter、Map、Reduce 操作
  - 并行流支持
  - Collectors 工具类

- **Lambda 表达式**: 完整支持
  - LambdaMetafactory 实现
  - 方法引用支持

- **Date Time API**: `java.time` 包完整实现
  - LocalDate、LocalTime、LocalDateTime
  - ZonedDateTime、OffsetDateTime
  - Duration、Period

- **接口增强**: 默认方法和静态方法支持

#### JNI 完整接口

- 全 JNI 规范函数实现
- 第三方 native 库兼容

#### 工具完善

- **jar 工具**: JAR 包创建、解压、查看
- **javap 工具**: Class 文件反汇编

#### 测试集成

- **jtreg 测试套件**: OpenJDK 官方测试集成
- JDK8 核心 API 覆盖率 ≥ 90%

### Changed

- **ferrous-build**: 产物目录结构 1:1 对齐 OpenJDK8

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.4.2-jdk8-ecj-net-security-complete] - 网络与安全体系完成

发布日期: 待定

### Added

#### java.net 网络编程

- **Socket 体系**: TCP/UDP Socket
- **ServerSocket**: 服务端 Socket
- **InetAddress**: IP 地址处理
- **URL/URLConnection**: URL 处理和连接
- **DatagramSocket**: 数据报 Socket

#### HTTP/HTTPS 支持

- HTTP 协议基础实现
- HTTPS 协议支持（TLS/SSL）

#### java.security 安全框架

- **MessageDigest**: MD5、SHA-1、SHA-256 等
- **SecureRandom**: 安全随机数生成
- **Key/Certificate**: 密钥和证书处理

#### JCE 加密扩展

- AES 对称加密
- RSA 非对称加密
- MAC 消息认证

#### 安全管理器

- 权限控制体系
- 策略文件机制

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.4.1-jdk8-ecj-io-nio-complete] - IO/NIO 体系完成

发布日期: 待定

### Added

#### java.io 字节流

- **InputStream/OutputStream**: 基础流
- **FileInputStream/FileOutputStream**: 文件流
- **BufferedInputStream/BufferedOutputStream**: 缓冲流
- **ByteArrayInputStream/ByteArrayOutputStream**: 字节数组流

#### java.io 字符流

- **Reader/Writer**: 基础字符流
- **FileReader/FileWriter**: 文件字符流
- **BufferedReader/BufferedWriter**: 缓冲字符流
- **InputStreamReader/OutputStreamWriter**: 转换流

#### 文件操作

- **File**: 文件和目录操作
- **RandomAccessFile**: 随机访问文件

#### java.nio 缓冲区

- **ByteBuffer**: 字节缓冲区
- **CharBuffer**: 字符缓冲区
- **IntBuffer/LongBuffer**: 其他基本类型缓冲区
- 堆内缓冲区、堆外直接缓冲区

#### java.nio.channels

- **FileChannel**: 文件通道
- **SocketChannel**: Socket 通道
- **ServerSocketChannel**: 服务端 Socket 通道
- **DatagramChannel**: 数据报通道
- 阻塞 IO、非阻塞 IO 支持

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.4.0-jdk8-ecj-collection-complete] - 集合框架完成

发布日期: 待定

### Added

#### List 接口实现

- **ArrayList**: 动态数组实现
- **LinkedList**: 双向链表实现
- **Vector**: 同步动态数组
- **Stack**: 栈实现

#### Set 接口实现

- **HashSet**: 基于 HashMap 的 Set
- **LinkedHashSet**: 保持插入顺序的 Set
- **TreeSet**: 基于红黑树的排序 Set

#### Map 接口实现

- **HashMap**: 哈希表实现
- **LinkedHashMap**: 保持插入顺序的 Map
- **TreeMap**: 基于红黑树的排序 Map
- **Hashtable**: 同步哈希表

#### 工具类

- **Collections**: 集合工具类
- **Arrays**: 数组工具类

#### Stream API 基础

- **java.util.stream**: Stream 框架
- **Collector/T.Collectors**: 收集器

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.3.2-jdk8-ecj-gc-complete] - GC 全功能完成

发布日期: 待定

### Added

#### 引用类型

- **SoftReference**: 软引用
- **WeakReference**: 弱引用
- **PhantomReference**: 虚引用
- **ReferenceQueue**: 引用队列

#### 解释器优化

- **栈顶缓存**: TOSCA 优化
- **常量池缓存**: 减少索引计算
- **热点计数**: 方法/循环执行统计
- **指令调度**: 优化指令执行顺序

#### 监控工具

- **jps**: JVM 进程状态工具
- **jstat**: JVM 统计监控工具

#### JVM 调优参数

- 堆内存配置
- 分代比例配置
- GC 日志配置
- 所有 OpenJDK -XX 参数兼容

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.3.1-jdk8-ecj-gc-core-complete] - 分代 GC 核心完成

发布日期: 待定

### Added

#### STW 机制

- **安全点**: SafePoint 实现
- **安全区域**: SafeRegion 实现
- 线程正确暂停和恢复

#### 年轻代 GC (Minor GC)

- **Eden 区分配**: 新对象分配
- **Survivor 区**: S0、S1 复制
- **年龄计数**: 对象年龄跟踪
- **晋升逻辑**: 对象晋升老年代

#### 老年代 GC (Full GC)

- **标记阶段**: 存活对象标记
- **清除阶段**: 垃圾对象清除
- **整理阶段**: 内存碎片整理

#### finalize() 支持

- 对象 finalization 机制
- finalize() 方法执行

#### GC 日志

- GC 详情日志
- GC 耗时日志
- 内存变化日志

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.3.0-jdk8-ecj-memory-management-ready] - 内存管理就绪

发布日期: 待定

### Added

#### 分代堆内存管理

- **Eden 区**: 年轻代 Eden 空间
- **Survivor0/Survivor1**: 幸存者空间
- **老年代**: Old Generation
- 可配置的内存比例

#### TLAB 分配器

- **线程本地分配缓冲区**: Thread-Local Allocation Buffer
- 多线程分配无竞争
- 分配性能优化

#### 根节点枚举

- 虚拟机栈根节点
- 本地方法栈根节点
- 方法区根节点
- 运行时常量池根节点
- JNI Handles 根节点

#### 可达性分析

- **引用链遍历**: GC Roots 到对象的引用链
- **存活对象标记**: 准确标记存活对象
- **垃圾对象识别**: 不可达对象识别

#### OOM 检测

- **堆内存溢出**: OutOfMemoryError
- **栈内存溢出**: StackOverflowError
- 准确的异常类型和消息

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.2.2-jdk8-ecj-core-complete] - java.lang 核心完成

发布日期: 待定

### Added

#### java.lang.Object

- equals()、hashCode()、toString()
- getClass()、clone()、finalize()
- 所有 native 方法 Rust 重写

#### java.lang.Class

- 类元信息访问
- 类型判断
- 反射基础支持

#### java.lang.String

- UTF-16 编码内部存储
- 字符串操作核心方法
- 压缩字符串优化（LATIN1/UTF16）

#### 其他核心类

- **java.lang.System**: 系统类，native 方法 Rust 重写
- **java.lang.Thread**: 线程类
- **java.lang.Throwable**: 异常基类
- **java.lang.ClassLoader**: 类加载器基类
- **java.lang.StringBuilder/StringBuffer**: 字符串构建

#### 基础反射机制

- **Class.forName()**: 类加载
- **字段访问**: getField/setField
- **方法调用**: invoke
- **构造器实例化**: newInstance

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.2.1-jdk8-ecj-thread-lock-complete] - 线程锁机制完成

发布日期: 待定

### Added

#### JVM 线程模型

- OS 线程 1:1 映射
- 线程创建、启动、休眠、终止
- 线程中断机制

#### 对象头结构

- Mark Word（哈希码、GC 年龄、锁状态）
- 线程 ID / 指向 Monitor 的指针
- 偏向锁、轻量级锁、重量级锁状态

#### synchronized 语义

- 方法级同步
- 代码块级同步
- 锁粗化优化

#### wait/notify 机制

- **锁池**: ObjectMonitor 锁池
- **等待池**: WaitSet 等待池
- **wait()**: 等待通知
- **notify()/notifyAll()**: 通知等待线程

#### 异常处理

- 全部异常类型支持
- 受检异常、非受检异常、错误
- 完整的栈轨迹输出

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.2.0-jdk8-ecj-bytecode-complete] - 全字节码指令完成

发布日期: 待定

### Added

#### 全部字节码指令

- **加载/存储指令**: iconst、iload、istore 等
- **算术指令**: iadd、isub、imul、idiv 等
- **类型转换指令**: i2l、i2f、i2d 等
- **对象操作指令**: new、getfield、putfield 等
- **方法调用指令**: invokevirtual、invokespecial、invokestatic、invokeinterface
- **跳转指令**: ifeq、ifne、goto、tableswitch 等
- **返回指令**: ireturn、areturn、return 等
- **数组指令**: newarray、iaload、iastore 等
- **异常指令**: athrow
- **同步指令**: monitorenter、monitorexit

#### 类加载器体系

- **Bootstrap ClassLoader**: 启动类加载器
- **Extension ClassLoader**: 扩展类加载器
- **Application ClassLoader**: 应用类加载器
- **双亲委派模型**: Parent Delegation Model
- **自定义类加载器**: ClassLoader 扩展支持

#### 类验证流程

- **格式验证**: ClassFile 格式检查
- **语义验证**: 语义正确性检查
- **字节码验证**: 类型系统验证
- **符号引用验证**: 符号引用解析

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.1.0-jdk8-ecj-mvp] - MVP 完成

发布日期: 待定

> **里程碑**: FerrousJDK 首个可用版本，可执行 HelloWorld 程序。

### Added

#### ClassFile 解析器

- 魔数验证（0xCAFEBABE）
- 版本号解析
- 常量池完整解析（所有常量池类型）
- 字段表、方法表、属性表解析

#### 运行时数据区

- **方法区**: 类元信息存储
- **堆**: 对象分配
- **Java 栈**: 方法调用栈
- **本地方法栈**: native 方法栈
- **程序计数器**: 指令地址

#### 栈帧结构

- 局部变量表
- 操作数栈
- 动态链接

#### 启动类加载器

- 类的加载、验证、准备、解析、初始化
- 完整 JVMS 规范流程

#### 字节码解释器

- 核心指令实现（约 60% 常用指令）
- 基本异常处理

#### java 启动器

- `./bin/java HelloWorld` 命令支持

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.0.3-classfile-parser-ready] - ClassFile 解析器就绪

发布日期: 待定

### Added

- 完整的 ClassFile 解析器
- JVMS8 §4 完全符合
- 全量单元测试

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.0.2-infra-ready] - 基础设施就绪

发布日期: 2026-04-01

### Added

- **CI/CD 流水线**: GitHub Actions 配置
- **ECJ 集成**: javac 命令封装
- **测试框架**: 单元测试和集成测试
- **Rustfmt/Clippy**: 代码规范配置

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [v0.0.1-init] - 项目初始化

发布日期: 2026-03-31

### Added

- **Git 仓库**: 完整的 Git 仓库初始化
- **Cargo Workspace**: Rust Workspace 配置
- **核心分支**: `dev`、`jdk8-lts`
- **规范文档**: README、CONTRIBUTING、CODE_OF_CONDUCT
- **基础目录结构**: 核心 crate 骨架

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## 版本历史说明

### 分支策略

| 分支 | 用途 | 保护状态 |
|------|------|----------|
| `dev` | 主开发集成分支 | 受保护 |
| `jdk8-lts` | JDK8 LTS 长期维护分支 | 受保护 |
| `jdk17-lts` | JDK17 LTS 长期维护分支 | 受保护 |
| `jdk21-lts` | JDK21 LTS 长期维护分支 | 受保护 |
| `feature/*` | 功能开发分支 | 临时 |
| `bugfix/*` | Bug 修复分支 | 临时 |
| `release/*` | 发布准备分支 | 临时 |
| `hotfix/*` | 热修复分支 | 临时 |

### 提交规范

我们遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <subject>

<body>

<footer>
```

类型：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式
- `refactor`: 重构
- `test`: 测试
- `chore`: 构建/工具

---

## 迁移指南

### 从 v0.4.x 迁移到 v0.5.x

无破坏性变更。

### 从 v0.5.x 迁移到 v1.0.0

> **重要**: v1.0.0 移除了 `-ecj` 标识。如有使用 ECJ 特定功能，请切换到标准 Java 编译。

### 从 v0.x 迁移到 v1.0.0

- 无破坏性 API 变更
- 兼容性配置文件格式不变
- JVM 选项完全兼容

---

## 联系与支持

- **Issue**: https://github.com/uv301/ferrous_jdk/issues
- **讨论**: https://github.com/uv301/ferrous_jdk/discussions
- **Discord**: https://discord.gg/ferrous-jdk

---

**最后更新**: 2026-04-02
