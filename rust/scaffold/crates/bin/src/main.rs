use clap::Parser;
use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use tracing::Level;
use commands::{update, new};

#[derive(Parser)]
struct Cli {
    /// 日志级别: trace, debug, info, warn, error
    #[arg(short = 'l', long = "log-level", global = true)]
    log_level: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    UPDATE,
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
    let app = Cli::parse();

    // 初始化 tracing
    init_tracing(app.log_level);

    match app.command {
        Some(Commands::UPDATE) => {
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
fn init_tracing(log_level: Option<String>) {
    // 确定默认日志级别：debug 构建默认 info，release 构建默认 error
    let default_level = if cfg!(debug_assertions) {
        Level::INFO
    } else {
        Level::ERROR
    };

    // 解析日志级别
    let level = if let Some(level_str) = log_level {
        // 用户通过 --log-level 指定了日志级别
        match level_str.to_lowercase().as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => default_level,
        }
    } else {
        // 使用环境变量 RUST_LOG，否则使用默认级别
        default_level
    };

    // 从环境变量 RUST_LOG 读取，并添加默认级别
    let filter = EnvFilter::from_default_env()
        .add_directive(level.into());

    // 配置彩色输出到终端
    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_target(true)
                .with_line_number(true)
        )
        .init();
}
