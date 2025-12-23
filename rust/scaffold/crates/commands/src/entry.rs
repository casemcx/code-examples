use clap::Parser;
use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    /// 更新本地代码仓库
    Update,
    /// 创建新项目
    New {
        /// 模板名称
        #[arg(short, long)]
        template: Option<String>,
        /// 项目名称
        #[arg(short, long)]
        name: Option<String>,
    },
}

fn main() -> Result<()> {
    // 初始化 tracing subscriber
    init_tracing();

    let app = Cli::parse();

    match app.command {
        Some(Commands::Update) => {
            update::update()?;
        }
        Some(Commands::New { template, name }) => {
            new::new(template, name)?;
        }
        None => {
            println!("请使用 --help 查看使用说明");
        }
    }

    Ok(())
}

/// 初始化 tracing 日志系统
fn init_tracing() {
    // 从环境变量 RUST_LOG 读取日志级别，默认为 info
    let filter = EnvFilter::from_default_env()
        .add_directive("statr=info".into());

    // 配置彩色输出到终端
    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_target(true)
                .with_line_number(true)
                .with_file(false)
                .with_thread_ids(false)
                .with_thread_names(false)
        )
        .init();
}
