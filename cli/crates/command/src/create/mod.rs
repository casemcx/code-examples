use common::template::Template;
use utils::print;

use service::{
    get_project_name,
    get_project_template
};

pub mod command;
mod vue;
mod service;


pub fn crate_project(name: Option<String>, template_name: String) {
    print::print_title();
    println!();
    let project_name = get_project_name(name);
    println!();
    let template: Template = get_project_template(template_name);

    match template {
        Template::Vue(template_info) => { // 添加其他明确支持的模板
            vue::create_vue(project_name, template_info);
        }
        
        // 更规范的兜底分支写法
        _ => {
            // 使用标准错误输出
            eprintln!("Error: 暂不支持该模板");
            // 添加错误处理（例如退出进程）
            std::process::exit(1);
        }
    }

   
}