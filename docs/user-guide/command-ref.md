# JDK 工具命令参考

本文档提供 FerrousJDK 完整工具链的详细命令参考，所有工具均与 OpenJDK 保持 100% 参数兼容。

## 1. java - JVM 启动器

### 1.1 语法

```bash
java [options] mainclass [arguments...]
java [options] -jar jarfile [arguments...]
```

### 1.2 标准选项

| 选项 | 描述 |
|------|------|
| `-cp <class search path>` | 类路径，多个目录用 `:` (Unix) 或 `;` (Windows) 分隔 |
| `-classpath <class search path>` | 同 `-cp` |
| `-D<name>=<value>` | 设置系统属性 |
| `-version` | 打印版本信息 |
| `-X` | 打印 `-X` 选项帮助 |
| `-?` 或 `-help` | 打印帮助信息 |
| `-enableassertions[:<packagename>...\|:<packagename>...\|]` | 启用断言 |
| `-disableassertions[:<packagename>...\|:<packagename>...\|]` | 禁用断言 |

### 1.3 运行时选项

| 选项 | 描述 |
|------|------|
| `-ea[:<packagename>...\|:<packagename>...\|]` | 启用断言 (同 `-enableassertions`) |
| `-da[:<packagename>...\|:<packagename>...\|]` | 禁用断言 |
| `-esa` | 启用系统类断言 |
| `-dsa` | 禁用系统类断言 |

### 1.4 性能调优选项

#### 堆内存

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `-Xms<size>` | 初始堆大小 | 物理内存/64 |
| `-Xmx<size>` | 最大堆大小 | 物理内存/4 |
| `-Xss<size>` | 线程栈大小 | 1 MB |

#### 垃圾回收器

| 选项 | 描述 |
|------|------|
| `-XX:+UseSerialGC` | 使用串行 GC |
| `-XX:+UseParallelGC` | 使用并行 GC |
| `-XX:+UseConcMarkSweepGC` | 使用 CMS GC (JDK8, 已废弃) |
| `-XX:+UseG1GC` | 使用 G1 GC |
| `-XX:+UseZGC` | 使用 ZGC |
| `-XX:+UseShenandoahGC` | 使用 Shenandoah GC |

#### JIT 编译

| 选项 | 描述 |
|------|------|
| `-Xint` | 仅使用解释器模式 |
| `-Xcomp` | 优先编译模式 |
| `-Xmixed` | 混合模式 (默认) |
| `-XX:+TieredCompilation` | 启用分层编译 |

### 1.5 示例

```bash
# 运行主类
java -cp lib/myapp.jar com.example.Main arg1 arg2

# 运行 JAR 文件
java -jar myapp.jar

# 设置系统属性
java -Dapp.name=MyApp -Dlog.level=DEBUG -cp . MyApp

# 调优内存
java -Xms512m -Xmx2g -Xss256k -XX:+UseG1GC -jar myapp.jar

# 调试模式
java -ea -Xdebug -Xrunjdwp:transport=dt_socket,server=y,suspend=n,address=5005 MyApp
```

## 2. javac - Java 编译器

### 2.1 语法

```bash
javac [options] sourcefiles...
javac [options] @files
```

### 2.2 常用选项

| 选项 | 描述 |
|------|------|
| `-d <directory>` | 指定输出目录 |
| `-cp <path>` | 指定类路径 |
| `-source <release>` | 指定源代码版本 (8, 9, 11, 17, 21) |
| `-target <release>` | 指定目标字节码版本 |
| `-encoding <encoding>` | 指定源文件编码 |
| `-g` | 生成所有调试信息 |
| `-g:none` | 不生成调试信息 |
| `-O` | 优化编译 (已废弃) |
| `-verbose` | 打印编译详情 |
| `-deprecation` | 打印已废弃 API 使用警告 |
| `-nowarn` | 禁用警告 |
| `-Xlint:<keys>` | 启用特定警告检查 |

### 2.3 警告检查选项

| 选项 | 描述 |
|------|------|
| `-Xlint:all` | 启用所有警告 |
| `-Xlint:none` | 禁用所有警告 |
| `-Xlint:unchecked` | 未检查操作警告 |
| `-Xlint:deprecation` | 废弃 API 警告 |
| `-Xlint:rawtypes` | 原始类型警告 |
| `-Xlint:cast` | 冗余类型转换警告 |
| `-Xlint:divzero` | 除零警告 |
| `-Xlint:empty` | 空语句块警告 |
| `-Xlint:path` | 无效路径警告 |
| `-Xlint:serial` | 缺少 serialVersionUID 警告 |

### 2.4 注解处理

| 选项 | 描述 |
|------|------|
| `-processor <class1>[,<class2>...]` | 指定注解处理器 |
| `-proc:none` | 禁用注解处理 |
| `-proc:only` | 仅运行注解处理 |

### 2.5 模块相关 (JDK9+)

| 选项 | 描述 |
|------|------|
| `--module <module>` | 编译指定模块 |
| `-d <directory>` | 模块输出目录 |
| `--module-path <path>` | 模块路径 |
| `--add-modules <modules>` | 添加根模块 |

### 2.6 示例

```bash
# 基本编译
javac MyApp.java

# 指定输出目录
javac -d ./target/classes MyApp.java

# 指定源和目标版本
javac -source 17 -target 17 MyApp.java

# 启用所有警告
javac -Xlint:all -Werror MyApp.java

# 编译多个文件
javac -d ./classes *.java

# 使用模块
javac --module-path libs -d out src/*/java/module-info.java $(find src -name "*.java")
```

## 3. jar - JAR 包管理工具

### 3.1 语法

```bash
jar [options] [jarfile] [manifest] -C directory files
```

### 3.2 操作选项

| 选项 | 描述 |
|------|------|
| `c` | 创建新的 JAR 文件 |
| `x` | 提取 JAR 文件内容 |
| `t` | 列出 JAR 文件内容 |
| `u` | 更新现有 JAR 文件 |
| `i` | 生成索引信息 |

### 3.3 功能选项

| 选项 | 描述 |
|------|------|
| `v` | 详细输出 |
| `f <jarfile>` | 指定 JAR 文件名 |
| `m <manifest>` | 包含清单文件 |
| `0` | 仅存储，不压缩 |
| `M` | 不创建清单文件 |
| `e <classname>` | 设置入口类 |
| `C <directory>` | 更改到指定目录 |

### 3.4 示例

```bash
# 创建 JAR
jar cf myapp.jar -C classes .

# 创建带清单的 JAR
jar cfm myapp.jar manifest.txt -C classes .

# 设置入口类
jar cfe myapp.jar com.example.Main -C classes .

# 提取 JAR
jar xf myapp.jar

# 列出内容
jar tf myapp.jar

# 更新 JAR
jar uf myapp.jar newfile.txt

# 运行 JAR
java -jar myapp.jar
```

## 4. javap - Class 文件反汇编器

### 4.1 语法

```bash
javap [options] class_file...
```

### 4.2 选项

| 选项 | 描述 |
|------|------|
| `-c` | 反汇编字节码 |
| `-d <directory>` | 指定输出目录 |
| `-p` 或 `-private` | 显示所有成员 (包括 private) |
| `-protected` | 显示 protected/public 成员 |
| `-public` | 仅显示 public 成员 |
| `-s` | 打印内部类型签名 |
| `-sysinfo` | 显示系统信息 |
| `-constants` | 显示 static final 常量 |
| `-verbose` 或 `-v` | 打印详细信息 |

### 4.3 示例

```bash
# 基本反编译
javap MyClass

# 显示详细信息
javap -verbose MyClass

# 反汇编字节码
javap -c MyClass

# 显示所有成员
javap -p -v MyClass
```

## 5. jps - JVM 进程状态工具

### 5.1 语法

```bash
jps [options] [hostid]
```

### 5.2 选项

| 选项 | 描述 |
|------|------|
| `-q` | 仅显示进程 ID |
| `-m` | 显示传递给 main 方法的参数 |
| `-l` | 显示应用程序入口 (full class name) |
| `-v` | 显示 JVM 参数 |
| `-V` | 隐藏传递给 JVM 参数 |

### 5.3 示例

```bash
# 列出所有 Java 进程
jps

# 显示详细信息
jps -lvm

# 查看特定主机 (需要 jstatd)
jps 192.168.1.100
```

## 6. jstat - JVM 统计监控工具

### 6.1 语法

```bash
jstat [options] <vmid> [interval] [count]
```

### 6.2 选项

| 选项 | 描述 |
|------|------|
| `-class` | 类加载统计 |
| `-compiler` | JIT 编译统计 |
| `-gc` | GC 堆统计 |
| `-gccapacity` | GC 容量统计 |
| `-gcutil` | GC 使用率统计 |
| `-gccause` | GC 原因统计 |
| `-gcnew` | 年轻代 GC 统计 |
| `-gcold` | 老年代 GC 统计 |
| `-printcompilation` | JIT 编译方法统计 |

### 6.3 示例

```bash
# 类加载统计
jstat -class <pid>

# GC 使用率 (每1秒打印一次，共10次)
jstat -gcutil <pid> 1000 10

# GC 容量统计
jstat -gccapacity <pid>

# JIT 编译统计
jstat -compiler <pid>
```

## 7. jmap - 内存映射分析工具

### 7.1 语法

```bash
jmap [options] <pid>
jmap [options] <executable core>
jmap [options] [server-id@]<remote server IP>
```

### 7.2 选项

| 选项 | 描述 |
|------|------|
| `-heap` | 显示堆配置和使用情况 |
| `-histo[:live]` | 显示对象统计 (可用 -live 只统计活跃对象) |
| `-clstats` | 显示类加载器统计 |
| `-finalizerinfo` | 显示待终结对象信息 |
| `-dump:<options>` | 生成堆转储 |

### 7.3 dump 选项

| 选项 | 描述 |
|------|------|
| `format=b` | 二进制格式 |
| `file=<filename>` | 输出文件名 |
| `live` | 仅转储活跃对象 |

### 7.4 示例

```bash
# 堆信息
jmap -heap <pid>

# 对象直方图
jmap -histo <pid>

# 活跃对象直方图
jmap -histo:live <pid>

# 堆转储
jmap -dump:format=b,file=heap.bin <pid>

# 转储活跃堆
jmap -dump:live,format=b,file=live_heap.bin <pid>
```

## 8. jstack - 线程栈 dump 工具

### 8.1 语法

```bash
jstack [options] <pid>
```

### 8.2 选项

| 选项 | 描述 |
|------|------|
| `-F` | 强制生成 dump (用于挂起进程) |
| `-l` | 显示锁的附加信息 |
| `-m` | 混合模式 (包含 native 栈) |

### 8.3 示例

```bash
# 基本线程 dump
jstack <pid>

# 带锁信息的 dump
jstack -l <pid>

# 强制 dump (进程无响应时)
jstack -F <pid>

# 混合模式
jstack -m <pid>
```

## 9. jcmd - 诊断命令工具

### 9.1 语法

```bash
jcmd <pid> <command> [arguments]
```

### 9.2 可用命令

| 命令 | 描述 |
|------|------|
| `VM.native_memory` | 原生内存统计 |
| `VM.classloader_stats` | 类加载器统计 |
| `GC.class_histogram` | 堆对象直方图 |
| `GC.heap_dump` | 堆转储 |
| `GC.run_finalization` | 运行终结 |
| `VM.uptime` | JVM 运行时间 |
| `VM.system_properties` | 系统属性 |
| `VM.version` | JVM 版本 |
| `Thread.print` | 打印线程栈 |
| `VM.flags` | VM 标志 |

### 9.3 示例

```bash
# 列出所有命令
jcmd <pid> help

# 打印线程
jcmd <pid> Thread.print

# 堆转储
jcmd <pid> GC.heap_dump filename=heap.bin

# 运行时间
jcmd <pid> VM.uptime
```

## 10. jdb - Java 调试器

### 10.1 语法

```bash
jdb [options] <class>
jdb [options] -attach <address>
```

### 10.2 基本命令

| 命令 | 描述 |
|------|------|
| `run` | 开始执行 |
| `cont` | 继续执行 |
| `stop in <class>.<method>` | 设置断点 |
| `stop at <class>:<line>` | 在行设置断点 |
| `clear` | 列出断点 |
| `delete <breakpoint>` | 删除断点 |
| `step` | 单步执行 |
| `next` | 单步跳过 |
| `print <expr>` | 打印变量 |
| `locals` | 打印局部变量 |
| `where` | 打印线程栈 |
| `threads` | 列出线程 |
| `quit` | 退出 |

### 10.3 示例

```bash
# 调试应用
jdb MyClass

# 连接已运行的进程
jdb -attach 5005

# 设置断点并运行
jdb MyClass
> stop in MyClass.main
> run
> print args
> cont
```

## 11. jrunscript - 脚本 shell

### 11.1 语法

```bash
jrunscript [options] [scripts...]
```

### 11.2 选项

| 选项 | 描述 |
|------|------|
| `-cp <classpath>` | 类路径 |
| `-f <scriptfile>` | 执行脚本文件 |
| `-e <script>` | 执行内联脚本 |
| `-l <language>` | 指定脚本语言 |
| `-交互` | 交互模式 |

### 11.3 示例

```bash
# 交互模式
jrunscript -l js

# 执行 JavaScript
jrunscript -l js -e "print('Hello from JavaScript')"

# 执行 Nashorn 脚本
jrunscript myscript.js
```

## 12. 兼容性说明

FerrousJDK 工具链与 OpenJDK 保持 100% 参数兼容，但可能存在以下差异：

| 组件 | 状态 | 说明 |
|------|------|------|
| java | ✓ 完全兼容 | 所有标准选项支持 |
| javac | ✓ 完全兼容 | JDK8-JDK21 语法支持 |
| jar | ✓ 完全兼容 | 所有标准操作支持 |
| javap | ✓ 完全兼容 | 字节码反汇编 |
| jps | ✓ 完全兼容 | 进程列表 |
| jstat | ✓ 完全兼容 | 统计信息 |
| jmap | ✓ 完全兼容 | 内存分析 |
| jstack | ✓ 完全兼容 | 线程栈 |
| jcmd | ✓ 完全兼容 | 诊断命令 |
| jdb | ✓ 完全兼容 | 调试器 |
| jrunscript | ✓ 完全兼容 | 脚本 shell |
