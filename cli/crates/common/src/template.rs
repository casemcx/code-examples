/// 模板类型
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateType {
    /// Monorepo 基座
    Monorepo,
    /// 单体项目
    Standalone,
    /// Monorepo 子应用
    App,
}

#[derive(Debug, Clone)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub template_type: TemplateType,
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
    /// 获取所有模板类型（用于列表展示）
    pub fn all() -> Vec<Template> {
        vec![
            // React 模板
            Template::React(TemplateInfo {
                name: "React".to_string(),
                description: "React Monorepo 基座".to_string(),
                template_type: TemplateType::Monorepo,
                path: "templates/react/monorepo".to_string(),
            }),
            Template::React(TemplateInfo {
                name: "React".to_string(),
                description: "React 单体项目".to_string(),
                template_type: TemplateType::Standalone,
                path: "templates/react/standalone".to_string(),
            }),
            Template::React(TemplateInfo {
                name: "React".to_string(),
                description: "React 应用模板".to_string(),
                template_type: TemplateType::App,
                path: "templates/react/app".to_string(),
            }),
            // Vue 模板
            Template::Vue(TemplateInfo {
                name: "Vue".to_string(),
                description: "Vue Monorepo 基座".to_string(),
                template_type: TemplateType::Monorepo,
                path: "templates/vue/monorepo".to_string(),
            }),
            Template::Vue(TemplateInfo {
                name: "Vue".to_string(),
                description: "Vue 单体项目".to_string(),
                template_type: TemplateType::Standalone,
                path: "templates/vue/standalone".to_string(),
            }),
            Template::Vue(TemplateInfo {
                name: "Vue".to_string(),
                description: "Vue 应用模板".to_string(),
                template_type: TemplateType::App,
                path: "templates/vue/app".to_string(),
            }),
            // Nest 模板
            Template::Nest(TemplateInfo {
                name: "Nest".to_string(),
                description: "NestJS 单体项目".to_string(),
                template_type: TemplateType::Standalone,
                path: "templates/nest/standalone".to_string(),
            }),
            // Express 模板
            Template::Express(TemplateInfo {
                name: "Express".to_string(),
                description: "Express 单体项目".to_string(),
                template_type: TemplateType::Standalone,
                path: "templates/express/standalone".to_string(),
            }),
        ]
    }

    /// 获取所有框架名称（去重）
    pub fn all_frameworks() -> Vec<String> {
        vec![
            "React".to_string(),
            "Vue".to_string(),
            "Nest".to_string(),
            "Express".to_string(),
        ]
    }

    /// 根据框架名称获取 Monorepo 模板
    pub fn get_monorepo(name: &str) -> Option<Template> {
        Self::all()
            .into_iter()
            .find(|t| {
                let info = Self::transform_to_info(t);
                info.name == name && info.template_type == TemplateType::Monorepo
            })
    }

    /// 根据框架名称获取 Standalone 模板
    pub fn get_standalone(name: &str) -> Option<Template> {
        Self::all()
            .into_iter()
            .find(|t| {
                let info = Self::transform_to_info(t);
                info.name == name && info.template_type == TemplateType::Standalone
            })
    }

    /// 根据框架名称获取 App 模板
    pub fn get_app(name: &str) -> Option<Template> {
        Self::all()
            .into_iter()
            .find(|t| {
                let info = Self::transform_to_info(t);
                info.name == name && info.template_type == TemplateType::App
            })
    }

    /// 根据名称获取模板（返回第一个匹配的，用于向后兼容）
    pub fn get_by_name(name: &str) -> Option<Template> {
        Self::all()
            .into_iter()
            .find(|t| Self::transform_to_info(t).name == name)
    }

    /// 检查框架是否支持 monorepo
    pub fn supports_monorepo(name: &str) -> bool {
        Self::get_monorepo(name).is_some()
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