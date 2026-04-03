# 快速开始指南

本文档帮助您快速安装和配置 FerrousJDK，并运行您的第一个 Java 程序。

## 1. 系统要求

### 1.1 硬件要求

| 组件 | 最低要求 | 推荐配置 |
|------|----------|----------|
| CPU | 2 核 | 4 核或以上 |
| 内存 | 4 GB | 8 GB 或以上 |
| 磁盘 | 500 MB | 1 GB 或以上 |

### 1.2 软件要求

| 平台 | 支持版本 |
|------|----------|
| Linux | Ubuntu 20.04+, Debian 11+, CentOS 8+ |
| macOS | macOS 11+ (Big Sur 或更新) |
| Windows | Windows 10/11, Windows Server 2019+ |

## 2. 安装 FerrousJDK

### 2.1 从发布包安装

#### Linux / macOS

```bash
# 下载最新发布版本
wget https://github.com/uv301/ferrous_jdk/releases/latest/ferrous-jdk-{version}-linux-x86_64.tar.gz

# 解压到安装目录
sudo tar -xzf ferrous-jdk-{version}-linux-x86_64.tar.gz -C /opt/

# 设置环境变量
export JAVA_HOME=/opt/ferrous-jdk-{version}
export PATH=$JAVA_HOME/bin:$PATH

# 验证安装
java -version
```

#### Windows

```powershell
# 使用 winget 安装
winget install FerrousJDK.FerroJDK

# 或者手动解压到 C:\Program Files\
# 然后通过系统设置配置环境变量
```

### 2.2 从源码构建

详细构建步骤请参考 [开发环境搭建](../dev-guide/env-setup.md)。

```bash
# 克隆仓库
git clone https://github.com/uv301/ferrous_jdk.git
cd ferrous-jdk

# 构建 JDK17 版本
cargo run -p ferrous-build -- --lts jdk17 --platform linux-x86_64 --output ./dist

# 设置环境变量
export JAVA_HOME=$(pwd)/dist/ferrous-jdk-{version}
export PATH=$JAVA_HOME/bin:$PATH
```

## 3. 环境配置

### 3.1 JAVA_HOME 设置

#### Linux / macOS (bash)

```bash
# 编辑 ~/.bashrc 或 ~/.zshrc
echo 'export JAVA_HOME=/opt/ferrous-jdk' >> ~/.bashrc
echo 'export PATH=$JAVA_HOME/bin:$PATH' >> ~/.bashrc

# 使配置生效
source ~/.bashrc
```

#### Windows

```powershell
# 通过系统属性设置
# 控制面板 -> 系统 -> 高级系统设置 -> 环境变量

# 或者使用 PowerShell (永久设置)
[System.Environment]::SetEnvironmentVariable("JAVA_HOME", "C:\Program Files\FerrousJDK", "User")
[System.Environment]::SetEnvironmentVariable("PATH", "$env:JAVA_HOME\bin;$env:PATH", "User")
```

### 3.2 验证配置

```bash
# 检查 Java 版本
java -version

# 预期输出示例
# openjdk version "17.0.2" 2024-01-16
# FerrousJDK Runtime Environment (build 17.0.2)
# FerrousJDK 64-Bit Server VM (build, interpreted mode)

# 检查 javac 版本
javac -version

# 检查 JAVA_HOME
echo $JAVA_HOME
```

## 4. 运行 Hello World

### 4.1 创建 Java 源文件

```java
// HelloWorld.java
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, FerrousJDK!");
        
        // 展示一些基本功能
        System.out.println("Java Version: " + System.getProperty("java.version"));
        System.out.println("JVM Name: " + System.getProperty("java.vm.name"));
        System.out.println("OS: " + System.getProperty("os.name"));
        
        // 测试标准库
        var list = java.util.List.of("Apple", "Banana", "Cherry");
        System.out.println("Fruits: " + list);
    }
}
```

### 4.2 编译和运行

```bash
# 编译
javac HelloWorld.java

# 运行
java HelloWorld

# 预期输出
# Hello, FerrousJDK!
# Java Version: 17.0.2
# JVM Name: FerrousJDK 64-Bit Server VM
# OS: Linux
# Fruits: [Apple, Banana, Cherry]
```

## 5. 基本配置

### 5.1 JVM 选项

```bash
# 设置堆内存大小
java -Xmx2g -Xms512m HelloWorld

# 设置 GC 类型 (JDK17+)
java -XX:+UseZGC HelloWorld
java -XX:+UseParallelGC HelloWorld

# 启用类数据共享
java -Xshare:on HelloWorld

# 打印 GC 信息
java -XX:+PrintGCDetails -XX:+PrintGCTimeStamps HelloWorld

# 列出所有可用的 XX 选项
java -XX:+PrintFlagsFinal -version
```

### 5.2 常用 JVM 选项参考

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `-Xmx<size>` | 最大堆内存 | 物理内存的 1/4 |
| `-Xms<size>` | 初始堆内存 | 物理内存的 1/64 |
| `-Xss<size>` | 线程栈大小 | 1 MB |
| `-XX:+UseSerialGC` | 使用串行 GC | JDK8 默认 |
| `-XX:+UseG1GC` | 使用 G1 GC | JDK9+ 默认 |
| `-XX:+UseZGC` | 使用 ZGC | - |
| `-Xshare:on` | 启用类数据共享 | - |

### 5.3 配置文件

FerrousJDK 使用与 OpenJDK 相同的配置文件格式：

```properties
# conf/net.properties
http.proxyHost=proxy.example.com
http.proxyPort=8080

# conf/logging.properties
java.util.logging.ConsoleHandler.level=FINE
java.util.logging.SimpleFormatter.format=[%1$tF %1$tT] [%4$s] %5$s%n
```

## 6. 多版本共存

### 6.1 使用 update-alternatives (Linux)

```bash
# 添加 FerrousJDK 到 alternatives
sudo update-alternatives --install /usr/bin/java java /opt/ferrous-jdk/bin/java 100
sudo update-alternatives --install /usr/bin/javac javac /opt/ferrous-jdk/bin/javac 100

# 选择默认版本
sudo update-alternatives --config java
```

### 6.2 使用 jenv (跨平台)

```bash
# 安装 jenv
brew install jenv  # macOS
# 或 apt install jenv  # Ubuntu

# 添加 FerrousJDK
jenv add /opt/ferrous-jdk

# 列出可用版本
jenv versions

# 切换版本
jenv global ferrousjdk-17.0.2
```

## 7. 运行测试

### 7.1 运行 Java 单元测试

```bash
# 运行所有测试
java -jar junit-platform-console-standalone.jar --scan-classpath

# 运行特定测试类
java -cp .:junit.jar MyTestClass
```

### 7.2 性能基准测试

```bash
# 运行 SciMark2 基准测试
java -jar scimark2.jar

# 运行 DaCapo 基准测试
java -jar dacapo-9.12-bach.jar
```

## 8. 常见问题

### 8.1 找不到 JAVA_HOME

```bash
# 查找 java 安装位置
which java
readlink -f $(which java)

# 设置正确的 JAVA_HOME
export JAVA_HOME=$(dirname $(dirname $(readlink -f $(which java))))
```

### 8.2 内存不足

```bash
# 减少堆内存使用
java -Xmx512m -Xms256m -Xss256k MyApplication

# 使用串行 GC (内存占用更小)
java -XX:+UseSerialGC -Xmx512m MyApplication
```

### 8.3 类路径问题

```bash
# 检查类路径
java -XshowSettings:properties -version 2>&1 | grep java.class.path

# 运行带类路径的 jar
java -cp "lib/*:myapp.jar" com.example.Main
```

## 9. 下一步

- [完整命令参考](./command-ref.md) - 了解所有 JDK 工具的使用方法
- [性能调优指南](./tuning-guide.md) - 优化 FerrousJDK 性能
- [兼容性说明](./compatibility.md) - 了解与 OpenJDK 的兼容性详情
