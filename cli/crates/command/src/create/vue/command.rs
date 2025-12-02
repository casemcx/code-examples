use dialoguer::{console::style, theme::ColorfulTheme, Confirm, Input};
use colored::*;

use common::project::VueProject;

fn get_use_monorepo () -> bool {

    let mut theme: ColorfulTheme = ColorfulTheme::default();
    theme.success_prefix = style("✨".to_string());
    theme.prompt_prefix = style("✨".to_string());

    let input = Confirm::with_theme(&theme)
        .with_prompt(format!("{}", "是否需要monorepo".green()))
        .default(true)
        .interact()
        .unwrap(); 

    input
}


fn get_use_test() -> bool {
    let mut theme: ColorfulTheme = ColorfulTheme::default();
    theme.success_prefix = style("✨".to_string());
    theme.prompt_prefix = style("✨".to_string());

    let input: bool = Confirm::with_theme(&theme)
        .with_prompt(format!("{}", "是否需要测试框架".green()))
        .default(true)
        .interact()
        .unwrap();

    input
}

fn command_working_name () -> String {
    let mut theme: ColorfulTheme = ColorfulTheme::default();
    theme.success_prefix = style("✨".to_string());
    theme.prompt_prefix = style("✨".to_string());


    let input = Input::<String>::with_theme(&theme)
        .with_prompt(format!("{}", "working space".green()))
        .default("apps".into())
        .interact()
        .unwrap();

    input
}

fn get_use_sub_project() -> bool {
    let mut theme: ColorfulTheme = ColorfulTheme::default();
    theme.success_prefix = style("✨".to_string());
    theme.prompt_prefix = style("✨".to_string());

    let input: bool = Confirm::with_theme(&theme)
        .with_prompt(format!("{}", "是否需要一个子模块".green()))
        .default(true)
        .interact()
        .unwrap();

    input
}

fn command_sub_project_name () -> String {
    let mut theme: ColorfulTheme = ColorfulTheme::default();
    theme.success_prefix = style("✨".to_string());
    theme.prompt_prefix = style("✨".to_string());


    let input = Input::<String>::with_theme(&theme)
        .with_prompt(format!("{}", "子模块".green()))
        .default("apps".into())
        .interact()
        .unwrap();

    input
}


pub fn select_vue () -> VueProject {
    println!();

    let use_monorepo = get_use_monorepo();

    let mut working_name: Option<String> = None;  // 明确类型注解

    let mut use_sub_project: bool = false;

    let mut sub_project_name: Option<String> = None;

    if use_monorepo { 
        println!();
        working_name = Some(command_working_name());  // 使用Some包裹值


        println!();
        use_sub_project = get_use_sub_project();

        if use_sub_project {

            println!();
            sub_project_name = Some(command_sub_project_name());
        }
    }

    
    println!();
    let use_test: bool = get_use_test();

    return VueProject {
        use_monorepo,
        use_test,
        working_name,
        use_sub_project,
        sub_project_name
    }
}