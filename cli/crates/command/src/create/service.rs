
use colored::*;
use common::template::Template;

use super::command::{
    command_project_name,
    command_project_template
};

/// 获取项目名称
pub fn get_project_name(name: Option<String>) -> String {
    // 创建输入提示
    let project_name = match name {
        Some(name) => {
            println!("🚀 {}: {}", "项目名称".green(), name);
            name
        },
        None => command_project_name(),
    };

    project_name
}

/// 获取项目模板
pub fn get_project_template(template: String) -> Template{
    let template_info = if template.is_empty() {
        command_project_template()
    } else {
        match Template::get_by_name(&template) {
            Some(t) => t,
            None => {
                panic!("{} 模板 '{}' 不存在", "✗".red(), template)
            }
        }
    };

    template_info
}
