pub mod handler;

use anyhow::Result;
use handler::UpdateHandler;

/// 更新本地代码仓库
pub fn update() -> Result<()> {
    let handler = UpdateHandler::new();
    handler.execute()
}