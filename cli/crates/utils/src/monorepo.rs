use std::path::Path;

/// 检测当前目录是否是 monorepo
pub fn is_monorepo() -> bool {
    Path::new("pnpm-workspace.yaml").exists()
        || Path::new("lerna.json").exists()
        || Path::new("rush.json").exists()
        || Path::new("pnpm-lock.yaml").exists()
}

/// 获取默认的 workspace 目录
pub fn get_default_workspace() -> String {
    if Path::new("apps").exists() {
        String::from("apps")
    } else if Path::new("packages").exists() {
        String::from("packages")
    } else {
        String::from("apps")
    }
}

/// 检查指定的 workspace 目录是否存在
pub fn workspace_exists(workspace: &str) -> bool {
    Path::new(workspace).exists() && Path::new(workspace).is_dir()
}
