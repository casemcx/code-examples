use dialoguer::Input;
use colored::*;

/// 获取项目名称
pub fn get_project_name() -> String {
    // 创建输入提示
    let input = Input::<String>::new()
        .with_prompt(format!("🚀 {}", "项目名称".green()))
        .default("web-apps".into())
        .interact()
        .unwrap();

    input
}