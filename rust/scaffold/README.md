# Scaffold

一个基于 Rust 的脚手架管理工具，用于快速创建和管理项目模板。

## 项目结构

```
scaffold/
├── crates/
│   ├── bin/          # 二进制入口
│   ├── commands/     # 命令实现
│   └── utils/        # 工具函数
├── scripts/
│   ├── build.sh      # 构建脚本
│   ├── debug.sh      # 调试脚本
│   ├── test.sh       # 测试脚本
│   └── install.sh    # 安装脚本
└── Cargo.toml        # Workspace 配置
```

## 功能

- `new` - 创建新项目
- `update` - 更新模板

## 环境要求

- Rust 2024 edition
- Cargo

## 安装

### 从源码构建

```bash
# 克隆仓库
git clone <repository-url>
cd scaffold

# 构建项目
./scripts/build.sh --release

# 二进制文件位于
./target/release/scaffold
```

### 使用安装脚本（推荐）

```bash
# 一键安装
sh ./scripts/install.sh
```

安装完成后，如果提示 PATH 问题，运行：
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

然后就可以直接使用：
```bash
scaffold --help
```

### 使用 Cargo 安装

```bash
# 方式1: 直接安装 bin crate
cargo install --path crates/bin

# 方式2: 使用 cargo install 并指定 binary
cargo install --path . --bin scaffold

# 方式3: 构建后手动复制
./scripts/build.sh --release
cp target/release/scaffold ~/.cargo/bin/
```

## 使用

### 构建项目

```bash
# 构建调试版本
./scripts/build.sh

# 构建发布版本
./scripts/build.sh --release

# 清理后构建
./scripts/build.sh --clean --release

# 构建并运行
./scripts/build.sh --run
```

### 调试

```bash
# 使用 debug 日志级别运行
./scripts/debug.sh

# 使用 trace 日志级别运行
./scripts/debug.sh -l trace

# 构建后运行
./scripts/debug.sh -b

# 运行特定命令
./scripts/debug.sh -b -- new --help

# 使用 lldb 调试器
./scripts/debug.sh --lldb
```

### 测试

```bash
# 运行测试
./scripts/test.sh

# 测试后保留测试项目
./scripts/test.sh --no-cleanup

# 指定日志级别
./scripts/test.sh -l debug

# 测试前先构建
./scripts/test.sh -b
```

### 命令

#### 创建新项目

```bash
scaffold new --template <模板名> --name <项目名>
```

#### 更新模板

```bash
scaffold update
```

#### 日志级别控制

所有命令都支持 `-l` / `--log-level` 参数：

```bash
# 设置日志级别为 trace
scaffold -l trace new
# 或使用长格式
scaffold --log-level trace new

# 设置日志级别为 error
scaffold -l error update

# 可用级别: trace, debug, info, warn, error
```

**默认行为：**
- Debug 构建: 默认 `info` 级别
- Release 构建: 默认 `error` 级别

### 环境变量

- `RUST_LOG` - 控制日志级别 (trace/debug/info/warn/error)，优先级低于 `--log-level`
  ```bash
  RUST_LOG=trace scaffold new
  ```

- `RUST_BACKTRACE` - 显示错误堆栈
  ```bash
  RUST_BACKTRACE=1 scaffold new
  ```

## 开发

### 项目结构

- `crates/bin/` - CLI 入口点，包含 main.rs 和命令行参数解析
- `crates/commands/` - 命令处理逻辑
  - `new/` - 创建新项目命令
  - `update/` - 更新模板命令
- `crates/utils/` - 共享工具函数和日志配置

### 构建脚本

**build.sh** - 提供便捷的构建选项：

| 选项 | 说明 |
|------|------|
| `-r, --release` | 构建发布版本 |
| `-v, --verbose` | 显示详细输出 |
| `-c, --clean` | 构建前清理 |
| `--run` | 构建后运行程序 |
| `--run-args <args>` | 运行程序时的参数 |

**debug.sh** - 用于调试运行：

| 选项 | 说明 |
|------|------|
| `-l, --log-level <level>` | 设置日志级别 |
| `-b, --build` | 运行前先构建 |
| `-r, --release` | 使用 release 版本 |
| `--lldb` | 使用 lldb 调试器 |

**test.sh** - 用于功能测试：

| 选项 | 说明 |
|------|------|
| `--no-cleanup` | 测试后不删除测试项目 |
| `-l, --log-level <level>` | 设置日志级别 |
| `-b, --build` | 测试前先构建 |

**install.sh** - 一键安装到系统（自动构建 release 版本并安装到 `~/.cargo/bin`）

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
# 检查代码
cargo check

# 格式化代码
cargo fmt

# Lint 检查
cargo clippy
```

## 依赖

主要依赖项：

- `clap` - 命令行参数解析
- `anyhow` - 错误处理
- `tracing` - 日志框架
- `thiserror` - 错误定义
- `dialoguer` - 交互式命令行
- `handlebars` - 模板引擎
- `colored` - 彩色输出

## License

MIT
