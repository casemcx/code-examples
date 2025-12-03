use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 初始化新项目（可选择 monorepo 或单体项目）
    Init {
        /// 项目名称
        #[arg(short, long)]
        name: Option<String>,

        /// 模板名称
        #[arg(short, long, default_value = "")]
        template: String,
    },
    /// 在 monorepo 中创建新的子项目
    Create {
        /// 子项目名称
        #[arg(short, long)]
        name: Option<String>,

        /// 模板名称
        #[arg(short, long, default_value = "")]
        template: String,

        /// workspace 目录（如 apps, packages）
        #[arg(short, long)]
        workspace: Option<String>,
    },
    /// 列出可用模板
    List,
}
