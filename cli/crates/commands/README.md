# Commands - 命令实现

实现所有 CLI 命令的核心业务逻辑。

## 职责

- 实现各个命令的业务逻辑
- 处理用户交互（提示、选择等）
- 协调模板、文件复制等操作
- 错误处理和用户反馈

## 依赖

- `common` - 公共类型和模板定义
- `utils` - 工具函数（复制、打印等）
- `dialoguer` - 交互式命令行界面
- `colored` - 终端颜色输出

## 代码结构

```
src/
├── lib.rs              # Crate 入口
├── init/               # Init 命令
│   ├── mod.rs          # 主逻辑
│   ├── prompts.rs      # 交互提示
│   └── handler.rs      # 业务处理
├── create/             # Create 命令
│   ├── mod.rs          # 主逻辑
│   ├── prompts.rs      # 交互提示
│   └── handler.rs      # 业务处理
└── list/               # List 命令
    └── mod.rs          # 主逻辑
```

## 模块设计模式

每个命令模块遵循统一的设计模式：

### mod.rs - 主逻辑

包含命令的主要业务流程和协调逻辑。

```rust
// init/mod.rs
pub fn init_project(name: Option<String>, template_name: String) {
    // 1. 获取项目信息
    // 2. 选择项目类型（monorepo/standalone）
    // 3. 创建项目
    // 4. 显示后续步骤
}
```

### prompts.rs - 交互提示

处理所有用户交互（输入、选择、确认等）。

```rust
// init/prompts.rs
pub fn select_framework() -> String { /* ... */ }
pub fn ask_use_monorepo() -> bool { /* ... */ }
pub fn get_app_name() -> String { /* ... */ }
```

### handler.rs - 业务处理

处理纯业务逻辑，不涉及用户交互。

```rust
// init/handler.rs
pub fn get_project_name(name: Option<String>) -> String { /* ... */ }
pub fn get_framework_name(template_name: String) -> String { /* ... */ }
```

## 命令详解

### Init 命令

**功能**：初始化新项目

**流程**：
1. 获取项目名称（参数或提示）
2. 选择框架（Vue/React/Nest/Express）
3. 询问是否使用 Monorepo
4. 创建 Monorepo 基座或单体项目
5. （可选）创建初始应用

**关键函数**：
- `init_project()` - 主入口
- `create_monorepo_project()` - 创建 Monorepo 项目
- `create_standalone_project()` - 创建单体项目

### Create 命令

**功能**：在 Monorepo 中创建子项目

**流程**：
1. 检测是否在 Monorepo 中
2. 获取 workspace 目录
3. 获取应用名称
4. 选择框架
5. 创建子应用

**关键函数**：
- `create_sub_project()` - 主入口
- `create_app()` - 创建应用

### List 命令

**功能**：列出所有可用模板

**流程**：
1. 获取所有模板
2. 格式化输出

## 使用示例

### 在主程序中调用

```rust
use commands::{init::init_project, create::create_sub_project, list::list_project};

// 初始化项目
init_project(Some("my-project".to_string()), "Vue".to_string());

// 创建子项目
create_sub_project(
    Some("apps".to_string()),
    Some("admin".to_string()),
    "React".to_string()
);

// 列出模板
list_project();
```

## 添加新命令

要添加新命令，按以下步骤：

1. **创建命令目录**

```bash
mkdir -p src/new_command
```

2. **创建必要文件**

```
src/new_command/
├── mod.rs       # 主逻辑
├── prompts.rs   # 交互提示（可选）
└── handler.rs   # 业务处理（可选）
```

3. **实现主函数**

```rust
// src/new_command/mod.rs
pub fn execute_command(/* 参数 */) {
    // 实现逻辑
}
```

4. **在 lib.rs 中导出**

```rust
// src/lib.rs
pub mod new_command;
```

5. **在 bin/main.rs 中调用**

```rust
use commands::new_command::execute_command;

match command {
    Commands::NewCommand { /* ... */ } => {
        execute_command(/* ... */);
    }
}
```

## 错误处理

命令实现遵循以下错误处理原则：

1. **友好的错误提示**：使用 `eprintln!` 和颜色标记
2. **清晰的错误信息**：说明错误原因和解决方法
3. **适当的退出码**：使用 `std::process::exit(1)` 退出

示例：

```rust
if !monorepo::is_monorepo() {
    eprintln!("✗ {} 当前目录不是 monorepo 项目", "错误:".red());
    eprintln!("  提示: 使用 {} 创建新项目", "scaffold init".cyan());
    std::process::exit(1);
}
```

## 测试建议

- **单元测试**：测试 handler.rs 中的纯函数
- **集成测试**：测试完整的命令流程
- **模拟测试**：使用 mock 测试用户交互

## 性能考虑

- 使用懒加载避免不必要的模板加载
- 文件复制使用缓冲区优化
- 避免重复的文件系统操作
