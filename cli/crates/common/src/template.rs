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

/// 简化模板创建的宏
macro_rules! template {
    ($variant:ident, $name:expr, $desc:expr, $type:ident, $path:expr) => {
        Template::$variant(TemplateInfo {
            name: $name.to_string(),
            description: $desc.to_string(),
            template_type: TemplateType::$type,
            path: format!("templates/{}", $path),
        })
    };
}

impl Template {
    /// 获取所有模板类型（用于列表展示）
    pub fn all() -> Vec<Template> {
        vec![
            // React 模板
            template!(React, "React", "React Monorepo 基座", Monorepo, "react/monorepo"),
            template!(React, "React", "React 单体项目", Standalone, "react/standalone"),
            template!(React, "React", "React 应用模板", App, "react/app"),

            // Vue 模板
            template!(Vue, "Vue", "Vue Monorepo 基座", Monorepo, "vue/monorepo"),
            template!(Vue, "Vue", "Vue 单体项目", Standalone, "vue/standalone"),
            template!(Vue, "Vue", "Vue 应用模板", App, "vue/app"),

            // Nest 模板
            template!(Nest, "Nest", "NestJS 单体项目", Standalone, "nest/standalone"),

            // Express 模板
            template!(Express, "Express", "Express 单体项目", Standalone, "express/standalone"),
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