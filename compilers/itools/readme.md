# iTools

🛠️ iTools 是一个强大的 Rust 开发工具包，提供命令行接口和国际化支持。

## 特性

- ✨ 国际化支持：使用 Mozilla Fluent 框架实现多语言能力
- ✨ 命令行接口：提供便捷的项目管理命令
- ✨ 宏和函数：同时支持宏和函数形式的翻译调用
- ✨ 高级翻译特性：支持复数形式、性别形式等
- ✨ 语言自动检测：根据环境变量自动切换语言

## 使用方法

### 命令行接口

#### 可用命令
- `init <name>` - 初始化新项目
- `build` - 构建项目
- `dev` - 启动开发服务器
- `check` - 检查项目

#### 示例

```bash
# 初始化新项目
itools init my-project

# 构建项目
itools build

# 启动开发服务器
itools dev

# 检查项目
itools check
```

### 国际化功能

#### 基本翻译
```rust
use itools::t;

// 获取基本翻译
let message = t!("messages.cli_initialized");
```

#### 带参数的翻译
```rust
use itools::t;

// 获取带参数的翻译
let message = t!("messages.initializing_project", &[("name", "my-project")]);
```

#### 复数形式
```ftl
// 在 locales/en.ftl 中
messages.item_count = { $count ->
    [one] There is { $count } item
    *[other] There are { $count } items
}
```

使用方式：
```rust
let message = t!("messages.item_count", &[("count", "1")]);
```

#### 性别形式
```ftl
// 在 locales/en.ftl 中
messages.greeting = { $gender ->
    [male] Hello, Mr. { $name }!
    [female] Hello, Ms. { $name }!
    *[other] Hello, { $name }!
}
```

使用方式：
```rust
let message = t!("messages.greeting", &[("gender", "male"), ("name", "John")]);
```

## 注意事项

- 使用 `t!` 宏进行翻译，它提供编译时检查和更好的性能
- 语言切换：通过设置 `LANG` 环境变量来切换语言，例如 `LANG=zh-CN itools init my-project`
- 翻译资源：国际化资源存储在 `locales` 目录中，使用 Fluent (.ftl) 格式
- 最佳实践：使用语义化的键名，保持键名一致，充分利用 Fluent 的高级特性
- 性能优化：Fluent 资源在程序启动时加载，使用全局缓存减少重复解析
- 故障排除：翻译不显示时检查键名是否正确，参数不替换时确保参数名称与 Fluent 文件一致