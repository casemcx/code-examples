use common::template::TemplateInfo;
use command::select_vue;
use utils::copy;

mod command;

/// 创建vue项目
pub fn create_vue(project_name: String, template_info: TemplateInfo) {
    let vue = select_vue();
    let working_name = match vue.working_name {
        Some(name) => name,
        None => "".to_string(),
    };

    if vue.use_monorepo {
        let target: String = format!("{}/{}", template_info.path, "monorepo");
        let target_to = project_name.clone();
        // copy命令
        copy::copy_dir(target, target_to).unwrap();

        let sub_target = format!("{}/{}", template_info.path, "apps");
        let sub_to: String = format!("{}/{}", project_name.clone(), working_name);

        copy::copy_dir(sub_target, sub_to).unwrap();
    }
}