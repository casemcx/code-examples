use colored::*;
use common::template::Template;
use utils::{copy, print};

mod prompts;
mod handler;

use handler::{get_framework_name, get_project_name};

/// 初始化新项目
pub fn init_project(name: Option<String>, template_name: String) {
    print::print_title();
    println!();

    // 1. 获取项目名称
    let project_name = get_project_name(name);
    println!();

    // 2. 获取框架名称
    let framework = if template_name.is_empty() {
        prompts::select_framework()
    } else {
        get_framework_name(template_name)
    };
    println!();

    // 3. 询问是否使用 monorepo
    let use_monorepo = if Template::supports_monorepo(&framework) {
        prompts::ask_use_monorepo()
    } else {
        println!("⚠ {} 暂不支持 monorepo 模式，将创建单体项目", framework.yellow());
        false
    };
    println!();

    if use_monorepo {
        // 创建 monorepo 项目
        create_monorepo_project(&project_name, &framework);
    } else {
        // 创建单体项目
        create_standalone_project(&project_name, &framework);
    }
}

/// 创建 monorepo 项目
fn create_monorepo_project(project_name: &str, framework: &str) {
    println!("🚀 正在创建 {} Monorepo 项目: {}", framework.green(), project_name.cyan());

    // 获取 monorepo 模板
    let template = Template::get_monorepo(framework)
        .expect("无法获取 monorepo 模板");

    let template_info = Template::transform_to_info(&template);

    // 复制 monorepo 基座
    match copy::copy_dir(template_info.path.clone(), project_name.to_string()) {
        Ok(_) => {
            println!("✓ Monorepo 基座创建成功");
            println!();

            // 询问是否创建初始应用
            let create_app = prompts::ask_create_initial_app();
            if create_app {
                println!();
                let app_name = prompts::get_app_name();
                println!();
                let workspace = prompts::get_workspace_name();
                println!();

                // 创建子应用
                create_app_in_monorepo(project_name, &app_name, &workspace, framework);
            }

            println!();
            println!("🎉 {} 项目初始化完成!", "Monorepo".green());
            print_next_steps(project_name, true);
        }
        Err(e) => {
            eprintln!("✗ {} 创建失败: {}", "Monorepo".red(), e);
            std::process::exit(1);
        }
    }
}

/// 创建单体项目
fn create_standalone_project(project_name: &str, framework: &str) {
    println!("🚀 正在创建 {} 单体项目: {}", framework.green(), project_name.cyan());

    // 获取 standalone 模板
    let template = Template::get_standalone(framework)
        .expect("无法获取 standalone 模板");

    let template_info = Template::transform_to_info(&template);

    // 复制单体项目模板
    match copy::copy_dir(template_info.path.clone(), project_name.to_string()) {
        Ok(_) => {
            println!("✓ 单体项目创建成功");
            println!();
            println!("🎉 项目初始化完成!");
            print_next_steps(project_name, false);
        }
        Err(e) => {
            eprintln!("✗ 项目创建失败: {}", e.to_string().red());
            std::process::exit(1);
        }
    }
}

/// 在 monorepo 中创建子应用
fn create_app_in_monorepo(project_root: &str, app_name: &str, workspace: &str, framework: &str) {
    println!("📦 正在创建子应用: {}", app_name.cyan());

    // 获取 app 模板
    let template = Template::get_app(framework)
        .expect("无法获取 app 模板");

    let template_info = Template::transform_to_info(&template);

    // 目标路径：project_root/workspace/app_name
    let target_path = format!("{}/{}/{}", project_root, workspace, app_name);

    match copy::copy_dir(template_info.path.clone(), target_path) {
        Ok(_) => {
            println!("✓ 子应用创建成功");
        }
        Err(e) => {
            eprintln!("⚠ 子应用创建失败: {}", e.to_string().yellow());
        }
    }
}

/// 打印后续步骤
fn print_next_steps(project_name: &str, is_monorepo: bool) {
    println!("{}", "后续步骤:".green().bold());
    println!("  cd {}", project_name.cyan());

    if is_monorepo {
        println!("  {} 安装依赖并启动", "pnpm install &&".cyan());
    } else {
        println!("  {} 查看 README.md 了解更多", "cat".cyan());
    }
}
