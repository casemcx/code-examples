use dialoguer::{console::style, theme::ColorfulTheme, Input};
use colored::*;
use common::template::Template;
use utils::monorepo;

/// 获取 workspace 目录
pub fn get_workspace(workspace: Option<String>) -> String {
    match workspace {
        Some(ws) => {
            // 验证 workspace 是否存在
            if monorepo::workspace_exists(&ws) {
                println!("📁 {}: {}", "Workspace".green(), ws.cyan());
                ws
            } else {
                eprintln!("⚠ Workspace 目录 '{}' 不存在，使用默认值", ws.yellow());
                let default_ws = monorepo::get_default_workspace();
                println!("📁 {}: {}", "Workspace".green(), default_ws.cyan());
                default_ws
            }
        }
        None => {
            let default_ws = monorepo::get_default_workspace();

            let mut theme = ColorfulTheme::default();
            theme.success_prefix = style("📁".to_string());
            theme.prompt_prefix = style("📁".to_string());

            Input::<String>::with_theme(&theme)
                .with_prompt(format!("{}", "Workspace 目录".green()))
                .default(default_ws)
                .interact()
                .unwrap()
        }
    }
}

/// 获取应用名称
pub fn get_app_name(name: Option<String>) -> String {
    match name {
        Some(n) => {
            println!("📦 {}: {}", "应用名称".green(), n.cyan());
            n
        }
        None => {
            let mut theme = ColorfulTheme::default();
            theme.success_prefix = style("📦".to_string());
            theme.prompt_prefix = style("📦".to_string());

            Input::<String>::with_theme(&theme)
                .with_prompt(format!("{}", "应用名称".green()))
                .default("app".into())
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
