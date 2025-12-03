use dialoguer::{console::style, theme::ColorfulTheme, Input};
use colored::*;
use common::template::Template;

/// 获取项目名称
pub fn get_project_name(name: Option<String>) -> String {
    match name {
        Some(name) => {
            println!("🚀 {}: {}", "项目名称".green(), name.cyan());
            name
        }
        None => {
            let mut theme = ColorfulTheme::default();
            theme.success_prefix = style("🚀".to_string());
            theme.prompt_prefix = style("🚀".to_string());

            Input::<String>::with_theme(&theme)
                .with_prompt(format!("{}", "项目名称".green()))
                .default("my-project".into())
                .interact()
                .unwrap()
        }
    }
}

/// 获取框架名称
pub fn get_framework_name(template_name: String) -> String {
    // 验证框架名称是否有效
    let frameworks = Template::all_frameworks();

    if frameworks.contains(&template_name) {
        println!("✨ {}: {}", "选择框架".green(), template_name.cyan());
        template_name
    } else {
        eprintln!("✗ {} 框架 '{}' 不存在", "错误:".red(), template_name);
        eprintln!("  可用框架: {}", frameworks.join(", "));
        std::process::exit(1);
    }
}
