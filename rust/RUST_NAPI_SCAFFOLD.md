# Rust + NAPI-RS 双模式脚手架项目文档

## 概述

本文档介绍一个支持**双安装模式**的 Rust + NAPI-RS 脚手架项目。该仓库既可以作为：

1. **纯 Rust CLI 工具** - 通过 `cargo install` 安装和使用
2. **npm 包** - 通过 `npm install` 安装，提供 Node.js 绑定

通过模块化和特性门控设计，实现了代码的完全解耦，用户可以根据需求选择合适的安装方式。

## 双模式设计架构

### 核心设计原则

1. **核心功能独立**: 核心脚手架逻辑不依赖任何特定运行时
2. **适配器模式**: 为不同运行时提供适配器层
3. **特性门控**: 使用 Cargo features 控制功能编译
4. **统一接口**: 无论哪种安装方式，提供相同的功能接口

### 支持的安装模式

#### 模式一：纯 Rust CLI 工具
```bash
# 通过 Cargo 安装
cargo install scaffold

# 使用
scaffold init my-project --template basic
```

#### 模式二：npm 包（Node.js 绑定）
```bash
# 通过 npm 安装
npm install -g @your-org/scaffold

# 或本地安装
npm install scaffold

# 使用
scaffold init my-project --template basic

# 或在 Node.js 代码中使用
const { Scaffold } = require('@your-org/scaffold');
```

## 项目架构

### 核心概念

1. **Cargo Workspace**: 模块化的 Rust crate 组织
2. **核心解耦**: 核心逻辑与运行时完全分离
3. **适配器层**: 为不同运行时提供适配器
4. **特性门控**: 精确控制编译的组件
5. **统一接口**: 跨平台的一致使用体验

## 目录结构

```
scaffold/
├── Cargo.toml                 # Workspace 配置
├── README.md                  # 项目说明
├── LICENSE                    # 许可证
├── package.json               # npm 包配置（用于发布）
├── index.js                   # npm 包入口
├── index.d.ts                 # TypeScript 类型定义
└── crates/                    # Rust 工作空间
    ├── bin/                   # Rust CLI 主程序入口
    │   ├── Cargo.toml
    │   └── src/
    │       └── main.rs        # CLI 入口点
    ├── core/                  # 核心库（运行时无关）
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs         # 公共 API
    │       ├── engine.rs      # 核心引擎
    │       └── template.rs    # 模板处理
    ├── adapters/              # 适配器层
    │   ├── cli/               # CLI 适配器
    │   │   ├── Cargo.toml
    │   │   └── src/
    │   │       └── lib.rs     # CLI 特定逻辑
    │   └── napi/              # NAPI 适配器
    │       ├── Cargo.toml
    │       ├── build.rs       # 构建脚本
    │       ├── index.js       # JS 入口
    │       └── src/
    │           └── lib.rs     # NAPI 绑定
    ├── common/                # 公共类型和工具
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── types.rs       # 共享类型
    │       └── constants.rs   # 常量定义
    └── utils/                 # 工具函数
        ├── Cargo.toml
        └── src/
            ├── lib.rs
            ├── fs.rs          # 文件系统操作
            └── print.rs       # 打印输出
```

## 配置文件

### 1. 根 Cargo.toml

```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.80"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
homepage = "https://github.com/yourusername/rust-napi-scaffold"
repository = "https://github.com/yourusername/rust-napi-scaffold"

[workspace.dependencies]
# CLI 相关
clap = { version = "4.0", features = ["derive"] }
dialoguer = "0.11.0"
colored = "3.0.0"

# NAPI-RS 生态系统 (可选)
napi = { version = "3", default-features = false, features = [
  "async",
  "tokio_rt",
  "serde-json",
  "anyhow",
  "napi7",
  "compat-mode",
  "type-def"
], optional = true }
napi-build = { version = "2", default-features = false, optional = true }
napi-derive = { version = "3", default-features = false, features = [
  "compat-mode",
  "type-def"
], optional = true }

[features]
default = []
# CLI 特性 - 构建 Rust CLI 工具
cli = ["dep:clap", "dep:dialoguer", "dep:colored", "adapters/cli"]
# NAPI 特性 - 构建 Node.js 绑定
napi = ["dep:napi", "dep:napi-build", "dep:napi-derive", "core/napi", "common/napi", "adapters/napi"]
# 完整特性 - 同时支持两种模式
full = ["cli", "napi"]

# 异步运行时
tokio = { version = "1", features = [
  "rt",
  "rt-multi-thread",
  "macros",
  "test-util",
  "parking_lot"
] }

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = { version = "1" }

# 模板引擎
handlebars = "6.3.0"

# 文件系统
fs_extra = "1.3"
walkdir = "2.3"

# 错误处理
anyhow = "1.0"
thiserror = "2.0"

# 并发
crossbeam = "0.8"
parking_lot = "0.12"

# 工具
uuid = { version = "1", features = ["v4", "serde"] }
lazy_static = "1.4.0"
regex = "1.0"

[profile.dev]
panic = "abort"
incremental = true
codegen-units = 16

[profile.release]
panic = "abort"
codegen-units = 1
lto = "fat"
opt-level = 3
strip = true
```

### 2. npm 包配置 (package.json)

```json
{
  "name": "@your-org/scaffold",
  "version": "0.1.0",
  "description": "A Rust + NAPI-RS scaffold generator with dual installation modes",
  "main": "index.js",
  "types": "index.d.ts",
  "bin": {
    "scaffold": "./bin/scaffold.js"
  },
  "scripts": {
    "preinstall": "napi prebuild --download",
    "prepublishOnly": "napi prebuild --upload",
    "build": "napi build --platform --release",
    "build:debug": "napi build --platform",
    "test": "node test/test.js",
    "pretest": "npm run build"
  },
  "napi": {
    "name": "scaffold-napi",
    "triples": {
      "defaults": true,
      "additional": [
        "x86_64-pc-windows-msvc",
        "i686-pc-windows-msvc",
        "x86_64-apple-darwin",
        "aarch64-apple-darwin",
        "x86_64-unknown-linux-gnu",
        "x86_64-unknown-linux-musl",
        "aarch64-unknown-linux-gnu",
        "armv7-unknown-linux-gnueabihf"
      ]
    }
  },
  "devDependencies": {
    "@napi-rs/cli": "^2.18.0"
  },
  "engines": {
    "node": ">= 10"
  },
  "license": "MIT",
  "repository": "https://github.com/your-org/scaffold",
  "keywords": [
    "scaffold",
    "rust",
    "napi",
    "generator",
    "cli"
  ]
}
```

### 3. npm 入口文件 (index.js)

```javascript
// Node.js 包的入口文件
const path = require('path');

// 动态加载 NAPI 模块
let napi;
try {
  napi = require('./napi/'); // 尝试加载预构建的二进制文件
} catch (error) {
  // 如果预构建文件不存在，回退到源码构建
  console.warn('Pre-built binary not found, falling back to source build');
  napi = require('./adapters/napi');
}

// 导出核心功能
module.exports = {
  // 类和构造函数
  Scaffold: napi.Scaffold,
  ScaffoldEngine: napi.ScaffoldEngine,

  // 工具函数
  init: napi.init,
  create: napi.create,
  build: napi.build,
  list: napi.list,

  // 版本信息
  version: napi.version,

  // 默认导出
  default: napi.Scaffold
};

// 如果直接运行此文件，启动 CLI
if (require.main === module) {
  const { Command } = require('commander');
  const program = new Command();

  program
    .name('scaffold')
    .description('Rust + NAPI-RS scaffold generator')
    .version(napi.version);

  program
    .command('init')
    .argument('[name]', 'Project name')
    .option('-t, --template <template>', 'Template type', 'basic')
    .action(async (name, options) => {
      try {
        await napi.init(name, options);
      } catch (error) {
        console.error('Error:', error.message);
        process.exit(1);
      }
    });

  program.parse();
}
```

### 4. CLI 二进制配置 (crates/bin/Cargo.toml)

```toml
[package]
name = "scaffold"
version = "0.1.0"
edition = "2021"
description = "A Rust + NAPI-RS project scaffold generator"

[[bin]]
name = "scaffold"
path = "src/main.rs"

[features]
default = ["cli"]

[dependencies]
# CLI 特定依赖
clap = { workspace = true, optional = true }
colored = { workspace = true, optional = true }
anyhow = { workspace = true }

# 内部依赖
adapters = { path = "../adapters", optional = true }
core = { path = "../core" }
common = { path = "../common" }
utils = { path = "../utils" }

# 只在启用 CLI 特性时包含
[build-dependencies]
```

### 3. NAPI 绑定配置 (crates/binding/Cargo.toml)

```toml
[package]
name = "scaffold-napi"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[features]
default = ["napi"]
napi = ["dep:napi", "dep:napi-derive", "core/napi"]

[dependencies]
# 可选依赖
napi = { workspace = true, optional = true }
napi-derive = { workspace = true, optional = true }

# 核心依赖
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }

# 内部依赖
core = { path = "../core", default-features = false }
common = { path = "../common", default-features = false }

[build-dependencies]
napi-build = { workspace = true, optional = true }

[package.metadata.napi]
targets = [
  "x86_64-pc-windows-msvc",
  "x86_64-apple-darwin",
  "aarch64-apple-darwin",
  "x86_64-unknown-linux-gnu"
]
```

## 核心实现

### 1. CLI 命令定义 (crates/args/src/command/cli.rs)

```rust
use clap::{Parser, Subcommand};
use crate::command::templates::TemplateType;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Rust + NAPI-RS project
    Init {
        /// Project name
        name: Option<String>,
        /// Template type to use
        #[arg(short, long, default_value = "basic")]
        template: TemplateType,
    },
    /// Create a new component/module
    Create {
        /// Component name
        name: String,
        /// Component type
        #[arg(short, long, default_value = "module")]
        component_type: String,
        /// Target path (optional)
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Build the project
    Build {
        /// Build target
        #[arg(short, long, default_value = "release")]
        profile: String,
        /// Target platform
        #[arg(short, long)]
        target: Option<String>,
    },
    /// List available templates
    List,
}
```

### 2. 模板类型定义 (crates/args/src/command/templates.rs)

```rust
use clap::ValueEnum;
use std::fmt;

#[derive(Clone, Debug, ValueEnum)]
pub enum TemplateType {
    /// Basic Rust + NAPI-RS setup
    Basic,
    /// CLI tool template
    Cli,
    /// Web server template
    Server,
    /// Library template
    Library,
    /// Full-stack application
    FullStack,
}

impl TemplateType {
    pub fn description(&self) -> &'static str {
        match self {
            TemplateType::Basic => "Basic Rust + NAPI-RS setup with minimal configuration",
            TemplateType::Cli => "Command-line interface tool template",
            TemplateType::Server => "Web server template with HTTP handlers",
            TemplateType::Library => "Library template with public API",
            TemplateType::FullStack => "Full-stack application with frontend and backend",
        }
    }
}

impl fmt::Display for TemplateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateType::Basic => write!(f, "basic"),
            TemplateType::Cli => write!(f, "cli"),
            TemplateType::Server => write!(f, "server"),
            TemplateType::Library => write!(f, "library"),
            TemplateType::FullStack => write!(f, "fullstack"),
        }
    }
}
```

### 3. 核心 Rust 库 (crates/core/Cargo.toml)

```toml
[package]
name = "scaffold-core"
version = "0.1.0"
edition = "2021"

[features]
default = []
napi = ["dep:napi"]

[dependencies]
# 可选的 NAPI 支持
napi = { workspace = true, optional = true }

# 核心依赖
serde = { workspace = true }
serde_json = { workspace = true }
tokio = { workspace = true }
anyhow = { workspace = true }
handlebars = { workspace = true }

# 内部依赖
common = { path = "../common", default-features = false }
```

### 4. 核心 Rust 库实现 (crates/core/src/engine.rs)

```rust
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;
use crate::template::TemplateEngine;

#[cfg(feature = "napi")]
use napi::bindgen_prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldConfig {
    pub name: String,
    pub version: String,
    pub template_type: String,
    pub options: ScaffoldOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldOptions {
    pub include_napi: bool,
    pub include_cli: bool,
    pub workspace: bool,
    pub author: Option<String>,
    pub license: Option<String>,
}

impl Default for ScaffoldOptions {
    fn default() -> Self {
        Self {
            include_napi: true,
            include_cli: false,
            workspace: false,
            author: None,
            license: Some("MIT".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct ScaffoldEngine {
    config: Arc<Mutex<ScaffoldConfig>>,
    template_engine: TemplateEngine,
}

impl ScaffoldEngine {
    pub fn new(config: ScaffoldConfig) -> Self {
        let template_engine = TemplateEngine::new();

        Self {
            config: Arc::new(Mutex::new(config)),
            template_engine,
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        let config = self.config.lock().await;

        // 创建项目目录结构
        self.template_engine.create_project_structure(&config).await?;

        // 生成配置文件
        self.template_engine.generate_configs(&config).await?;

        // 生成源代码文件
        self.template_engine.generate_sources(&config).await?;

        Ok(())
    }

    pub async fn add_component(&self, name: &str, component_type: &str) -> Result<()> {
        let config = self.config.lock().await;

        self.template_engine.add_component(&config, name, component_type).await
    }

    pub async fn build_project(&self, profile: &str, target: Option<&str>) -> Result<String> {
        let config = self.config.lock().await;

        self.template_engine.build_project(&config, profile, target).await
    }
}
```

### 5. 适配器层配置

**CLI 适配器 (crates/adapters/cli/Cargo.toml)**:
```toml
[package]
name = "scaffold-cli-adapter"
version = "0.1.0"
edition = "2021"

[features]
default = ["cli"]
cli = ["dep:clap", "dep:dialoguer"]

[dependencies]
# CLI 特定依赖
clap = { workspace = true, optional = true }
dialoguer = { workspace = true, optional = true }
colored = { workspace = true }

# 核心依赖
core = { path = "../../core" }
common = { path = "../../common" }
utils = { path = "../../utils" }
```

**NAPI 适配器 (crates/adapters/napi/Cargo.toml)**:
```toml
[package]
name = "scaffold-napi-adapter"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[features]
default = ["napi"]
napi = ["dep:napi", "dep:napi-derive"]

[dependencies]
# NAPI 特定依赖
napi = { workspace = true, optional = true }
napi-derive = { workspace = true, optional = true }

# 核心依赖
core = { path = "../../core" }
common = { path = "../../common" }
utils = { path = "../../utils" }

[build-dependencies]
napi-build = { workspace = true, optional = true }
```

### 6. CLI 适配器实现 (crates/adapters/cli/src/lib.rs)

```rust
//! CLI 特定的适配器实现
//! 这个模块只在启用 CLI 特性时编译

#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use anyhow::Result;
use core::{ScaffoldEngine, ScaffoldConfig, ScaffoldOptions};

/// CLI 参数结构
#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Project name
    pub name: Option<String>,

    /// Template type
    #[arg(short, long, default_value = "basic")]
    pub template: String,

    /// Include CLI in generated project
    #[arg(long)]
    pub include_cli: bool,

    /// Include NAPI in generated project
    #[arg(long)]
    pub include_napi: bool,
}

/// CLI 命令处理器
#[cfg(feature = "cli")]
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project
    Init {
        #[arg(help = "Project name")]
        name: String,
        #[arg(short, long, default_value = "basic")]
        template: String,
    },
    /// Create a new component
    Create {
        #[arg(help = "Component name")]
        name: String,
        #[arg(short, long, default_value = "module")]
        component_type: String,
    },
    /// List available templates
    List,
}

/// CLI 适配器的主要实现
pub struct CliAdapter {
    engine: ScaffoldEngine,
}

impl CliAdapter {
    pub fn new(config: ScaffoldConfig) -> Self {
        let engine = ScaffoldEngine::new(config);
        Self { engine }
    }

    pub async fn execute_command(args: CliArgs) -> Result<()> {
        match args.name {
            Some(name) => {
                let config = ScaffoldConfig {
                    name,
                    version: "0.1.0".to_string(),
                    template_type: args.template,
                    options: ScaffoldOptions {
                        include_cli: args.include_cli,
                        include_napi: args.include_napi,
                        ..Default::default()
                    },
                };

                let adapter = Self::new(config);
                adapter.engine.initialize().await?;
                println!("✅ Project initialized successfully!");
            }
            None => {
                println!("No project name provided. Use --help for usage information.");
            }
        }
        Ok(())
    }
}

#[cfg(feature = "cli")]
pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}
```

### 7. NAPI 适配器实现 (crates/adapters/napi/src/lib.rs)

```rust
//! NAPI 特定的适配器实现
//! 这个模块只在启用 NAPI 特性时编译

#[cfg(feature = "napi")]
use napi::bindgen_prelude::*;
#[cfg(feature = "napi")]
use napi_derive::napi;

use core::{ScaffoldEngine, ScaffoldConfig, ScaffoldOptions};
use std::sync::Arc;
use tokio::sync::Mutex;

/// NAPI 导出的配置对象
#[cfg_attr(feature = "napi", napi(object))]
pub struct JsScaffoldConfig {
    pub name: String,
    pub version: Option<String>,
    pub template_type: String,
    pub options: Option<JsScaffoldOptions>,
}

#[cfg_attr(feature = "napi", napi(object))]
pub struct JsScaffoldOptions {
    pub include_cli: Option<bool>,
    pub include_napi: Option<bool>,
    pub author: Option<String>,
    pub license: Option<String>,
}

impl Default for JsScaffoldOptions {
    fn default() -> Self {
        Self {
            include_cli: Some(false),
            include_napi: Some(true),
            author: None,
            license: Some("MIT".to_string()),
        }
    }
}

impl From<JsScaffoldOptions> for ScaffoldOptions {
    fn from(value: JsScaffoldOptions) -> Self {
        Self {
            include_cli: value.include_cli.unwrap_or(false),
            include_napi: value.include_napi.unwrap_or(true),
            author: value.author,
            license: value.license,
            ..Default::default()
        }
    }
}

/// NAPI 导出的主要类
#[cfg_attr(feature = "napi", napi)]
pub struct ScaffoldNapi {
    inner: Arc<Mutex<ScaffoldEngine>>,
}

#[cfg(feature = "napi")]
impl ScaffoldNapi {
    #[napi(constructor)]
    pub fn new(config: JsScaffoldConfig) -> napi::Result<Self> {
        let scaffold_config = ScaffoldConfig {
            name: config.name,
            version: config.version.unwrap_or_else(|| "0.1.0".to_string()),
            template_type: config.template_type,
            options: config.options.unwrap_or_default().into(),
        };

        let engine = ScaffoldEngine::new(scaffold_config);

        Ok(Self {
            inner: Arc::new(Mutex::new(engine)),
        })
    }

    #[napi]
    pub async fn initialize(&self) -> napi::Result<()> {
        let engine = self.inner.lock().await;
        engine.initialize().await
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
    }

    #[napi]
    pub async fn add_component(&self, name: String, component_type: String) -> napi::Result<()> {
        let engine = self.inner.lock().await;
        engine.add_component(&name, &component_type).await
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
    }
}

/// NAPI 模块的导出函数
#[cfg(feature = "napi")]
#[napi(module_exports)]
pub fn module_exports(exports: &mut Object, env: Env) -> napi::Result<()> {
    // 导出版本信息
    let version = env.create_string(std::env!("CARGO_PKG_VERSION"))?;
    exports.set_named_property("version", version)?;

    // 导出主要类
    let scaffold_ctor = env.get_class_constructor::<ScaffoldNapi>()?;
    exports.set_named_property("Scaffold", scaffold_ctor)?;

    // 导出工具函数
    exports.set_named_property("ScaffoldEngine", env.get_undefined())?;

    Ok(())
}
```

### 8. NAPI-RS 绑定 (crates/binding/src/lib.rs)

```rust
#[cfg(feature = "napi")]
mod napi_impl;

// 重新导出核心功能，无论是否启用 NAPI 都可用
pub use core::{ScaffoldEngine, ScaffoldConfig, ScaffoldOptions, TemplateEngine};

// NAPI 特定实现只在启用特性时编译
#[cfg(feature = "napi")]
pub use napi_impl::*;
```

### 6. NAPI 实现模块 (crates/binding/src/napi_impl.rs)

```rust
use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::{ScaffoldEngine, ScaffoldConfig, ScaffoldOptions};
use std::sync::Arc;
use tokio::sync::Mutex;

#[napi(object)]
pub struct JsScaffoldConfig {
    pub name: String,
    pub version: String,
    pub template_type: String,
    pub options: Option<JsScaffoldOptions>,
}

impl TryFrom<JsScaffoldConfig> for ScaffoldConfig {
    type Error = Error;

    fn try_from(value: JsScaffoldConfig) -> Result<Self> {
        Ok(ScaffoldConfig {
            name: value.name,
            version: value.version,
            template_type: value.template_type,
            options: value.options.unwrap_or_default().into(),
        })
    }
}

#[napi(object)]
pub struct JsScaffoldOptions {
    pub include_napi: Option<bool>,
    pub include_cli: Option<bool>,
    pub workspace: Option<bool>,
    pub author: Option<String>,
    pub license: Option<String>,
}

impl Default for JsScaffoldOptions {
    fn default() -> Self {
        Self {
            include_napi: Some(true),
            include_cli: Some(false),
            workspace: Some(false),
            author: None,
            license: Some("MIT".to_string()),
        }
    }
}

impl From<JsScaffoldOptions> for ScaffoldOptions {
    fn from(value: JsScaffoldOptions) -> Self {
        Self {
            include_napi: value.include_napi.unwrap_or(true),
            include_cli: value.include_cli.unwrap_or(false),
            workspace: value.workspace.unwrap_or(false),
            author: value.author,
            license: value.license,
        }
    }
}

#[napi]
pub struct ScaffoldNapi {
    inner: Arc<Mutex<ScaffoldEngine>>,
}

#[napi]
impl ScaffoldNapi {
    #[napi(constructor)]
    pub fn new(config: JsScaffoldConfig) -> Result<Self> {
        let config = config.try_into()?;
        let engine = ScaffoldEngine::new(config);

        Ok(Self {
            inner: Arc::new(Mutex::new(engine)),
        })
    }

    #[napi]
    pub async fn initialize(&self) -> Result<()> {
        let engine = self.inner.lock().await;
        engine.initialize().await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    #[napi]
    pub async fn add_component(&self, name: String, component_type: String) -> Result<()> {
        let engine = self.inner.lock().await;
        engine.add_component(&name, &component_type).await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    #[napi]
    pub async fn build_project(&self, profile: String, target: Option<String>) -> Result<String> {
        let engine = self.inner.lock().await;
        engine.build_project(&profile, target.as_deref()).await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }
}

#[napi(module_exports)]
pub fn module_exports(exports: &mut Object, env: Env) -> Result<()> {
    // 导出版本信息
    let version = env.create_string(std::env!("CARGO_PKG_VERSION"))?;
    exports.set_named_property("version", version)?;

    // 导出主要类
    let scaffold_ctor = env.get_class_constructor::<ScaffoldNapi>()?;
    exports.set_named_property("Scaffold", scaffold_ctor)?;

    Ok(())
}
```

### 7. 公共库配置 (crates/common/Cargo.toml)

```toml
[package]
name = "scaffold-common"
version = "0.1.0"
edition = "2021"

[features]
default = []
napi = ["dep:napi"]

[dependencies]
# 可选的 NAPI 支持
napi = { workspace = true, optional = true }

# 核心依赖
serde = { workspace = true }
serde_json = { workspace = true }
uuid = { workspace = true, features = ["v4"] }
```

### 8. CLI 入口 (crates/bin/src/main.rs)

```rust
use anyhow::Result;
use clap::Parser;
use colored::*;
use args::command::cli::{Cli, Commands};
use commands::{init, create, build, list};
use utils::print::{print_banner, print_success, print_error};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // 打印横幅
    if cli.debug > 0 {
        print_banner();
    }

    match cli.command {
        Some(Commands::Init { name, template }) => {
            print_success(&format!("Initializing new project with template: {}", template));
            init::execute(name, template).await?;
        }
        Some(Commands::Create { name, component_type, path }) => {
            print_success(&format!("Creating new component: {} ({})", name, component_type));
            create::execute(name, component_type, path).await?;
        }
        Some(Commands::Build { profile, target }) => {
            print_success(&format!("Building project with profile: {}", profile));
            build::execute(profile, target).await?;
        }
        Some(Commands::List) => {
            list::execute().await?;
        }
        None => {
            print_error("No command provided. Use --help for available commands.");
        }
    }

    Ok(())
}
```

### 6. 工具函数 (crates/utils/src/print.rs)

```rust
use colored::*;
use std::io::{self, Write};

pub fn print_banner() {
    let banner = r#"
     ██████╗ ███████╗██████╗  █████╗ ████████╗
    ██╔════╝ ██╔════╝██╔══██╗██╔══██╗╚══██╔══╝
    ██║  ███╗█████╗  ██████╔╝███████║   ██║
    ██║   ██║██╔══╝  ██╔══██╗██╔══██║   ██║
    ╚██████╔╝███████╗██║  ██║██║  ██║   ██║
     ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝
    "#;

    println!("{}", banner.bright_cyan().bold());
    println!("{}", "Rust + NAPI-RS Scaffold Generator".bright_yellow());
    println!("{}", "Create high-performance Rust-powered Node.js projects".white());
    println!();
}

pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message.white());
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message.white());
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message.white());
}

pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message.white());
}

pub fn print_step(step: usize, total: usize, message: &str) {
    println!(
        "{} [{}/{}] {}",
        "→".cyan().bold(),
        step,
        total,
        message.white()
    );
    io::stdout().flush().unwrap();
}
```

## 双模式构建系统

### 1. 纯 Rust CLI 工具构建（不依赖 Node.js）

```bash
# 克隆项目
git clone <repository-url>
cd rust-napi-scaffold

# 基础 Rust 构建（不包含 NAPI）
cargo build

# 运行 CLI 工具
cargo run -- init my-project

# 发布构建
cargo build --release

# 运行测试（不包含 NAPI 测试）
cargo test --workspace --no-default-features

# 代码格式化
cargo fmt --all

# 代码检查
cargo clippy --workspace -- -D warnings

# 安装到本地
cargo install --path crates/bin --no-default-features
```

### 2. npm 包构建（需要 Node.js 环境）

```bash
# 构建 npm 包
npm run build

# 开发构建
npm run build:debug

# 测试
npm test

# 发布到 npm
npm publish
```

### 3. 完整构建（CLI + npm）

```bash
# 构建 Rust CLI 工具
cargo build --release --features cli

# 构建 npm 包
npm run build

# 或使用一步构建（需要 Node.js 环境）
npm run build:all

# 运行完整测试套件
npm run test:all

# 发布到两个平台
npm run publish:all
```

### 4. 分层构建策略

**核心库构建**（运行时无关）:
```bash
# 只构建核心功能
cargo build -p scaffold-core --no-default-features

# 构建核心库并测试
cargo test -p scaffold-core --no-default-features
```

**适配器构建**:
```bash
# 只构建 CLI 适配器
cargo build -p scaffold-cli-adapter --features cli

# 只构建 NAPI 适配器
cargo build -p scaffold-napi-adapter --features napi
```

### 5. 构建脚本示例

**scripts/build.sh**:
```bash
#!/bin/bash

set -e

echo "🚀 Building Rust + NAPI-RS Scaffold..."

# 构建纯 Rust 版本
echo "📦 Building pure Rust version..."
cargo build --release --no-default-features

# 构建 NAPI 版本（如果需要）
if [ "$1" = "--with-napi" ]; then
    echo "🔗 Building with NAPI support..."
    cargo build --release --features napi

    # 构建 Node.js 绑定
    echo "📚 Building Node.js bindings..."
    cd crates/binding
    npm ci
    npm run build:release
    cd ../..
fi

# 运行测试
echo "🧪 Running tests..."
cargo test --workspace --features napi

echo "✅ Build completed successfully!"
```

**scripts/cross-build.sh**:
```bash
#!/bin/bash

set -e

TARGETS=(
    "x86_64-pc-windows-msvc"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-unknown-linux-gnu"
)

for target in "${TARGETS[@]}"; do
    echo "🔨 Building for target: $target"

    # 纯 Rust 构建
    cargo build --release --target "$target" --no-default-features

    # 如果支持该目标的 NAPI 构建
    if cargo check --target "$target" --features napi 2>/dev/null; then
        echo "🔗 Building NAPI for $target"
        cargo build --release --target "$target" --features napi
    fi
done

echo "✅ Cross-platform build completed!"
```

### 6. 双模式发布脚本

**scripts/publish.sh**:
```bash
#!/bin/bash

set -e

echo "🚀 Publishing Scaffold to both platforms..."

# 检查环境
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found"
    exit 1
fi

if ! command -v npm &> /dev/null; then
    echo "❌ NPM not found"
    exit 1
fi

# 1. 发布 Rust CLI 工具到 crates.io
echo "📦 Publishing Rust CLI tool to crates.io..."
cd crates/bin
cargo publish --no-verify
cd ../..

# 等待 crates.io 索引更新
echo "⏳ Waiting for crates.io index..."
sleep 30

# 2. 发布 npm 包
echo "📦 Publishing npm package..."
npm publish

echo "✅ Successfully published to both platforms!"
echo "🦀 Install with: cargo install scaffold"
echo "📦 Install with: npm install -g @your-org/scaffold"
```

**Makefile**:
```makefile
.PHONY: build-rust build-npm build-all test-rust test-npm publish-rust publish-npm publish-all

# Rust 构建
build-rust:
	cargo build --release --features cli

# npm 构建
build-npm:
	npm run build

# 完整构建
build-all: build-rust build-npm

# Rust 测试
test-rust:
	cargo test --workspace --features cli

# npm 测试
test-npm:
	npm test

# 发布到 crates.io
publish-rust: build-rust
	cd crates/bin && cargo publish

# 发布到 npm
publish-npm: build-npm
	npm publish

# 发布到两个平台
publish-all: publish-rust publish-npm

# 检查环境
check-env:
	@echo "🦀 Rust: $(shell rustc --version)"
	@echo "📦 Node.js: $(shell node --version 2>/dev/null || echo 'Not installed')"
	@echo "📋 NPM: $(shell npm --version 2>/dev/null || echo 'Not installed')"
```

**justfile** (更现代的替代方案):
```just
# 默认任务
default: build

# 纯 Rust 构建
build:
    cargo build --release --no-default-features

# 带 NAPI 的构建
build-napi:
    cargo build --release --features napi

# 纯 Rust 测试
test:
    cargo test --workspace --no-default-features

# 包含 NAPI 的测试
test-napi:
    cargo test --workspace --features napi

# 代码格式化
fmt:
    cargo fmt --all

# 代码检查
clippy:
    cargo clippy --workspace -- -D warnings

# 清理构建
clean:
    cargo clean

# 安装纯 Rust 版本
install:
    cargo install --path crates/bin --no-default-features

# 安装包含 NAPI 的版本
install-napi:
    cargo install --path crates/bin --features napi

# 构建所有平台
cross-build:
    ./scripts/cross-build.sh

# 检查环境
check-env:
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    {{if command_available("node")}}
        echo "Node.js version: $(node --version)"
        echo "NPM version: $(npm --version)"
    {{else}}
        echo "Node.js: Not installed"
    {{endif}}

# 助手命令
help:
    echo "Available commands:"
    echo "  build       - Build pure Rust version"
    echo "  build-napi  - Build with NAPI support"
    echo "  test        - Run pure Rust tests"
    echo "  test-napi   - Run tests with NAPI support"
    echo "  install     - Install pure Rust version"
    echo "  install-napi - Install version with NAPI support"
    echo "  cross-build - Build for all platforms"
    echo "  check-env   - Check build environment"
```

### 6. 纯 Rust 使用示例

**examples/basic_usage.rs**:
```rust
use scaffold_core::{ScaffoldEngine, ScaffoldConfig, ScaffoldOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建配置
    let config = ScaffoldConfig {
        name: "my-project".to_string(),
        version: "0.1.0".to_string(),
        template_type: "basic".to_string(),
        options: ScaffoldOptions {
            include_napi: false,  // 纯 Rust 项目
            include_cli: true,
            workspace: false,
            author: Some("Your Name".to_string()),
            license: Some("MIT".to_string()),
        },
    };

    // 初始化引擎
    let engine = ScaffoldEngine::new(config);

    // 初始化项目
    engine.initialize().await?;

    // 添加组件
    engine.add_component("user-service", "module").await?;

    // 构建项目
    let build_output = engine.build_project("release", None).await?;
    println!("Build output: {}", build_output);

    Ok(())
}
```

**examples/library_usage.rs**:
```rust
// 将这个库作为依赖使用
use scaffold_core::{ScaffoldEngine, ScaffoldConfig};

fn create_project(name: &str, template: &str) -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        let config = ScaffoldConfig {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            template_type: template.to_string(),
            options: Default::default(),
        };

        let engine = ScaffoldEngine::new(config);
        engine.initialize().await
    })
}

fn main() {
    create_project("awesome-lib", "library").unwrap();
    println!("✅ Library project created successfully!");
}
```

### 7. 条件编译最佳实践

**crates/core/src/lib.rs**:
```rust
#![cfg_attr(not(feature = "napi"), allow(dead_code))]

pub mod engine;
pub mod template;

// 重新导出核心类型
pub use engine::{ScaffoldEngine, ScaffoldConfig, ScaffoldOptions};
pub use template::TemplateEngine;

// NAPI 特定的功能只在启用特性时可用
#[cfg(feature = "napi")]
pub mod napi_exports;

// 平台特定的功能
#[cfg(target_os = "windows")]
pub mod windows_specific;

#[cfg(target_os = "macos")]
pub mod macos_specific;

#[cfg(target_os = "linux")]
pub mod linux_specific;

// 测试辅助模块
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_creation() {
        let config = ScaffoldConfig {
            name: "test".to_string(),
            version: "0.1.0".to_string(),
            template_type: "basic".to_string(),
            options: Default::default(),
        };

        let engine = ScaffoldEngine::new(config);
        assert!(engine.initialize().await.is_ok());
    }

    // NAPI 特定测试
    #[cfg(feature = "napi")]
    #[tokio::test]
    async fn test_napi_functionality() {
        // NAPI 特定的测试
    }
}
```

## 模板系统

### 1. 模板目录结构

```
templates/
├── basic/
│   ├── Cargo.toml.hbs
│   ├── src/
│   │   └── lib.rs.hbs
│   └── README.md.hbs
├── cli/
│   ├── Cargo.toml.hbs
│   ├── src/
│   │   └── main.rs.hbs
│   └── args/
│       └── cli.rs.hbs
└── server/
    ├── Cargo.toml.hbs
    ├── src/
    │   ├── lib.rs.hbs
    │   └── handlers/
    │       └── mod.rs.hbs
    └── routes/
        └── mod.rs.hbs
```

### 2. Handlebars 模板示例

**templates/basic/Cargo.toml.hbs**:
```toml
[package]
name = "{{name}}"
version = "{{version}}"
edition = "2021"
authors = ["{{author}}"]
license = "{{license}}"
description = "{{description}}"

[dependencies]
{{#if includeNapi}}
napi = { version = "3", features = ["async"] }
napi-derive = "3"
tokio = { version = "1", features = ["full"] }
{{/if}}

{{#if includeCli}}
clap = { version = "4", features = ["derive"] }
{{/if}}
```

## 双模式使用指南

### 1. Rust CLI 工具使用

```bash
# 安装
cargo install scaffold

# 初始化新项目
scaffold init my-rust-project --template basic

# 创建带 CLI 的项目
scaffold init my-cli --template cli --include-cli

# 创建 Web 服务项目
scaffold init my-server --template server

# 添加组件
scaffold create user-service module
```

### 2. npm 包使用

```bash
# 全局安装
npm install -g @your-org/scaffold

# 或本地安装
npm install @your-org/scaffold

# 使用 CLI
scaffold init my-project --template basic

# 添加组件
scaffold create user-service module
```

### 3. Node.js 编程接口

```javascript
// 引入模块
const { Scaffold } = require('@your-org/scaffold');

// 创建实例
const scaffold = new Scaffold({
  name: 'my-project',
  templateType: 'basic',
  options: {
    includeNapi: true,
    includeCli: false,
    author: 'Your Name',
    license: 'MIT'
  }
});

// 使用
async function createProject() {
  try {
    await scaffold.initialize();
    console.log('✅ Project created successfully!');

    await scaffold.addComponent('user-service', 'module');
    console.log('✅ Component added!');
  } catch (error) {
    console.error('❌ Error:', error.message);
  }
}

createProject();
```

### 4. 构建生成的项目

```bash
# 开发构建
scaffold build debug

# 发布构建
scaffold build release

# 交叉编译
scaffold build release --target x86_64-pc-windows-msvc
```

## 双模式优势对比

### Rust CLI 模式
**优势:**
- ✅ 零依赖，无需 Node.js 环境
- ✅ 极快的启动速度和执行性能
- ✅ 单一二进制文件，易于分发
- ✅ 与 Rust 生态系统完美集成
- ✅ 内存安全和线程安全

**适用场景:**
- Rust 开发者
- CI/CD 环境中的自动化脚本
- 对性能要求极高的场景
- 嵌入式系统或受限环境

### npm 包模式
**优势:**
- ✅ 与 JavaScript/TypeScript 生态系统集成
- ✅ 支持编程接口调用
- ✅ 可作为 npm 依赖引入
- ✅ 跨平台预编译二进制文件
- ✅ 熟悉的包管理和安装方式

**适用场景:**
- JavaScript/TypeScript 开发者
- Node.js 项目中的构建脚本
- 需要编程接口的场景
- 前端工程化工具链

### 双模式统一性

无论使用哪种安装方式，都提供：
- 🔄 **相同的功能接口**
- 🔄 **一致的使用体验**
- 🔄 **统一的配置格式**
- 🔄 **兼容的项目输出**

## 最佳实践

### 1. 项目组织

- **单一职责**: 每个 crate 负责特定功能
- **特性门控**: 使用 Cargo features 控制功能编译
- **依赖管理**: 使用 workspace 统一管理依赖
- **模块化**: 功能模块化，便于测试和维护

### 2. 构建策略

- **渐进式构建**: 支持纯 Rust 和带 NAPI 的构建
- **条件编译**: 使用 `#[cfg(feature = "...")]` 控制代码编译
- **交叉编译**: 支持多平台构建和发布
- **缓存利用**: 利用 Cargo 的增量构建加速开发

### 3. 代码质量

- **错误处理**: 使用 `Result<T>` 和 `anyhow` 进行错误处理
- **类型安全**: 充分利用 Rust 的类型系统
- **文档**: 为公共 API 编写文档注释
- **测试覆盖**: 分别测试纯 Rust 和 NAPI 功能

### 4. 用户体验

- **彩色输出**: 使用 `colored` 提供友好的终端输出
- **交互式界面**: 使用 `dialoguer` 提供交互式选择
- **进度指示**: 提供清晰的进度反馈
- **环境检测**: 自动检测构建环境并提示用户

### 5. 性能优化

- **异步编程**: 使用 `tokio` 进行异步操作
- **并行处理**: 利用多核处理器并行执行任务
- **零成本抽象**: 充分利用 Rust 的零成本抽象特性
- **缓存**: 适当使用缓存减少重复计算

### 6. 版本管理

- **语义化版本**: 遵循 SemVer 规范
- **特性兼容性**: 确保向后兼容性
- **发布流程**: 建立清晰的发布和版本管理流程

### 7. 依赖管理

- **最小依赖**: 只引入必要的依赖
- **可选依赖**: 使用 optional dependencies 减少默认依赖
- **版本锁定**: 在 workspace 中统一管理版本
- **安全审计**: 定期检查依赖的安全漏洞

## 扩展指南

### 1. 添加新模板

1. 在 `templates/` 目录创建模板文件
2. 在 `TemplateType` 枚举添加新类型
3. 更新模板引擎生成逻辑

### 2. 添加新命令

1. 在 `Commands` 枚举添加新命令
2. 在 `commands/` 目录创建命令实现
3. 在 `main.rs` 处理新命令

### 3. 集成新工具

1. 在 `utils/` 目录添加工具函数
2. 更新相关配置文件
3. 添加测试用例

## 故障排除

### 常见问题

1. **构建失败**: 检查 Rust 版本和依赖兼容性
2. **NAPI 构建**: 确保 Node.js 和 NAPI 工具链正确安装
3. **模板错误**: 验证 Handlebars 模板语法
4. **权限问题**: 确保有文件写入权限

### 调试技巧

- 使用 `RUST_LOG=debug` 环境变量启用详细日志
- 使用 `cargo expand` 查看宏展开
- 使用 `nm` 检查生成的二进制文件符号

## 总结

这个架构提供了一个完整的 Rust + NAPI-RS 脚手架解决方案，结合了：

- 现代化的 Rust 开发实践
- 清晰的项目组织结构
- 强大的 CLI 工具
- 灵活的模板系统
- 优秀的用户体验
- 可扩展的设计

通过遵循这个架构，你可以快速创建高质量的 Rust + Node.js 项目，同时保持代码的可维护性和可扩展性。