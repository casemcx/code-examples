use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 创建新项目
    Create {
        /// 项目名称
        #[arg(short, long)]
        name: Option<String>,
        
        /// 模板名称
        #[arg(short, long, default_value = "")]
        template: String,
    },
    /// 列出可用模板
    List,
}
