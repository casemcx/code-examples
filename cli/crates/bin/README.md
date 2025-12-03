# Scaffold CLI - 主程序

主程序入口 crate，负责解析命令行参数并路由到相应的命令处理器。

## 职责

- 程序入口点（`main` 函数）
- 解析命令行参数
- 路由到相应的命令实现
- 协调各个 crate 的功能

## 依赖关系

```
bin
├── args      # CLI 参数定义
├── commands  # 命令实现
└── common    # 公共类型
```

## 代码结构

```
src/
└── main.rs   # 主入口文件
```

## 主要流程

```rust
1. 解析命令行参数 (使用 args crate)
   ↓
2. 匹配命令类型 (Init/Create/List)
   ↓
3. 调用对应的命令处理器 (commands crate)
   ↓
4. 执行完成/错误处理
```

## 命令路由

| 命令 | 处理器 | 说明 |
|------|--------|------|
| `init` | `commands::init::init_project()` | 初始化新项目 |
| `create` | `commands::create::create_sub_project()` | 创建子项目 |
| `list` | `commands::list::list_project()` | 列出可用模板 |

## 构建运行

```bash
# 开发模式运行
cargo run -- --help

# 构建
cargo build --release

# 可执行文件位置
target/release/scaffold
```

## 示例用法

```bash
# 初始化项目
cargo run -- init -n my-project -t Vue

# 在 monorepo 中创建子项目
cargo run -- create -n admin -t React -w apps

# 列出模板
cargo run -- list
```
