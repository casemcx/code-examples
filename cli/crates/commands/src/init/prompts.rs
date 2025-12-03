use dialoguer::{console::style, theme::ColorfulTheme, Confirm, Input, Select};
use colored::*;
use common::template::Template;

/// 选择框架
pub fn select_framework() -> String {
    let mut theme = ColorfulTheme::default();
    theme.success_prefix = style("✨".to_string());
    theme.prompt_prefix = style("✨".to_string());
    theme.active_item_prefix = style("➤ ".to_string()).for_stderr().green();

    let frameworks = Template::all_frameworks();

    let selection = Select::with_theme(&theme)
        .with_prompt(format!("{}", "选择项目框架".green()))
        .items(&frameworks)
        .default(0)
        .interact()
        .unwrap();

    frameworks[selection].clone()
}

/// 询问是否使用 monorepo
pub fn ask_use_monorepo() -> bool {
    let mut theme = ColorfulTheme::default();
    theme.success_prefix = style("🚀".to_string());
    theme.prompt_prefix = style("🚀".to_string());

    Confirm::with_theme(&theme)
        .with_prompt(format!("{}", "是否使用 Monorepo 架构?".green()))
        .default(true)
        .interact()
        .unwrap()
}

/// 询问是否创建初始应用
pub fn ask_create_initial_app() -> bool {
    let mut theme = ColorfulTheme::default();
    theme.success_prefix = style("✨".to_string());
    theme.prompt_prefix = style("✨".to_string());

    Confirm::with_theme(&theme)
        .with_prompt(format!("{}", "是否创建一个初始应用?".green()))
        .default(true)
        .interact()
        .unwrap()
}

/// 获取应用名称
pub fn get_app_name() -> String {
    let mut theme = ColorfulTheme::default();
    theme.success_prefix = style("📦".to_string());
    theme.prompt_prefix = style("📦".to_string());

    Input::<String>::with_theme(&theme)
        .with_prompt(format!("{}", "应用名称".green()))
        .default("web-app".into())
        .interact()
        .unwrap()
}

/// 获取 workspace 名称
pub fn get_workspace_name() -> String {
    let mut theme = ColorfulTheme::default();
    theme.success_prefix = style("📁".to_string());
    theme.prompt_prefix = style("📁".to_string());

    Input::<String>::with_theme(&theme)
        .with_prompt(format!("{}", "Workspace 目录".green()))
        .default("apps".into())
        .interact()
        .unwrap()
}
