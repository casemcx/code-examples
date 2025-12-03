use clap::Parser;

use args::command::cli::{Cli, Commands};
use commands::{create::create_sub_project, init::init_project, list::list_project};

fn main() {
    let app = Cli::parse();

    match app.command {
        Some(Commands::Init { name, template }) => {
            init_project(name, template);
        }
        Some(Commands::Create {
            name,
            template,
            workspace,
        }) => {
            create_sub_project(workspace, name, template);
        }
        Some(Commands::List) => {
            list_project();
        }
        None => {
            println!("请使用 --help 查看使用说明");
        }
    }
}
