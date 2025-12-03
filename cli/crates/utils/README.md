# Utils - 工具函数库

提供项目中通用的工具函数，包括文件操作、终端输出、Monorepo 检测等。

## 职责

- 文件和目录复制操作
- 终端彩色输出和格式化
- Monorepo 项目检测
- 其他通用工具函数

## 依赖

- `fs_extra` - 文件系统扩展操作
- `colored` - 终端颜色输出
- `lazy_static` - 静态变量定义

## 代码结构

```
src/
├── lib.rs          # Crate 入口
├── copy.rs         # 文件复制工具
├── print.rs        # 打印和输出工具
└── monorepo.rs     # Monorepo 相关工具
```

## 模块详解

### copy.rs - 文件复制

提供高性能的目录复制功能。

#### copy_dir()

复制整个目录及其内容。

**函数签名**：
```rust
pub fn copy_dir(source: String, to: String) -> Result<(), Box<dyn Error>>
```

**参数**：
- `source`: 源目录路径（相对于当前工作目录）
- `to`: 目标目录路径（相对于当前工作目录）

**返回**：
- `Ok(())`: 复制成功
- `Err(...)`: 复制失败，包含错误信息

**特性**：
- 使用 5MB 缓冲区优化性能
- 自动覆盖已存在的文件
- 保持目录结构
- 详细的错误提示

**使用示例**：
```rust
use utils::copy;

// 复制模板到项目目录
match copy::copy_dir(
    "templates/vue/monorepo".to_string(),
    "my-project".to_string()
) {
    Ok(_) => println!("✓ 项目创建成功"),
    Err(e) => eprintln!("✗ 创建失败: {}", e),
}
```

**配置选项**：

通过 `COPY_OPTIONS` 静态变量配置：

```rust
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
```

### print.rs - 终端输出

提供美化的终端输出功能。

#### print_gradient()

打印渐变色文字。

**函数签名**：
```rust
pub fn print_gradient(
    word: &str,
    start_color: (u8, u8, u8),
    end_color: (u8, u8, u8)
)
```

**参数**：
- `word`: 要打印的文字
- `start_color`: 起始颜色 RGB 值
- `end_color`: 结束颜色 RGB 值

**示例**：
```rust
use utils::print;

// 打印红色到紫色的渐变文字
print::print_gradient(
    "Welcome to Scaffold!",
    (255, 0, 0),    // 红色
    (148, 0, 211)   // 紫色
);
```

#### print_title()

打印标准的欢迎标题。

**函数签名**：
```rust
pub fn print_title()
```

**示例**：
```rust
use utils::print;

// 打印欢迎标题
print::print_title();
// 输出: Hello, thank can use Front Scaffold!（渐变色）
```

### monorepo.rs - Monorepo 工具

提供 Monorepo 项目检测和管理功能。

#### is_monorepo()

检测当前目录是否是 Monorepo 项目。

**函数签名**：
```rust
pub fn is_monorepo() -> bool
```

**检测规则**：
检查以下文件是否存在：
- `pnpm-workspace.yaml` - pnpm workspace 配置
- `lerna.json` - Lerna 配置
- `rush.json` - Rush 配置
- `pnpm-lock.yaml` - pnpm 锁文件

**返回**：
- `true`: 当前目录是 Monorepo
- `false`: 当前目录不是 Monorepo

**示例**：
```rust
use utils::monorepo;

if monorepo::is_monorepo() {
    println!("✓ 检测到 Monorepo 项目");
} else {
    eprintln!("✗ 当前目录不是 Monorepo 项目");
}
```

#### get_default_workspace()

获取默认的 workspace 目录名称。

**函数签名**：
```rust
pub fn get_default_workspace() -> String
```

**逻辑**：
1. 如果 `apps` 目录存在，返回 `"apps"`
2. 否则如果 `packages` 目录存在，返回 `"packages"`
3. 否则返回默认值 `"apps"`

**示例**：
```rust
use utils::monorepo;

let workspace = monorepo::get_default_workspace();
println!("默认 workspace: {}", workspace);
```

#### workspace_exists()

检查指定的 workspace 目录是否存在。

**函数签名**：
```rust
pub fn workspace_exists(workspace: &str) -> bool
```

**参数**：
- `workspace`: workspace 目录名称

**返回**：
- `true`: 目录存在且是目录
- `false`: 目录不存在或不是目录

**示例**：
```rust
use utils::monorepo;

if monorepo::workspace_exists("apps") {
    println!("✓ apps 目录存在");
} else {
    println!("⚠ apps 目录不存在");
}
```

## 完整使用示例

### 示例 1：创建项目

```rust
use utils::{copy, print};

fn create_project(template_path: &str, project_name: &str) {
    // 打印欢迎信息
    print::print_title();
    println!();

    // 复制模板
    match copy::copy_dir(
        template_path.to_string(),
        project_name.to_string()
    ) {
        Ok(_) => {
            println!("✓ 项目创建成功");
        }
        Err(e) => {
            eprintln!("✗ 创建失败: {}", e);
            std::process::exit(1);
        }
    }
}
```

### 示例 2：Monorepo 检测

```rust
use utils::monorepo;

fn validate_monorepo() -> bool {
    if !monorepo::is_monorepo() {
        eprintln!("✗ 当前目录不是 Monorepo 项目");
        eprintln!("  提示: 使用 'scaffold init' 创建新项目");
        return false;
    }

    let workspace = monorepo::get_default_workspace();
    if !monorepo::workspace_exists(&workspace) {
        eprintln!("⚠ Workspace 目录 '{}' 不存在", workspace);
        return false;
    }

    true
}
```

### 示例 3：自定义渐变输出

```rust
use utils::print;

fn print_success_message(message: &str) {
    // 绿色渐变
    print::print_gradient(
        message,
        (0, 255, 0),    // 浅绿
        (0, 128, 0)     // 深绿
    );
}

fn print_error_message(message: &str) {
    // 红色渐变
    print::print_gradient(
        message,
        (255, 100, 100), // 浅红
        (139, 0, 0)      // 深红
    );
}
```

## 添加新工具函数

### 1. 创建新模块文件

```bash
touch src/new_util.rs
```

### 2. 实现工具函数

```rust
// src/new_util.rs

/// 新的工具函数描述
pub fn my_utility_function(param: &str) -> String {
    // 实现逻辑
    format!("Result: {}", param)
}
```

### 3. 在 lib.rs 中导出

```rust
// src/lib.rs
pub mod copy;
pub mod print;
pub mod monorepo;
pub mod new_util;  // 新增
```

### 4. 在其他 crate 中使用

```rust
use utils::new_util;

let result = new_util::my_utility_function("test");
```

## 性能优化建议

### 文件复制优化

- 使用大缓冲区（当前 5MB）
- 对于大文件考虑异步复制
- 添加进度条显示（可选）

### 输出优化

- 减少不必要的颜色输出
- 批量输出而非逐字符打印
- 使用缓冲写入

## 错误处理最佳实践

1. **详细的错误信息**

```rust
return Err(format!("源目录不存在: {}", source_path.display()).into());
```

2. **用户友好的提示**

```rust
eprintln!("✗ 创建失败: {}", e);
eprintln!("  提示: 检查模板目录是否存在");
```

3. **适当的错误传播**

```rust
.map_err(|e| format!("目录复制失败: {}", e))?
```

## 测试建议

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_dir() {
        // 测试目录复制
    }

    #[test]
    fn test_is_monorepo() {
        // 测试 monorepo 检测
    }
}
```

### 集成测试

在 `tests/` 目录创建集成测试：

```rust
// tests/copy_integration_test.rs
use utils::copy;

#[test]
fn test_full_project_copy() {
    // 测试完整的项目复制流程
}
```
