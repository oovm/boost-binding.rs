# iTools CLI

## 国际化功能

iTools CLI 使用 Mozilla Fluent 框架实现国际化支持，提供了强大的多语言能力。

### 支持的语言
- 英文 (`en`)
- 中文 (`zh-CN`)

### 如何使用国际化功能

#### 基本翻译
```rust
use itools::t;

// 获取基本翻译
let message = t("messages.cli_initialized");
```

#### 带参数的翻译
```rust
use itools::t_with_args;

// 获取带参数的翻译
let message = t_with_args("messages.initializing_project", &[("name", "my-project")]);
```

### 国际化资源管理

国际化资源存储在 `locales` 目录中，使用 Fluent (.ftl) 格式：
- `locales/en.ftl` - 英文资源
- `locales/zh-CN.ftl` - 中文资源

### 如何添加新的翻译

1. **在 Fluent 文件中添加新的翻译条目**
   ```ftl
   // 在 locales/en.ftl 中
   my.new.message = This is a new message
   
   // 在 locales/zh-CN.ftl 中
   my.new.message = 这是一条新消息
   ```

2. **在代码中使用新的翻译**
   ```rust
   let message = t("my.new.message");
   ```

### 高级特性

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
let message = t_with_args("messages.item_count", &[("count", "1")]);
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
let message = t_with_args("messages.greeting", &[("gender", "male"), ("name", "John")]);
```

### 语言切换

通过设置 `LANG` 环境变量来切换语言：

```bash
# 使用英文
LANG=en itools init my-project

# 使用中文
LANG=zh-CN itools init my-project
```

### 最佳实践

1. **使用语义化的键名** - 例如 `errors.invalid_input` 而不是 `error1`
2. **保持键名一致** - 在所有语言文件中使用相同的键名
3. **使用 Fluent 的高级特性** - 充分利用复数形式、性别等特性
4. **测试翻译** - 确保所有语言的翻译都能正常工作
5. **回退机制** - 当某个语言的翻译不存在时，会自动回退到英文

### 性能优化

- Fluent 资源在程序启动时加载，避免运行时的文件 I/O
- 使用全局缓存，减少重复解析
- 只加载当前语言和英文资源，减少内存使用

### 扩展支持的语言

1. **创建新的 Fluent 文件** - 例如 `locales/fr.ftl` 用于法语
2. **在 `I18n::new()` 方法中添加对新语言的支持**
3. **更新语言检测逻辑**

### 故障排除

- **翻译不显示**：检查键名是否正确，以及是否在所有语言文件中都存在
- **参数不替换**：确保参数名称与 Fluent 文件中的一致
- **语言切换不生效**：检查 `LANG` 环境变量的设置是否正确

## 命令行接口

### 可用命令
- `init <name>` - 初始化新项目
- `build` - 构建项目
- `dev` - 启动开发服务器
- `check` - 检查项目

### 示例

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
