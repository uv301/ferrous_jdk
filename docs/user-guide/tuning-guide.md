# 性能调优指南

本文档提供 FerrousJDK 性能调优的详细指南，涵盖 GC 调优、JIT 配置、内存管理和线程优化。

## 1. 性能调优概述

### 1.1 调优原则

1. **测量优先**：使用工具收集数据，量化性能问题
2. **渐进优化**：每次只改一个参数，验证效果
3. **权衡取舍**：性能调优通常是权衡的艺术
4. **场景适配**：根据工作负载特性选择最佳配置

### 1.2 性能诊断工具

| 工具 | 用途 |
|------|------|
| `jstat` | GC 和 JIT 统计 |
| `jmap` | 堆分析和转储 |
| `jstack` | 线程分析 |
| `jcmd` | 综合诊断 |
| VisualVM | 可视化分析 |

## 2. 垃圾回收调优

### 2.1 GC 选择指南

| GC 类型 | 适用场景 | 启动参数 |
|--------|----------|----------|
| Serial GC | 小内存 (<100MB)、单核、无暂停要求 | `-XX:+UseSerialGC` |
| Parallel GC | 注重吞吐量、批处理作业 | `-XX:+UseParallelGC` |
| G1 GC | 平衡延迟和吞吐量、中大型堆 | `-XX:+UseG1GC` (JDK9+ 默认) |
| ZGC | 超低延迟 (<1ms)、超大堆 (>4GB) | `-XX:+UseZGC` |
| Shenandoah | 低延迟、中大型堆 | `-XX:+UseShenandoahGC` |

### 2.2 G1 GC 调优

G1 (Garbage First) 是 JDK9+ 的默认 GC，推荐用于大多数场景。

#### 基础参数

```bash
# 设置目标暂停时间 (默认 200ms)
java -XX:MaxGCPauseMillis=100 -jar myapp.jar

# 设置堆大小
java -Xms4g -Xmx4g -XX:+UseG1GC -jar myapp.jar

# 设置 GC 线程数
java -XX:ParallelGCThreads=8 -XX:ConcGCThreads=4 -XX:+UseG1GC -jar myapp.jar
```

#### 进阶参数

```bash
# 调整堆region大小
-XX:G1HeapRegionSize=<size>    # 1MB, 2MB, 4MB, 8MB, 16MB, 32MB

# 调整 InitiatingHeapOccupancyPercent
-XX:InitiatingHeapOccupancyPercent=45  # 默认45%

# 调整 G1ReservePercent
-XX:G1ReservePercent=10   # 默认10%

# 混合 GC 调优
-XX:G1MixedGCLiveThresholdPercent=85      # 默认85%
-XX:G1HeapWastePercent=5                  # 默认5%
```

#### G1 调优示例

```bash
# 低延迟优先配置
java -Xms8g -Xmx8g \
    -XX:+UseG1GC \
    -XX:MaxGCPauseMillis=50 \
    -XX:G1HeapRegionSize=8m \
    -XX:ParallelGCThreads=16 \
    -XX:ConcGCThreads=4 \
    -XX:+UnlockExperimentalVMOptions \
    -XX:G1NewSizePercent=30 \
    -XX:G1MaxNewSizePercent=60 \
    -jar myapp.jar
```

### 2.3 ZGC 调优

ZGC 是专为超低延迟设计的 GC，暂停时间通常 <1ms。

#### 基础参数

```bash
# 启用 ZGC
java -XX:+UseZGC -jar myapp.jar

# 设置堆大小
java -Xms16g -Xmx16g -XX:+UseZGC -jar myapp.jar

# 设置并发 GC 线程数
java -XX:ConcGCThreads=8 -XX:+UseZGC -jar myapp.jar
```

#### ZGC 进阶配置

```bash
# 调优延迟目标
-XX:SoftMaxHeapSize=<size>   # 软上限，允许内存压力下减少堆

# NUMA 感知 (多插槽服务器)
-XX:+UseNUMA

# 大页面支持
java -XX:+UseLargePages -XX:ZPath=/hugepages -jar myapp.jar
```

#### ZGC 调优示例

```bash
# 超低延迟配置
java -Xms32g -Xmx32g \
    -XX:+UseZGC \
    -XX:ConcGCThreads=12 \
    -XX:SoftMaxHeapSize=24g \
    -XX:+UseNUMA \
    -jar myapp.jar
```

### 2.4 Parallel GC 调优

适用于批处理和计算密集型应用。

```bash
# 基础配置
java -Xms8g -Xmx8g -XX:+UseParallelGC -jar myapp.jar

# 吞吐量调优
java -Xms8g -Xmx8g \
    -XX:+UseParallelGC \
    -XX:ParallelGCThreads=16 \
    -XX:+UseParallelOldGC \
    -XX:-UseAdaptiveSizePolicy \
    -XX:OldSize=4g \
    -XX:NewSize=2g \
    -jar myapp.jar
```

### 2.5 GC 日志配置

```bash
# 基础 GC 日志
java -Xlog:gc*:file=gc.log -jar myapp.jar

# 详细 GC 日志
java -Xlog:gc*=debug:file=gc_debug.log:time,uptime,level,tags -jar myapp.jar

# 使用 GC 日志旋转
java -Xlog:gc*:file=gc.log:filecount=10,filesize=100m -jar myapp.jar

# ZGC 特定日志
java -Xlog:gc*:file=gc_zgc.log -XX:+UseZGC -jar myapp.jar
```

### 2.6 GC 调优检查清单

- [ ] 堆大小是否合理？(`-Xms` 和 `-Xmx` 应该相等)
- [ ] GC 类型是否适合工作负载？
- [ ] GC 暂停时间是否满足 SLA？
- [ ] 吞吐量是否满足需求？
- [ ] GC 日志显示是否正常？

## 3. JIT 编译器调优

### 3.1 分层编译

分层编译是 JIT 优化的关键：

| 层级 | 描述 | 触发条件 |
|------|------|----------|
| 0 | 解释执行 | 默认 |
| 1 | C1 快速编译 | 方法调用 1000 次 或 循环回边 10000 次 |
| 2 | C2 完整编译 | 方法调用 10000 次 或 循环回边 100000 次 |

```bash
# 启用分层编译 (JDK8 默认)
java -XX:+TieredCompilation -jar myapp.jar

# 禁用分层编译
java -XX:-TieredCompilation -jar myapp.jar
```

### 3.2 JIT 日志

```bash
# 打印编译方法
java -XX:+PrintCompilation -jar myapp.jar

# 详细编译日志
java -XX:+LogCompilation -XX:LogFile=compilation.log -jar myapp.jar

# 分析编译日志
java -XX:+UnlockDiagnosticVMOptions \
     -XX:+PrintInlining -jar myapp.jar
```

### 3.3 内联控制

```bash
# 最大内联方法大小
-XX:MaxInlineSize=35           # 默认 35 字节
-XX:FreqInlineSize=325         # 热点方法最大内联 325 字节

# 禁用特定方法内联
-XX:CompileCommand=exclude,com/example/MyClass::expensiveMethod

# 强制内联特定方法
-XX:CompileCommand=inline,com/example/Utility::*
```

### 3.4 热点探测阈值

```bash
# 调整 Tier0 触发阈值
-XX:Tier0Threshold=100        # 编译前解释执行次数
-XX:Tier1Threshold=2000       # C1 编译阈值
-XX:Tier2Threshold=10000      # C2 编译阈值

# 适用场景
# - 短生命周期应用：降低阈值更快触发编译
# - 长运行应用：可使用默认值
```

### 3.5 逃逸分析

```bash
# 启用逃逸分析 (默认开启)
-XX:+DoEscapeAnalysis -jar myapp.jar

# 禁用逃逸分析 (用于调试)
-XX:-DoEscapeAnalysis -jar myapp.jar
```

## 4. 内存调优

### 4.1 堆内存设置

```bash
# 初始和最大堆设置
java -Xms4g -Xmx4g -jar myapp.jar

# 生产环境建议
# - 初始堆 = 最大堆 (避免动态调整开销)
# - 最大堆 = 可用内存的 50-75%
# - 考虑其他进程内存需求
```

### 4.2 Metaspace 调优 (JDK8+)

```bash
# 元空间初始大小
-XX:MetaspaceSize=256m

# 元空间最大大小 (无上限可能导致 OOM)
-XX:MaxMetaspaceSize=512m

# JDK8 PermGen 设置 (已废弃)
-XX:PermSize=256m
-XX:MaxPermSize=512m
```

### 4.3 直接内存

```bash
# 直接内存最大大小 (默认与堆大小相同)
-XX:MaxDirectMemorySize=4g

# 适用场景：NIO 大量使用直接缓冲区
```

### 4.4 线程栈

```bash
# 线程栈大小
-Xss1m          # 默认 1MB
-Xss256k        # 更小的栈，减少内存占用

# 适用场景
# - 大量线程：减小栈大小
# - 深层递归：增大栈大小
```

### 4.5 对象分配

```bash
# TLAB (线程本地分配缓冲区)
-XX:+UseTLAB              # 启用 TLAB (默认)
-XX:TLABSize=256k         # TLAB 大小
-XX:-ResizeTLAB           # 禁用 TLAB 动态调整

# 对象头优化
-XX:+UseCompressedClassPointers    # 压缩类指针 (默认)
-XX:+UseCompressedOops            # 压缩对象指针 (默认)
```

## 5. 线程调优

### 5.1 线程数计算

```bash
# 线程数公式
线程数 = CPU核心数 * 目标利用率 * (1 + 等待时间/计算时间)

# 示例：CPU密集型 (4核，100%利用率)
java -XX:CICompilerCount=4 -jar myapp.jar

# 示例：IO密集型 (4核，100%利用率，等待时间=4倍计算时间)
java -XX:CICompilerCount=8 -jar myapp.jar
```

### 5.2 JIT 编译线程

```bash
# 编译器线程数
-XX:CICompilerCount=4      # 默认 = CPU核心数

# C1 编译器线程
-XX:CompilerCount=3

# 适用场景
# - 大量短方法：增加编译器线程
# - 编译时间过长：减少编译器线程
```

### 5.3 GC 线程

```bash
# 并行 GC 线程数
-XX:ParallelGCThreads=8   # 默认 = CPU核心数

# 并发 GC 线程数
-XX:ConcGCThreads=4        # 默认 = ParallelGCThreads / 4

# 建议
# - GC 线程总数 <= CPU核心数
# - 避免与业务线程争抢 CPU
```

### 5.4 虚拟线程 (JDK21+)

```bash
# 虚拟线程配置
java -XX:+UseVirtualThreads -jar myapp.jar

# 虚拟线程栈大小
-XX:VirtualThreadMinMountThreshold=20  # 默认 20
-XX:VirtualThreadContendedLocking=true   # 启用争用检测

# 适用场景
# - 大量并发任务
# - IO 密集型应用
# - 任务数量 >> CPU 核心数
```

## 6. 应用层优化

### 6.1 类数据共享 (CDS)

```bash
# 启用 CDS (JDK10+ 默认开启)
java -Xshare:on -jar myapp.jar

# 转储 CDS 归档
java -Xshare:dump -XX:+UseG1GC -jar myapp.jar

# 使用 CDS 归档
java -Xshare:on -XX:SharedArchiveFile=archive.jsa -jar myapp.jar
```

### 6.2 应用类数据共享 (AppCDS)

```bash
# 1. 创建应用类列表
java -XX:+UseG1GC -Xshare:off \
     -XX:DumpLoadedClassList=classes.lst \
     -jar myapp.jar

# 2. 创建 CDS 归档
java -XX:+UseG1GC \
     -XX:SharedClassListFile=classes.lst \
     -XX:SharedArchiveFile=app_cds.jsa \
     -Xshare:dump

# 3. 使用 AppCDS 运行
java -XX:+UseG1GC \
     -XX:SharedArchiveFile=app_cds.jsa \
     -Xshare:on \
     -jar myapp.jar
```

### 6.3 提前编译 (AOT) (JDK9+)

```bash
# 编译模块
jaotc --output library.so --lib-name lib \
      --module java.base

# 使用 AOT 编译的库
java -XX:+UseAOT \
     -XX:AOTLibrary=./library.so \
     -jar myapp.jar
```

## 7. 生产环境配置示例

### 7.1 Web 应用 (低延迟优先)

```bash
java -Xms4g -Xmx4g \
    -XX:+UseG1GC \
    -XX:MaxGCPauseMillis=50 \
    -XX:G1HeapRegionSize=4m \
    -XX:ParallelGCThreads=8 \
    -XX:ConcGCThreads=4 \
    -XX:+TieredCompilation \
    -XX:+UseStringDeduplication \
    -Xlog:gc*:file=gc.log \
    -jar webapp.jar
```

### 7.2 大数据处理 (高吞吐)

```bash
java -Xms16g -Xmx16g \
    -XX:+UseParallelGC \
    -XX:ParallelGCThreads=16 \
    -XX:+UseParallelOldGC \
    -XX:-UseAdaptiveSizePolicy \
    -XX:NewSize=4g \
    -XX:MaxNewSize=4g \
    -XX:OldSize=12g \
    -XX:SurvivorRatio=8 \
    -XX:+TieredCompilation \
    -Xlog:gc=debug:file=gc.log \
    -jar batchprocessor.jar
```

### 7.3 微服务 (通用)

```bash
java -Xms2g -Xmx2g \
    -XX:+UseG1GC \
    -XX:MaxGCPauseMillis=100 \
    -XX:+TieredCompilation \
    -XX:+UseStringDeduplication \
    -XX:+HeapDumpOnOutOfMemoryError \
    -XX:HeapDumpPath=/var/logs \
    -Xlog:gc*:file=gc.log:time,uptime,tags \
    -jar microservice.jar
```

### 7.4 JDK21 虚拟线程配置

```bash
java -Xms4g -Xmx4g \
    -XX:+UseG1GC \
    -XX:+UseVirtualThreads \
    -XX:VirtualThreadMinMountThreshold=5 \
    -XX:MaxGCPauseMillis=50 \
    -XX:+TieredCompilation \
    -Xlog:gc*:file=gc.log \
    -jar virtualthread-app.jar
```

## 8. 性能监控

### 8.1 JFR (Java Flight Recorder)

```bash
# 启动时启用 JFR
java -XX:+FlightRecorder \
     -XX:StartFlightRecording=dumponexit=true,filename=recording.jfr \
     -jar myapp.jar

# JMC 连接实时录制
java -XX:+FlightRecorder \
     -XX:StartFlightRecording=settings=profile \
     -jar myapp.jar
```

### 8.2 性能基准测试

```bash
# 使用 JMH 进行基准测试
java -jar benchmarks.jar

# 运行 DaCapo 基准
java -jar dacapo-9.12-bach.jar -n 10 mybenchmark

# 运行 SPECjvm2008
java -jar SPECjvm2008.jar
```

## 9. 常见问题排查

| 问题 | 症状 | 解决方案 |
|------|------|----------|
| GC 暂停过长 | 延迟尖峰 | 调整 MaxGCPauseMillis，考虑 ZGC |
| 内存泄漏 | 堆持续增长 | 使用 jmap 分析堆转储 |
| JIT 编译过慢 | 启动慢 | 使用分层编译，减少编译阈值 |
| 线程争用 | CPU 利用率低 | 分析锁争用，使用轻量级锁 |
| 类加载慢 | 启动慢 | 使用 CDS/AppCDS |
