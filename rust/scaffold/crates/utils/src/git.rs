use anyhow::{Context, Result};
use std::process::Command;

pub struct GitRepo {
    url: String,
}

impl GitRepo {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub fn clone_to(&self, target_dir: &str) -> Result<()> {
        let output = Command::new("git")
            .arg("clone")
            .arg(&self.url)
            .arg(target_dir)
            .output()
            .with_context(|| format!("Failed to execute git clone command"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git clone failed: {}", stderr);
        }

        Ok(())
    }
}
