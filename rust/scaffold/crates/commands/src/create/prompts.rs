use dialoguer::{console::style, theme::ColorfulTheme, Select};
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
