use std::env;
use std::error::Error;
use fs_extra::dir::{copy, CopyOptions};
use lazy_static::lazy_static;

lazy_static! {
    static ref COPY_OPTIONS: CopyOptions = {
        let mut options = CopyOptions::new();
        options.content_only = false;          // 复制目录元信息
        options.overwrite = true;              // 覆盖已存在文件
        options.copy_inside = true;            // 保持目录结构
        options.buffer_size = 1024 * 1024 * 5; // 5MB缓冲区
        options
    };
}

pub fn copy_dir(source: String, to: String) -> Result<(), Box<dyn Error>> {
    // 获取可执行文件路径并转换为 PathBuf
    let cwd = env::current_dir()
        .map_err(|e| format!("无法获取可执行路径: {}", e))?;

    // 构建完整路径
    let source_path = cwd.join(source);
    let dest_path = cwd.join(to);

    // 验证源路径存在
    if !source_path.exists() {
        return Err(format!("源目录不存在: {}", source_path.display()).into());
    }

    // 执行复制操作
    copy(&source_path, &dest_path, &COPY_OPTIONS)
        .map_err(|e| format!("目录复制失败: {}", e))?;

    Ok(())
}

