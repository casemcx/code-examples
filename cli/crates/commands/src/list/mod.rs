use colored::*;
use common::template::Template;

pub fn list_project() {
    let temps = Template::all();
    temps.iter().for_each(|temp| {
        match temp {
            Template::React(info) |
            Template::Vue(info) |
            Template::Nest(info) |
            Template::Express(info) => {
                println!("📦 {}: {}", info.name.green(), info.description);
                println!();
            }
        }
    });
}