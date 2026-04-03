# 构建系统架构

本文档描述 FerrousJDK 构建系统的架构设计，包括构建流程、产物对齐策略、跨平台编译和打包机制。

## 1. 构建系统概述

### 1.1 设计目标

FerrousJDK 构建系统 (`ferrous-build`) 承担以下核心职责：

- **1:1 产物对齐**：生成的 JDK 产物与 OpenJDK 目录结构完全一致
- **跨平台构建**：支持 Linux、macOS、Windows 三大主流平台
- **多架构支持**：支持 x86_64 和 ARM64 架构
- **灵活配置**：支持多种 LTS 版本、GC 策略、JIT 后端的组合构建

### 1.2 构建流程概览

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Build Pipeline                                │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐              │
│  │ Config      │───►│ Build       │───►│ Package     │              │
│  │ Parse       │    │ Compile     │    │ Assemble    │              │
│  └─────────────┘    └─────────────┘    └─────────────┘              │
│        │                  │                  │                      │
│        ▼                  ▼                  ▼                      │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐              │
│  │  Feature    │    │  Rust Code │    │  Resource   │              │
│  │  Resolve    │    │  Compile   │    │  Copy       │              │
│  └─────────────┘    └─────────────┘    └─────────────┘              │
│                            │                  │                      │
│                            ▼                  ▼                      │
│                      ┌─────────────┐    ┌─────────────┐              │
│                      │  Java Code  │    │  Validation │              │
│                      │  Compile    │    │  Checksum   │              │
│                      └─────────────┘    └─────────────┘              │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## 2. 构建配置

### 2.1 命令行参数

```rust
pub struct BuildConfig {
    pub lts_version: LTSVersion,       // jdk8, jdk17, jdk21
    pub platform: Platform,            // linux-x86_64, macos-arm64, windows-x86_64
    pub output_dir: PathBuf,
    pub features: Vec<String>,          // gc策略、JIT后端等
    pub build_type: BuildType,          // debug, release
    pub with_tools: bool,               // 是否构建工具链
    pub with_tests: bool,               // 是否包含测试
}
```

命令行接口：

```bash
# 基本构建
cargo run -p ferrous-build -- --lts jdk17 --platform linux-x86_64 --output ./dist

# 完整构建（包含所有工具）
cargo run -p ferrous-build -- --lts jdk21 --platform macos-arm64 --output ./dist --full

# 自定义特性
cargo run -p ferrous-build -- --lts jdk8 --gc parallel --jit llvm --output ./dist
```

### 2.2 配置文件

支持 TOML 配置文件：

```toml
# ferrous-build.toml
[lts]
version = "jdk17"

[platform]
os = "linux"
arch = "x86_64"

[features]
gc = ["parallel", "zgc"]
jit = ["cranelift"]
tools = true

[build]
jobs = 8
debug = false

[output]
dir = "./target/dist"
checksum = true
```

## 3. 产物目录结构

### 3.1 OpenJDK 对齐结构

FerrousJDK 构建产物严格遵循 OpenJDK 目录结构：

```
ferrous-jdk-{version}/
│
├── bin/                              # 可执行文件
│   ├── java                          # JVM 启动器
│   ├── javac                         # Java 编译器
│   ├── jar                           # JAR 管理工具
│   ├── javap                         # Class 文件分析器
│   ├── jdb                           # Java 调试器
│   ├── jps                           # Java 进程状态
│   ├── jstack                        # 线程栈 dump
│   ├── jstat                         # 统计监控
│   ├── jmap                          # 内存映射分析
│   ├── jrunscript                    # 脚本 shell
│   └── ...
│
├── conf/                             # 配置文件
│   ├── net.properties
│   ├── logging.properties
│   ├── sound.properties
│   └── ...
│
├── include/                          # C 头文件
│   ├── jni.h                         # JNI 接口
│   ├── jni_md.h                      # JNI 机器相关定义
│   ├── jvmti.h                       # JVM TI 接口
│   ├── jvmti_md.h                    # JVM TI 机器相关定义
│   └── ...
│
├── jre/                              # JRE 环境 (JDK8)
│   └── ...
│
├── lib/                              # 运行时库
│   ├── jvm.cfg                       # JVM 配置
│   ├── classlist                     # 类列表
│   ├── default.jfc                   # Flight Recorder 配置
│   ├── jvm.hprof.txt                 # HPROF 配置
│   ├── psfont.properties.ja
│   ├── pslogicjfontmap.properties
│   ├── resources.jar                 # 资源文件
│   │
│   ├── server/                       # Server JVM
│   │   ├── Xusage.txt
│   │   └── {libjvm.so|jvm.dll}       # JVM 动态库
│   │
│   ├── client/                       # Client JVM (JDK8)
│   │   └── {libjvm.so|jvm.dll}
│   │
│   └── jmods/                        # JMOD 文件 (JDK9+)
│       ├── java.base.jmod
│       ├── java.logging.jmod
│       └── ...
│
├── release                           # 版本信息文件
├── legal/                            # 法律声明
│   ├── java.base/
│   └── ...
└── ...
```

### 3.2 release 文件格式

```properties
IMPLEMENTOR="Ferrous Systems"
IMPLEMENTOR_VERSION="FerrousJDK"
JAVA_VERSION="17.0.2"
JAVA_VERSION_DATE="2024-01-16"
LIBC="default"
MODULES="java.base java.logging java.sql java.naming java.desktop java.instrument java.management java.security.sasl java.prefs java.xml java.crypto.cryptoki java.crypto.ec java.sql.rowset java.scripting java.management.rmi java.rmi"
OS_ARCH="x86_64"
OS_NAME="Linux"
SOURCE=" .:git-tag:v17.0.2"
```

## 4. 编译流程

### 4.1 Rust 代码编译

```rust
pub struct RustCompiler {
    workspace_root: PathBuf,
    target_dir: PathBuf,
}

impl RustCompiler {
    pub fn compile(&self, config: &BuildConfig) -> Result<CompilationArtifact> {
        // 1. 解析特性
        let features = self.resolve_features(&config.features)?;
        
        // 2. 构建 cargo 命令
        let mut cmd = Command::new("cargo");
        cmd.arg("build");
        
        if config.build_type == BuildType::Release {
            cmd.arg("--release");
        }
        
        // 3. 添加特性参数
        for feature in &features {
            cmd.arg("--features").arg(feature);
        }
        
        // 4. 设置目标平台
        let target = self.platform_to_rust_target(&config.platform);
        cmd.arg("--target").arg(target);
        
        // 5. 执行编译
        let output = cmd.output()
            .context("cargo build failed")?;
        
        // 6. 收集产物
        self.collect_artifacts(&output)
    }
}
```

### 4.2 Java 代码编译

标准库中的 Java 代码（如果存在）需要编译：

```rust
pub struct JavaCompiler {
    javac_path: PathBuf,
}

impl JavaCompiler {
    pub fn compile(&self, sources: &[PathBuf], output_dir: &Path) -> Result<()> {
        let mut cmd = Command::new(&self.javac_path);
        cmd.arg("-d").arg(output_dir);
        cmd.arg("-source").arg(self.source_version());
        cmd.arg("-target").arg(self.target_version());
        cmd.arg("-encoding").arg("UTF-8");
        
        // 添加所有源文件
        cmd.args(sources);
        
        let output = cmd.output()
            .context("javac compilation failed")?;
        
        if !output.status.success() {
            return Err(CompilationError::new(String::from_utf8_lossy(&output.stderr)));
        }
        
        Ok(())
    }
}
```

### 4.3 资源复制

```rust
pub struct ResourceManager {
    source_dir: PathBuf,
    target_dir: PathBuf,
}

impl ResourceManager {
    pub fn copy_resources(&self, config: &BuildConfig) -> Result<()> {
        // 复制动态库
        self.copy_dynamic_libraries()?;
        
        // 复制配置文件
        self.copy_config_files()?;
        
        // 复制法律声明
        self.copy_legal_files()?;
        
        // 复制字符集数据
        self.copy_charsets()?;
        
        // 复制 JMOD 文件 (JDK9+)
        if config.lts_version >= LTSVersion::JDK9 {
            self.copy_jmod_files()?;
        }
        
        Ok(())
    }
    
    fn copy_dynamic_libraries(&self) -> Result<()> {
        let lib_dir = self.target_dir.join("lib");
        let server_dir = lib_dir.join("server");
        
        // 复制 libjvm.so / jvm.dll / libjvm.dylib
        self.copy_file(
            self.target_dir.join("target/release/libjvm.so"),
            server_dir.join("libjvm.so"),
        )?;
        
        Ok(())
    }
}
```

## 5. 跨平台构建

### 5.1 平台映射

| Platform | Rust Target | JVM Lib Suffix |
|----------|-------------|----------------|
| Linux x86_64 | x86_64-unknown-linux-gnu | .so |
| Linux ARM64 | aarch64-unknown-linux-gnu | .so |
| macOS x86_64 | x86_64-apple-darwin | .dylib |
| macOS ARM64 | aarch64-apple-darwin | .dylib |
| Windows x86_64 | x86_64-pc-windows-msvc | .dll |

### 5.2 交叉编译支持

```rust
pub struct CrossCompiler {
    target: Platform,
}

impl CrossCompiler {
    pub fn setup_for(&self) -> Result<()> {
        match self.target {
            Platform::LinuxARM64 => {
                // 安装 ARM64 cross 工具链
                self.install_cross_toolchain("aarch64-unknown-linux-gnu")?;
            }
            Platform::WindowsX64 => {
                // Windows 交叉编译需要 mingw 环境
                self.setup_mingw()?;
            }
            _ => {}
        }
        
        Ok(())
    }
}
```

### 5.3 Docker 构建支持

```dockerfile
# Dockerfile.cross-build
FROM rust:1.75 as builder

# 安装交叉编译工具链
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

# 复制源代码
COPY . /build
WORKDIR /build

# 构建 ARM64 版本
RUN cargo build --release --target aarch64-unknown-linux-gnu

# 产物阶段
FROM ubuntu:22.04
COPY --from=builder /build/target/aarch64-unknown-linux-gnu/release/libjvm.so /lib/
```

## 6. 产物校验

### 6.1 文件完整性检查

```rust
pub struct ChecksumValidator;

impl ChecksumValidator {
    pub fn compute_sha256(path: &Path) -> Result<String> {
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];
        
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        
        Ok(hex::encode(hasher.finalize()))
    }
    
    pub fn validate(&self, artifact_dir: &Path) -> Result<ValidationReport> {
        let mut report = ValidationReport::new();
        
        // 验证关键文件存在
        for file in REQUIRED_FILES {
            let path = artifact_dir.join(file);
            if !path.exists() {
                report.add_error(format!("Missing required file: {}", file));
            }
        }
        
        // 验证文件大小合理
        for file in ALL_LIBRARIES {
            let path = artifact_dir.join(file);
            let size = path.metadata()?.len();
            if size < MIN_LIBRARY_SIZE {
                report.add_error(format!("File too small: {} ({} bytes)", file, size));
            }
        }
        
        Ok(report)
    }
}
```

### 6.2 版本一致性检查

```rust
pub fn validate_version_consistency(product_dir: &Path) -> Result<()> {
    // 读取 release 文件
    let release_content = fs::read_to_string(product_dir.join("release"))?;
    let release = parse_release_file(&release_content)?;
    
    // 验证 JAVA_VERSION
    let expected_version = env!("CARGO_PKG_VERSION");
    if release.get("JAVA_VERSION") != Some(expected_version) {
        return Err(VersionMismatch(format!(
            "Version mismatch: {} vs {}",
            release.get("JAVA_VERSION"),
            expected_version
        )));
    }
    
    // 验证 JVM 库版本
    let libjvm = load_jvm_library(product_dir)?;
    let lib_version = libjvm.get_version()?;
    if lib_version.major() != release.major_version() {
        return Err(VersionMismatch("JVM library version mismatch"));
    }
    
    Ok(())
}
```

## 7. 打包与发布

### 7.1 多格式打包

```rust
pub enum PackageFormat {
    TarGz,     // Linux/ macOS
    Zip,       // Windows
    Deb,       // Debian/Ubuntu
    Rpm,       // RHEL/CentOS
    Pkg,       // macOS Installer
    Msi,       // Windows Installer
}

pub struct PackageBuilder {
    format: PackageFormat,
    version: String,
    lts: LTSVersion,
}

impl PackageBuilder {
    pub fn build(&self, artifact_dir: &Path, output_dir: &Path) -> Result<PathBuf> {
        let package_name = format!(
            "ferrous-jdk-{}-{}",
            self.version,
            self.lts
        );
        
        match self.format {
            PackageFormat::TarGz => self.create_tarball(artifact_dir, output_dir, &package_name),
            PackageFormat::Zip => self.create_zip(artifact_dir, output_dir, &package_name),
            PackageFormat::Deb => self.create_deb(artifact_dir, output_dir, &package_name),
            // ...
        }
    }
}
```

### 7.2 发布清单

```
dist/
├── ferrous-jdk-17.0.2-jdk17-linux-x86_64.tar.gz
├── ferrous-jdk-17.0.2-jdk17-linux-aarch64.tar.gz
├── ferrous-jdk-17.0.2-jdk17-macos-x86_64.tar.gz
├── ferrous-jdk-17.0.2-jdk17-macos-aarch64.tar.gz
├── ferrous-jdk-17.0.2-jdk17-windows-x86_64.zip
└── SHA256SUMS
```

## 8. 构建脚本集成

### 8.1 Makefile 支持

```makefile
# Makefile
.PHONY: build test package clean

JDK_VERSION ?= 17
PLATFORM ?= linux-x86_64
OUTPUT_DIR ?= ./dist

build:
	cargo build --release
	cargo run -p ferrous-build -- \
		--lts jdk$(JDK_VERSION) \
		--platform $(PLATFORM) \
		--output $(OUTPUT_DIR)

test: build
	$(OUTPUT_DIR)/bin/java -version
	$(OUTPUT_DIR)/bin/java -Xshare:dump

package: build
	cargo run -p ferrous-build -- \
		--lts jdk$(JDK_VERSION) \
		--platform $(PLATFORM) \
		--output $(OUTPUT_DIR) \
		--package tar.gz

clean:
	rm -rf $(OUTPUT_DIR)
	cargo clean
```

### 8.2 CI/CD 集成

```yaml
# .github/workflows/build.yml
name: Build and Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        platform: [linux-x86_64, linux-aarch64, macos-x86_64, macos-aarch64, windows-x86_64]
        jdk: [8, 17, 21]
    
    runs-on: ${{ matrix.platform }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
        with:
          targets: ${{ matrix.platform }}
      
      - name: Build
        run: |
          cargo build --release --features jdk${{ matrix.jdk }}
      
      - name: Package
        run: |
          cargo run -p ferrous-build -- \
            --lts jdk${{ matrix.jdk }} \
            --platform ${{ matrix.platform }} \
            --output ./dist
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: ferrous-jdk-${{ matrix.jdk }}-${{ matrix.platform }}
          path: ./dist/*.tar.gz
```

## 9. 相关文档

- [整体架构](./overall-arch.md) - FerrousJDK 完整架构概览
- [JVM 核心架构](./jvm-arch.md) - 类加载、运行时、GC 等
- [标准库架构](./stdlib-arch.md) - Java 标准库实现策略
- [开发环境搭建](../dev-guide/env-setup.md) - 本地构建环境配置
