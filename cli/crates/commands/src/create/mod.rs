use colored::*;
use common::template::Template;
use utils::{copy, monorepo, print};

mod prompts;
mod handler;

use handler::{get_app_name, get_framework_name, get_workspace};

/// 在 monorepo 中创建新的子项目
pub fn create_sub_project(
    workspace: Option<String>,
    name: Option<String>,
    template_name: String,
) {
    print::print_title();
    println!();

    // 1. 检测是否在 monorepo 中
    if !monorepo::is_monorepo() {
        eprintln!("✗ {} 当前目录不是 monorepo 项目", "错误:".red());
        eprintln!("  提示: 使用 {} 创建新项目", "scaffold init".cyan());
        std::process::exit(1);
    }

    println!("✓ 检测到 Monorepo 项目");
    println!();

    // 2. 获取 workspace 目录
    let workspace_dir = get_workspace(workspace);
    println!();

    // 3. 获取应用名称
    let app_name = get_app_name(name);
    println!();

    // 4. 获取框架
    let framework = if template_name.is_empty() {
        prompts::select_framework()
    } else {
        get_framework_name(template_name)
    };
    println!();

    // 5. 创建子项目
    create_app(&workspace_dir, &app_name, &framework);
}

/// 创建子应用
fn create_app(workspace: &str, app_name: &str, framework: &str) {
    println!("📦 正在创建 {} 子应用: {}", framework.green(), app_name.cyan());

    // 获取 app 模板
    let template = Template::get_app(framework);

    match template {
        Some(tpl) => {
            let template_info = Template::transform_to_info(&tpl);

            // 目标路径：workspace/app_name
            let target_path = format!("{}/{}", workspace, app_name);

            match copy::copy_dir(template_info.path.clone(), target_path.clone()) {
                Ok(_) => {
                    println!("✓ 子应用创建成功");
                    println!();
                    println!("🎉 子应用初始化完成!");
                    println!();
                    println!("{}", "后续步骤:".green().bold());
                    println!("  cd {}", target_path.cyan());
                    println!("  {} 查看项目文档", "cat README.md |".cyan());
                }
                Err(e) => {
                    eprintln!("✗ 子应用创建失败: {}", e.to_string().red());
                    std::process::exit(1);
                }
            }
        }
        None => {
            eprintln!("✗ {} 不支持创建子应用", framework.red());
            eprintln!("  提示: 该框架可能不支持 monorepo 模式");
            std::process::exit(1);
        }
    }
}

// 保留旧的函数用于向后兼容（废弃）
#[deprecated(note = "请使用 init::init_project 创建新项目，或使用 create_sub_project 在 monorepo 中创建子项目")]
pub fn crate_project(_name: Option<String>, _template_name: String) {
    eprintln!("✗ {} 'create' 命令已更新", "提示:".yellow());
    eprintln!("  - 创建新项目: {}", "scaffold init".cyan());
    eprintln!("  - 在 monorepo 中创建子项目: {}", "scaffold create".cyan());
    std::process::exit(1);
}
