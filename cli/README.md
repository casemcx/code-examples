# Scaffold CLI

一个基于 Rust 的脚手架命令行工具,用于快速创建项目模板。

## 项目介绍

Scaffold 是一个简单高效的项目脚手架工具,帮助开发者快速初始化新项目。通过预定义的项目模板,可以快速生成标准化的项目结构,提高开发效率。

## 技术栈

- **Rust** - 系统编程语言
- **Clap** - 命令行解析库
- **Dialoguer** - 交互式命令行界面
- **Handlebars** - 模板引擎
- **Colored** - 终端颜色输出

## 项目结构

```
cli/
├── crates/
│   ├── bin/          # 主程序入口
│   ├── cli/          # 命令行接口定义
│   ├── command/      # 命令实现
│   ├── common/       # 公共类型和常量
│   └── utils/        # 工具函数
├── Cargo.toml        # Workspace 配置
└── README.md
```

## 环境要求

- Rust >= 1.70.0
- Cargo (随 Rust 一起安装)

## 安装

### 从源码构建

```bash
# 克隆项目
git clone <repository-url>
cd cli

# 构建项目
cargo build --release

# 可执行文件位于 target/release/scaffold
```

### 安装到系统

```bash
# 安装到 ~/.cargo/bin/ (确保该目录在 PATH 中)
cargo install --path crates/bin
```

## 使用说明

### 查看帮助

```bash
scaffold --help
```

### 列出可用模板

查看所有可用的项目模板:

```bash
scaffold list
```

### 创建新项目

使用指定的模板创建新项目:

```bash
# 交互式创建 (会提示输入项目名称)
scaffold create -t <template-name>

# 直接指定项目名称
scaffold create -n <project-name> -t <template-name>

# 简写形式
scaffold create --name my-project --template vue
```

### 命令参数说明

#### create 命令

- `-n, --name <NAME>` - 项目名称 (可选,不提供则交互式输入)
- `-t, --template <TEMPLATE>` - 模板名称 (必需)

#### list 命令

无需参数,直接列出所有可用模板。

## 开发指南

### 开发环境设置

```bash
# 安装依赖
cargo fetch

# 运行开发版本
cargo run -- --help

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

### 添加新模板

1. 在相应的模板目录中添加模板文件
2. 在 `common` crate 中定义模板配置
3. 在 `command` crate 中实现模板创建逻辑

### 项目构建

```bash
# 调试构建
cargo build

# 发布构建 (优化编译)
cargo build --release
```

## 常见问题

### Q: 如何添加自定义模板?

A: 目前需要修改源码添加模板支持。后续版本将支持自定义模板配置。

### Q: 创建的项目在哪里?

A: 项目会在当前工作目录下创建,使用指定的项目名称作为目录名。

## 贡献指南

欢迎提交 Issue 和 Pull Request!

提交前请确保:
- 代码通过 `cargo fmt` 格式化
- 代码通过 `cargo clippy` 检查
- 所有测试通过 `cargo test`

## 许可证

[添加许可证信息]

## 作者

[添加作者信息]

## 更新日志

### v0.1.0 (当前版本)

- 初始版本
- 支持基本的项目创建功能
- 支持模板列表查看
