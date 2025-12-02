use dialoguer::{console::style, theme::ColorfulTheme, Confirm, Input, Select};
use colored::*;


use common::template::Template;


/// 获取项目名称
pub fn command_project_name() -> String {
    let mut theme: ColorfulTheme = ColorfulTheme::default();
    theme.success_prefix = style("🚀".to_string());
    theme.prompt_prefix = style("🚀".to_string());


    let input = Input::<String>::with_theme(&theme)
        .with_prompt(format!("{}", "项目名称".green()))
        .default("web-apps".into())
        .interact()
        .unwrap();

    input
}

/// 获取项目模版
pub fn command_project_template() -> Template { 
    let templates = Template::all();
    let mut theme: ColorfulTheme = ColorfulTheme::default();
    theme.success_prefix = style("✨".to_string());
    theme.prompt_prefix = style("✨".to_string());
    theme.active_item_prefix = style("➤ ".to_string()).for_stderr().green();

    let items: Vec<String> = templates
        .iter()
        .map(|t| match t {
            Template::React(info) => info.description.clone(),
            Template::Vue(info) => info.description.clone(),
            Template::Nest(info) => info.description.clone(),
            Template::Express(info) => info.description.clone(),
        })
        .collect();

    let selection = Select::with_theme(&theme)
        .with_prompt(format!("{}", "选择项目模板".green()))
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    templates[selection].clone()
}

/// 是否是monorepo基座
pub fn command_use_monorepo () -> bool {
    let mut theme: ColorfulTheme = ColorfulTheme::default();
    theme.success_prefix = style("🚀".to_string());
    theme.prompt_prefix = style("🚀".to_string());

    let input = Confirm::with_theme(&theme)
        .with_prompt("是否需要monorepo")
        .default(true)
        .interact()
        .unwrap(); 

    input
}

/// 判断是否需要测试框架
pub fn command_use_test () -> bool {
    let mut theme: ColorfulTheme = ColorfulTheme::default();
    theme.success_prefix = style("🚀".to_string());
    theme.prompt_prefix = style("🚀".to_string());

    let input: bool = Confirm::with_theme(&theme)
        .with_prompt("是否需要测试框架")
        .default(true)
        .interact()
        .unwrap();

    input
}

/// 获取子仓库名称
pub fn command_working_name () -> String {
    let mut theme: ColorfulTheme = ColorfulTheme::default();
    theme.success_prefix = style("🚀".to_string());
    theme.prompt_prefix = style("🚀".to_string());


    let input = Input::<String>::with_theme(&theme)
        .with_prompt(format!("{}", "子仓库名称".green()))
        .default("apps".into())
        .interact()
        .unwrap();

    input
}