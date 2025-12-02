#[derive(Debug, Clone)]
pub struct VueProject {
    /// 是否需要monorepo
    pub use_monorepo: bool,
    /// 子仓库名称
    pub working_name: Option<String>,
    /// 是否需要测试框架
    pub use_test: bool,

    /// 是否需要一个基础模板
    pub use_sub_project: bool,
    
    /// 是否需要一个基础模板
    pub sub_project_name: Option<String>,
}