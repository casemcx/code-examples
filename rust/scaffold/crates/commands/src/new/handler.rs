use std::fs;
use std::path::Path;
use anyhow::{Result, Context};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use fs_extra::dir::{copy, CopyOptions};
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::env;
use utils::{info, warn};
use crate::config;

#[derive(Debug, Clone, Deserialize)]
pub struct ScaffoldConfig {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub language: Option<String>,
    pub tags: Option<Vec<String>>,
    pub path: String,
}

pub struct NewHandler {
    home_dir: String,
    current_dir: String,
}

impl NewHandler {
    pub fn new() -> Self {
        let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let current_dir = env::current_dir()
            .unwrap_or_else(|_| Path::new(".").to_path_buf())
            .to_string_lossy()
            .to_string();
        Self { home_dir, current_dir }
    }

    pub fn execute(&self, template_name: Option<String>, project_name: Option<String>) -> Result<()> {
        // 日志级别测试消息 - 用于测试日志过滤

        // 1. 读取配置文件
        let config_file = format!("{}/{}", self.home_dir, config::HOME_SCAFFOLD_CONFIG);
        let configs = self.read_scaffold_configs(&config_file)?;

        if configs.is_empty() {
            anyhow::bail!("未找到任何模板配置，请先运行 'statr update' 更新模板仓库");
        }

        // 2. 选择模板
        let selected_config = if let Some(name) = template_name {
            self.find_config_by_name(&configs, &name)?
        } else {
            self.select_config_interactive(&configs)?
        };

        info!(template = %selected_config.name, path = %selected_config.path, "已选择模板");

        // 3. 获取项目名称
        let project_name = if let Some(name) = project_name {
            name
        } else {
            self.ask_project_name(&selected_config.name)?
        };

        // 4. 检查目标目录
        let target_dir = format!("{}/{}", self.current_dir, project_name);
        if Path::new(&target_dir).exists() {
            anyhow::bail!("目录 {} 已存在", target_dir);
        }

        // 验证模板路径是否存在
        if !Path::new(&selected_config.path).exists() {
            anyhow::bail!("模板路径不存在: {}", selected_config.path);
        }

        // 5. 复制模板到目标目录
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .expect("Invalid template")
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        );
        spinner.set_message(format!("正在复制模板到 {}...", target_dir));
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));

        self.copy_template(&selected_config.path, &target_dir)?;

        spinner.finish_with_message(format!("模板已复制到 {}", target_dir));

        // 6. 初始化 git 仓库（可选）
        info!("正在初始化 Git 仓库...");
        self.init_git_repo(&target_dir)?;
        info!("Git 仓库初始化完成");

        info!(project = %project_name, "项目创建完成");
        info!("使用以下命令开始:");
        info!("  cd {}", project_name);
        info!("  # 查看项目文件");
        Ok(())
    }

    fn read_scaffold_configs(&self, config_file: &str) -> Result<Vec<ScaffoldConfig>> {
        if !Path::new(config_file).exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(config_file)
            .with_context(|| format!("读取配置文件 {} 失败", config_file))?;

        let configs: Vec<ScaffoldConfig> = serde_json::from_str(&content)
            .with_context(|| format!("解析配置文件 {} 失败", config_file))?;

        Ok(configs)
    }

    fn find_config_by_name(&self, configs: &[ScaffoldConfig], name: &str) -> Result<ScaffoldConfig> {
        configs
            .iter()
            .find(|c| c.name == name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("未找到名为 '{}' 的模板", name))
    }

    fn select_config_interactive(&self, configs: &[ScaffoldConfig]) -> Result<ScaffoldConfig> {
        let items: Vec<String> = configs
            .iter()
            .map(|c| {
                format!(
                    "{} - {}",
                    c.name.cyan(),
                    c.description.as_ref().unwrap_or(&"无描述".to_string()).dimmed()
                )
            })
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("请选择一个模板")
            .items(&items)
            .default(0)
            .interact()?;

        Ok(configs[selection].clone())
    }

    fn ask_project_name(&self, default_name: &str) -> Result<String> {
        use dialoguer::Input;

        let name = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("请输入项目名称")
            .default(default_name.to_string())
            .interact()?;

        Ok(name)
    }

    fn copy_template(&self, source: &str, target: &str) -> Result<()> {
        info!(source = %source, target = %target, "开始复制模板");

        // 创建目标目录
        fs::create_dir_all(target)
            .with_context(|| format!("创建目标目录 {} 失败", target))?;

        // 复制选项
        let mut options = CopyOptions::new();
        options.content_only = true; // 只复制内容，不复制源目录本身

        copy(source, target, &options)
            .with_context(|| format!("复制模板从 {} 到 {} 失败", source, target))?;

        Ok(())
    }

    fn init_git_repo(&self, project_dir: &str) -> Result<()> {
        use std::process::Command;

        let output = Command::new("git")
            .arg("init")
            .current_dir(project_dir)
            .output();

        match output {
            Ok(output) if output.status.success() => Ok(()),
            _ => {
                // Git 初始化失败不是致命错误，只是警告
                warn!("Git 初始化失败或未安装 Git");
                Ok(())
            }
        }
    }
}

impl Default for NewHandler {
    fn default() -> Self {
        Self::new()
    }
}
