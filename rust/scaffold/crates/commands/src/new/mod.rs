pub mod handler;

use anyhow::Result;
use handler::NewHandler;

/// 创建新项目
pub fn new(template_name: Option<String>, project_name: Option<String>) -> Result<()> {
    let handler = NewHandler::new();
    handler.execute(template_name, project_name)
}
