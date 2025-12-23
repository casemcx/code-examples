use std::fs;
use std::path::Path;
use anyhow::{Result, Context};
use fs_extra::dir::remove;
use serde::{Deserialize, Serialize};
use indicatif::{ProgressBar, ProgressStyle};
use utils::{git::GitRepo, info};
use crate::config;

pub struct UpdateHandler {
    home_dir: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ScaffoldConfig {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub language: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(skip_deserializing)]
    #[serde(default = "default_path")]
    pub path: String,
}

fn default_path() -> String {
    String::new()
}

impl UpdateHandler {
    pub fn new() -> Self {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        Self { home_dir }
    }

    pub fn execute(&self) -> Result<()> {
        info!("开始更新代码仓库...");

        // 1. 下载GitHub代码到临时目录
        let temp_dir = format!("{}/{}/{}{}", self.home_dir, config::SCAFFOLD_DIR_NAME, config::CODES_DIR_NAME, config::TEMP_DIR_SUFFIX);
        let target_dir = format!("{}/{}/{}", self.home_dir, config::SCAFFOLD_DIR_NAME, config::CODES_DIR_NAME);

        // 确保scaffold目录存在
        let scaffold_dir = format!("{}/{}", self.home_dir, config::SCAFFOLD_DIR_NAME);
        if !Path::new(&scaffold_dir).exists() {
            fs::create_dir_all(&scaffold_dir)
                .with_context(|| format!("创建{}目录失败: {}", config::SCAFFOLD_DIR_NAME, scaffold_dir))?;
        }

        // 2. 克隆GitHub仓库到临时目录
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .expect("Invalid template")
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        );
        spinner.set_message(format!("正在下载GitHub代码到 {}...", temp_dir));
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));
        self.clone_repository(&temp_dir)?;
        spinner.finish_with_message("代码下载完成");

        // 3. 删除旧的codes目录
        if Path::new(&target_dir).exists() {
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.yellow} {msg}")
                    .expect("Invalid template")
                    .tick_strings(&["◐", "◓", "◑", "◒"])
            );
            spinner.set_message("正在删除旧的codes目录...");
            spinner.enable_steady_tick(std::time::Duration::from_millis(150));
            self.remove_directory(&target_dir)?;
            spinner.finish_with_message("旧目录已删除");
        }

        // 4. 重命名临时目录为codes
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} {msg}")
                .expect("Invalid template")
                .tick_strings(&["◰", "◳", "◲", "◱"])
        );
        spinner.set_message("正在重命名目录...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(120));
        self.rename_directory(&temp_dir, &target_dir)?;
        spinner.finish_with_message("目录重命名完成");

        // 5. 解析codes目录下的所有子目录，获取scaffold.json
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.magenta} {msg}")
                .expect("Invalid template")
                .tick_strings(&["⣾", "⣽", "⣻", "⢿"])
        );
        spinner.set_message("正在解析scaffold配置...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));
        let scaffold_configs = self.parse_scaffold_configs(&target_dir)?;
        spinner.finish_with_message(format!("找到 {} 个scaffold配置", scaffold_configs.len()));

        // 6. 写入配置到 ~/.scaffold/config.json
        if !scaffold_configs.is_empty() {
            let scaffold_dir = format!("{}/{}", self.home_dir, config::HOME_SCAFFOLD_DIR);
            let config_file = format!("{}/{}", self.home_dir, config::HOME_SCAFFOLD_CONFIG);
            spinner.set_message("正在保存配置到文件...");
            spinner.enable_steady_tick(std::time::Duration::from_millis(50));

            self.write_scaffold_configs(&scaffold_dir, &config_file, &scaffold_configs)?;
            spinner.finish_with_message(format!("配置已保存到 {}", config_file));
        }

        for config in &scaffold_configs {
            info!(
                name = %config.name,
                description = %config.description.as_ref().unwrap_or(&"无描述".to_string()),
                path = %config.path,
                "找到模板配置"
            );
        }

        info!("代码仓库更新完成！");
        Ok(())
    }

    fn write_scaffold_configs(&self, scaffold_dir: &str, config_file: &str, configs: &[ScaffoldConfig]) -> Result<()> {
        // 确保 .scaffold 目录存在
        if !Path::new(scaffold_dir).exists() {
            fs::create_dir_all(scaffold_dir)
                .with_context(|| format!("创建目录 {} 失败", scaffold_dir))?;
        }

        let json_content = serde_json::to_string_pretty(configs)
            .with_context(|| "序列化配置失败")?;

        fs::write(config_file, json_content)
            .with_context(|| format!("写入配置文件 {} 失败", config_file))?;

        Ok(())
    }

    fn clone_repository(&self, target_dir: &str) -> Result<()> {
        // 如果临时目录已存在，先删除
        if Path::new(target_dir).exists() {
            self.remove_directory(target_dir)?;
        }

        // 使用GitRepo克隆代码仓库
        let repo = GitRepo::new(config::GITHUB_REPO_URL);
        repo.clone_to(target_dir)
            .with_context(|| format!("克隆仓库到 {} 失败", target_dir))?;

        Ok(())
    }

    fn remove_directory(&self, dir_path: &str) -> Result<()> {
        remove(dir_path)
            .with_context(|| format!("删除目录 {} 失败", dir_path))?;
        Ok(())
    }

    fn rename_directory(&self, from: &str, to: &str) -> Result<()> {
        fs::rename(from, to)
            .with_context(|| format!("重命名目录从 {} 到 {} 失败", from, to))?;
        Ok(())
    }

    fn parse_scaffold_configs(&self, codes_dir: &str) -> Result<Vec<ScaffoldConfig>> {
        let mut configs = Vec::new();
        let entries = fs::read_dir(codes_dir)
            .with_context(|| format!("读取目录 {} 失败", codes_dir))?;

        for entry in entries {
            let entry = entry.with_context(|| "读取目录条目失败")?;
            let path = entry.path();

            // 只处理一级子目录
            if path.is_dir() {
                if let Some(config) = self.read_scaffold_config(&path)? {
                    configs.push(config);
                }
            }
        }

        Ok(configs)
    }

    fn read_scaffold_config(&self, dir_path: &Path) -> Result<Option<ScaffoldConfig>> {
        let config_path = dir_path.join(config::SCAFFOLD_CONFIG_FILE);

        // 如果配置文件不存在，返回 None
        if !config_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("读取文件 {} 失败", config_path.display()))?;

        let mut config: ScaffoldConfig = serde_json::from_str(&content)
            .with_context(|| format!("解析{}失败: {}", config::SCAFFOLD_CONFIG_FILE, config_path.display()))?;

        // 设置模板路径
        config.path = dir_path.to_string_lossy().to_string();

        Ok(Some(config))
    }
}


impl Default for UpdateHandler {
    fn default() -> Self {
        Self::new()
    }
}