use clap::Parser;

use cli::command::cli::{
    Cli,
    Commands
};
use command::{
    create::crate_project,
    list::list_project,
};

fn main() {
    let app = Cli::parse();

    match app.command {
        Some(Commands::Create { name, template }) => {
            crate_project(name, template)
        }
        Some(Commands::List) => {
            list_project();
        }
        None => {
            println!("请使用 --help 查看使用说明");
        }
    }

}