use dialoguer::{Select, theme::ColorfulTheme};
use colored::*;
use common::template::Template;

/// 获取项目模板
pub fn get_project_template() -> Template {
    let templates = Template::all();
    let items: Vec<String> = templates
        .iter()
        .map(|t| match t {
            Template::React(info) => info.description.clone(),
            Template::Vue(info) => info.description.clone(),
            Template::Nest(info) => info.description.clone(),
            Template::Express(info) => info.description.clone(),
        })
        .collect();

    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .with_prompt(format!("{}", "选择项目模板".green()))
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    templates[selection].clone()
}