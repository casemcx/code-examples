# Common - 公共类型定义

定义整个项目共享的数据类型、结构体和常量。

## 职责

- 定义项目模板相关类型
- 定义项目配置类型
- 提供模板查询和管理功能
- 确保类型一致性

## 依赖

无外部依赖（纯 Rust 标准库）

## 代码结构

```
src/
├── lib.rs          # Crate 入口
├── template.rs     # 模板相关类型
└── project.rs      # 项目配置类型
```

## 核心类型

### TemplateType

模板类型枚举，定义了三种模板类型：

```rust
pub enum TemplateType {
    /// Monorepo 基座
    Monorepo,
    /// 单体项目
    Standalone,
    /// Monorepo 子应用
    App,
}
```

**使用场景**：
- 区分不同类型的模板
- 根据需求选择合适的模板类型

### TemplateInfo

模板信息结构体：

```rust
pub struct TemplateInfo {
    pub name: String,              // 模板名称 (如 "Vue", "React")
    pub description: String,        // 描述信息
    pub template_type: TemplateType, // 模板类型
    pub path: String,               // 模板路径
}
```

**示例**：
```rust
TemplateInfo {
    name: "Vue".to_string(),
    description: "Vue Monorepo 基座".to_string(),
    template_type: TemplateType::Monorepo,
    path: "templates/vue/monorepo".to_string(),
}
```

### Template

模板枚举，按框架分类：

```rust
pub enum Template {
    React(TemplateInfo),
    Vue(TemplateInfo),
    Nest(TemplateInfo),
    Express(TemplateInfo),
}
```

## Template 核心方法

### all() - 获取所有模板

返回所有可用的模板列表。

```rust
let templates = Template::all();
// 返回所有模板（包括 Monorepo、Standalone、App 类型）
```

### all_frameworks() - 获取所有框架

返回去重后的框架名称列表。

```rust
let frameworks = Template::all_frameworks();
// 返回: ["React", "Vue", "Nest", "Express"]
```

### get_monorepo(name: &str) - 获取 Monorepo 模板

根据框架名称获取 Monorepo 基座模板。

```rust
let template = Template::get_monorepo("Vue");
// 返回 Vue 的 Monorepo 模板
```

### get_standalone(name: &str) - 获取 Standalone 模板

根据框架名称获取单体项目模板。

```rust
let template = Template::get_standalone("React");
// 返回 React 的单体项目模板
```

### get_app(name: &str) - 获取 App 模板

根据框架名称获取应用模板。

```rust
let template = Template::get_app("Vue");
// 返回 Vue 的应用模板
```

### supports_monorepo(name: &str) - 检查 Monorepo 支持

检查指定框架是否支持 Monorepo。

```rust
if Template::supports_monorepo("Vue") {
    // Vue 支持 monorepo
}
```

### transform_to_info(template: &Template) - 提取模板信息

从 Template 枚举中提取 TemplateInfo。

```rust
let template = Template::get_monorepo("Vue").unwrap();
let info = Template::transform_to_info(&template);
println!("路径: {}", info.path);
```

## 项目配置类型

### VueProject

Vue 项目配置（示例）：

```rust
pub struct VueProject {
    pub use_monorepo: bool,
    pub use_test: bool,
    pub working_name: Option<String>,
    pub use_sub_project: bool,
    pub sub_project_name: Option<String>,
}
```

## 使用示例

### 列出所有框架

```rust
use common::template::Template;

let frameworks = Template::all_frameworks();
for framework in frameworks {
    println!("框架: {}", framework);
}
```

### 根据用户选择获取模板

```rust
use common::template::{Template, TemplateType};

fn get_template_by_choice(
    framework: &str,
    use_monorepo: bool
) -> Option<Template> {
    if use_monorepo {
        Template::get_monorepo(framework)
    } else {
        Template::get_standalone(framework)
    }
}

// 使用
let template = get_template_by_choice("Vue", true);
```

### 检查并获取模板

```rust
use common::template::Template;

let framework = "Vue";

// 检查是否支持 monorepo
if Template::supports_monorepo(framework) {
    // 获取 monorepo 模板
    if let Some(template) = Template::get_monorepo(framework) {
        let info = Template::transform_to_info(&template);
        println!("创建 {} Monorepo 项目", info.name);
        println!("模板路径: {}", info.path);
    }
}
```

## 添加新框架

要添加新的框架模板：

1. **在 Template 枚举中添加新变体**

```rust
pub enum Template {
    React(TemplateInfo),
    Vue(TemplateInfo),
    Angular(TemplateInfo), // 新增
    // ...
}
```

2. **在 all() 方法中添加模板定义**

```rust
pub fn all() -> Vec<Template> {
    vec![
        // ... 现有模板

        // Angular 模板
        Template::Angular(TemplateInfo {
            name: "Angular".to_string(),
            description: "Angular Monorepo 基座".to_string(),
            template_type: TemplateType::Monorepo,
            path: "templates/angular/monorepo".to_string(),
        }),
        Template::Angular(TemplateInfo {
            name: "Angular".to_string(),
            description: "Angular 单体项目".to_string(),
            template_type: TemplateType::Standalone,
            path: "templates/angular/standalone".to_string(),
        }),
        // ...
    ]
}
```

3. **在 all_frameworks() 中添加框架名**

```rust
pub fn all_frameworks() -> Vec<String> {
    vec![
        "React".to_string(),
        "Vue".to_string(),
        "Angular".to_string(), // 新增
        // ...
    ]
}
```

4. **更新 transform_to_info() 方法**

```rust
pub fn transform_to_info(temp: &Template) -> &TemplateInfo {
    match temp {
        Template::React(info) |
        Template::Vue(info) |
        Template::Angular(info) | // 新增
        // ...
        => info
    }
}
```

## 设计原则

1. **不可变性**：模板信息一旦创建就不应修改
2. **类型安全**：使用枚举而非字符串，避免拼写错误
3. **集中管理**：所有模板定义在一处，便于维护
4. **查询便捷**：提供多种查询方法，满足不同需求

## 注意事项

- 所有路径都是相对于项目根目录
- 模板路径需要与实际文件系统结构对应
- 框架名称应保持一致的大小写（建议首字母大写）
- 不支持 Monorepo 的框架只需定义 Standalone 模板
