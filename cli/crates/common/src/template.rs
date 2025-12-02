#[derive(Debug, Clone)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub path: String,
    
}

#[derive(Debug, Clone)]
pub enum Template {
    React(TemplateInfo),
    Vue(TemplateInfo),
    Nest(TemplateInfo),
    Express(TemplateInfo),
}

impl Template {
    pub fn all() -> Vec<Template> {
        vec![
            Template::React(TemplateInfo {
                name: String::from("React"),
                description: String::from("React 项目"),
                path: String::from("templates/react"),
            }),
            Template::Vue(TemplateInfo {
                name: String::from("Vue"),
                description: String::from("Vue 项目"),
                path: String::from("templates/vue"),
            }),
            Template::Nest(TemplateInfo {
                name: String::from("Nest"),
                description: String::from("NestJS 项目"),
                path: String::from("templates/nest"),
            }),
            Template::Express(TemplateInfo {
                name: String::from("Express"),
                description: String::from("Express 项目"),
                path: String::from("templates/express"),
            }),
        ]
    }

    pub fn get_by_name(name: &str) -> Option<Template> {
        Self::all()
            .into_iter()
            .find(|t| match t {
                Template::React(info) => info.name == name,
                Template::Vue(info) => info.name == name,
                Template::Nest(info) => info.name == name,
                Template::Express(info) => info.name == name,
            })
    }

    pub fn transform_to_info(temp: &Template) -> &TemplateInfo {
        match temp {
            Template::React(info) | 
            Template::Vue(info) |
            Template::Nest(info) |
            Template::Express(info) => info
        }
    }
}