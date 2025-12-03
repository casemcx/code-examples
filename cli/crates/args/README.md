# Args - CLI 参数定义

使用 Clap 框架定义命令行接口（CLI）的参数结构。

## 职责

- 定义 CLI 命令结构
- 定义命令参数和选项
- 提供类型安全的参数解析
- 自动生成帮助文档

## 依赖

- `clap` - 命令行参数解析框架
- `common` - 公共类型定义

## 代码结构

```
src/
├── lib.rs              # Crate 入口
└── command/
    ├── mod.rs
    └── cli.rs          # CLI 定义
```

## 主要类型

### Cli

程序的主要 CLI 结构体：

```rust
pub struct Cli {
    pub command: Option<Commands>,
}
```

### Commands

支持的命令枚举：

```rust
pub enum Commands {
    /// 初始化新项目（可选择 monorepo 或单体项目）
    Init {
        name: Option<String>,
        template: String,
    },

    /// 在 monorepo 中创建新的子项目
    Create {
        name: Option<String>,
        template: String,
        workspace: Option<String>,
    },

    /// 列出可用模板
    List,
}
```

## 命令参数说明

### Init 命令

```bash
scaffold init [OPTIONS]

Options:
  -n, --name <NAME>          项目名称
  -t, --template <TEMPLATE>  模板名称 [default: ]
  -h, --help                 Print help
```

### Create 命令

```bash
scaffold create [OPTIONS]

Options:
  -n, --name <NAME>            子项目名称
  -t, --template <TEMPLATE>    模板名称 [default: ]
  -w, --workspace <WORKSPACE>  workspace 目录（如 apps, packages）
  -h, --help                   Print help
```

### List 命令

```bash
scaffold list

列出所有可用的项目模板
```

## 使用示例

```rust
use clap::Parser;
use args::command::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { name, template }) => {
            // 处理 init 命令
        }
        Some(Commands::Create { name, template, workspace }) => {
            // 处理 create 命令
        }
        Some(Commands::List) => {
            // 处理 list 命令
        }
        None => {
            println!("请使用 --help 查看使用说明");
        }
    }
}
```

## 扩展命令

要添加新命令：

1. 在 `Commands` 枚举中添加新变体
2. 使用 Clap 的宏定义参数
3. 添加文档注释（会自动成为帮助文本）

示例：

```rust
pub enum Commands {
    // ... 现有命令

    /// 更新项目依赖
    Update {
        /// 是否强制更新
        #[arg(short, long)]
        force: bool,
    },
}
```
