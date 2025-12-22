use clap::Parser;

use commands::update::update;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    UPDATE,
    Init {
        name: String,
        #[arg(short, long)]
        template: Option<String>,
    },
    Create {
        name: String,
        #[arg(short, long)]
        template: Option<String>,
        #[arg(short, long)]
        workspace: bool,
    },
    List,
}

fn main() {
    let app = Cli::parse();

    match app.command {
        Some(Commands::UPDATE) => {
            let _ = update();
            println!("安装完成")
        }
        Some(Commands::Init { name, template }) => {
            println!("初始化项目: {}", name);
            if let Some(t) = template {
                println!("使用模板: {}", t);
            }
        }
        Some(Commands::Create {
            name,
            template,
            workspace,
        }) => {
            println!("创建项目: {}", name);
            if let Some(t) = template {
                println!("使用模板: {}", t);
            }
            if workspace {
                println!("创建为工作空间");
            }
        }
        Some(Commands::List) => {
            println!("列出可用模板");
        }
        None => {
            println!("请使用 --help 查看使用说明");
        }
    }
}
