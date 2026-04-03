# 本地开发环境搭建

本文档帮助开发者搭建 FerrousJDK 的本地开发环境。

## 1. 系统要求

### 1.1 硬件要求

| 组件 | 最低要求 | 推荐配置 |
|------|----------|----------|
| CPU | 4 核 | 8 核或以上 |
| 内存 | 8 GB | 16 GB 或以上 |
| 磁盘 | 10 GB | 20 GB 或以上 SSD |
| 网络 | 稳定 | - |

### 1.2 操作系统

| 平台 | 支持版本 | 构建支持 |
|------|----------|----------|
| Linux | Ubuntu 20.04+, Debian 11+ | ✓ 原生 |
| macOS | macOS 11+ | ✓ 原生 |
| Windows | Windows 10/11, WSL2 | ✓ 原生 |

## 2. 基础工具安装

### 2.1 Rust 工具链

#### Linux / macOS

```bash
# 安装 Rust (如果尚未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Rust nightly (当前开发需要)
rustup install nightly

# 设置 nightly 为默认
rustup default nightly

# 验证安装
rustc --version
cargo --version
```

#### Windows

```powershell
# 使用 rustup 安装
winget install Rustlang.Rustup

# 或从 https://rustup.rs 下载安装

# 验证安装
rustc --version
cargo --version
```

### 2.2 必要构建工具

#### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    cmake \
    git \
    curl \
    wget \
    unzip \
    pkg-config \
    libssl-dev \
    clang \
    llvm
```

#### Linux (Fedora/RHEL)

```bash
sudo dnf install -y \
    gcc \
    gcc-c++ \
    make \
    cmake \
    git \
    clang \
    llvm \
    openssl-devel
```

#### macOS

```bash
# 安装 Xcode Command Line Tools
xcode-select --install

# 安装 Homebrew (如果尚未安装)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 安装必要工具
brew install cmake git llvm
```

#### Windows

```powershell
# 安装 Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools

# 或安装完整 Visual Studio 2022 Community
winget install Microsoft.VisualStudio.2022.Community
```

### 2.3 Git 配置

```bash
git config --global user.name "Your Name"
git config --global user.email "you@example.com"
git config --global core.autocrlf input  # Linux/macOS
git config --global core.autocrlf true   # Windows
```

## 3. 获取源码

### 3.1 克隆仓库

```bash
# 克隆主仓库
git clone https://github.com/uv301/ferrous_jdk.git
cd ferrous-jdk

# 查看所有分支
git branch -a

# 检出开发分支
git checkout dev
```

### 3.2 初始化子模块

```bash
# 初始化 git 子模块
git submodule update --init --recursive
```

## 4. 开发工具配置

### 4.1 IDE 设置

#### VS Code (推荐)

```bash
# 安装扩展
code --install-extension rust-lang.rust-analyzer
code --install-extension ms-vscode.cpptools
code --install-extension vadimcn.vscode-lldb
```

推荐配置 `settings.json`:

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": ["jdk17", "cranelift", "debug"],
  "editor.formatOnSave": true,
  "editor.rulers": [100],
  "files.insertFinalNewline": true,
  "files.trimTrailingWhitespace": true
}
```

#### IntelliJ IDEA / RustRover

1. File → Project Structure → Modules → Add → Cargo
2. 设置 Cargo.toml 位置
3. 启用 Rust 插件

#### CLion

1. 安装 Rust 插件
2. File → Project Structure → Modules → Add → Cargo
3. 配置 Cargo.toml

### 4.2 代码格式化

```bash
# 检查格式
cargo fmt --check

# 格式化代码
cargo fmt
```

### 4.3 代码检查

```bash
# 运行 clippy
cargo clippy

# 修复 clippy 警告
cargo clippy --fix
```

## 5. 构建 FerrousJDK

### 5.1 开发构建

```bash
# 开发构建 (debug 模式)
cargo build

# 完整开发构建 (JDK17 + Cranelift)
cargo build --features jdk17,cranelift,debug
```

### 5.2 生产构建

```bash
# 生产构建
cargo build --release --features jdk17,cranelift

# 指定多个特性
cargo build --release --features jdk17,serial-gc,cranelift
```

### 5.3 生成 JDK 产物

```bash
# 构建 JDK17 产物
cargo run -p ferrous-build -- --lts jdk17 --platform linux-x86_64 --output ./dist

# 完整构建 (包含工具)
cargo run -p ferrous-build -- --lts jdk17 --platform linux-x86_64 --output ./dist --full
```

### 5.4 构建产物验证

```bash
# 检查产物目录
ls -la ./dist/

# 验证 java 版本
./dist/bin/java -version
```

## 6. 运行测试

### 6.1 单元测试

```bash
# 运行所有测试
cargo test

# 运行特定 crate 的测试
cargo test -p ferrous-core

# 运行特定测试
cargo test test_class_loading --features jdk17
```

### 6.2 集成测试

```bash
# 运行集成测试
cargo test --test integration

# 运行特定集成测试
cargo test --test integration -- --nocapture
```

### 6.3 性能测试

```bash
# 安装 criterion
cargo install cargo-criterion

# 运行性能基准测试
cargo criterion --features jdk17

# 查看基准测试报告
open target/criterion/report/index.html
```

### 6.4 代码覆盖率

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 运行覆盖率
cargo tarpaulin --features jdk17 -o html

# 查看报告
open target/tarpaulin-report.html
```

## 7. jtreg 测试

jtreg 是 OpenJDK 官方使用的回归测试框架。

### 7.1 安装 jtreg

```bash
# 下载 jtreg
wget https://builds.openjdk.org/jtreg/7.3.1/jtreg-7.3.1.zip
unzip jtreg-7.3.1.zip -d /opt/

# 设置环境变量
export JTREG_HOME=/opt/jtreg
export PATH=$JTREG_HOME/bin:$PATH
```

### 7.2 运行 jtreg 测试

```bash
# 基本运行
jtreg -jdk:$JAVA_HOME -dir:tests/jtreg test/

# 运行特定测试
jtreg -jdk:$JAVA_HOME -dir:tests/jtreg tests/api/java/lang/

# 生成 HTML 报告
jtreg -jdk:$JAVA_HOME -dir:tests/jtreg -reportDir:./jtreg-report test/
```

## 8. 调试配置

### 8.1 LLDB 调试

#### Linux / macOS

```bash
# 编译 debug 版本
cargo build --features jdk17,debug

# 启动 LLDB
lldb ./target/debug/java

# 设置断点
b Java_java_lang_Object_hashCode

# 运行
r

# 查看变量
frame variable
```

#### Windows

```powershell
# 使用 Visual Studio 或 WinDbg
# 参考 LLDB 配置
```

### 8.2 GDB 调试 (Linux)

```bash
# 安装 Rust 支持
rustup component add rust-src

# 编译 debug 版本
cargo build --features jdk17,debug

# 启动 GDB
gdb ./target/debug/java

# 加载 Rust 符号
(gdb) set rust-pretty-printers on
(gdb) set load-split-dwarf on

# 设置断点
(gdb) b Java_java_lang_Object_hashCode

# 运行
(gdb) run
```

### 8.3 Visual Studio Code 调试

创建 `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug java",
      "cargo": {
        "args": "build --features jdk17,debug",
        "env": {}
      },
      "program": "${workspaceFolder}/target/debug/java",
      "args": "-jar ${workspaceFolder}/tests/hello.jar",
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

### 8.4 远程调试

```bash
# 启动 JVM 并开启调试端口
java -agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=5005 -jar myapp.jar

# 连接调试器
# VS Code: 使用 "Java: Attach" 配置
# IntelliJ: Run → Attach to Process
```

## 9. 高级配置

### 9.1 使用 LLVM 后端

```bash
# 安装 LLVM
brew install llvm  # macOS
# 或 apt install llvm  # Ubuntu

# 构建使用 LLVM
cargo build --features jdk17,llvm,debug
```

### 9.2 并行构建

```bash
# 使用所有 CPU 核心
export CARGO_BUILD_JOBS=8

# 或直接指定
cargo build -j 8
```

### 9.3 构建缓存

```bash
# 使用 sccache 加速构建
cargo install sccache

export RUSTC_WRAPPER=sccache
export SCCACHE_CACHE_SIZE=20G

cargo build --features jdk17
```

## 10. 常见问题

### 10.1 编译错误：missing git

```bash
sudo apt install git
```

### 10.2 编译错误：LLVM not found

```bash
# Linux
sudo apt install llvm-dev libclang-dev clang

# macOS
brew install llvm
export LIBCLANG_PATH=$(brew --prefix llvm)/lib
```

### 10.3 编译错误：Windows msvc

```powershell
# 安装 Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
```

### 10.4 内存不足

```bash
# 减少并行构建数量
cargo build -j 2

# 或者增大 swap
sudo fallocate -l 8G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

### 10.5 验证 Rust 工具链

```bash
# 检查工具链
rustc --version
cargo --version
rustup show
rustfmt --version

# 如果有问题，尝试更新
rustup update
```

## 11. 下一步

- [模块开发指南](./module-guide.md) - 了解 crate 结构和贡献代码
- [native 方法重写指南](./native-rewrite.md) - 标准和库 native 方法重写
- [JDK17 适配指南](./jdk17-adapt.md) - 版本差异处理
