# itools-i18n

国际化支持模块 for iTools，提供完整的国际化功能，包括翻译管理、语言切换和参数化翻译。

## 特性

- ✅ 支持多语言翻译
- ✅ 参数化翻译
- ✅ 复数形式支持
- ✅ 性别形式支持
- ✅ 语言自动检测和切换
- ✅ 翻译缓存机制
- ✅ 错误处理和回退机制
- ✅ 支持 Fluent 格式

## 快速开始

### 基本使用

```rust
use itools_i18n::{t, t_with_args};

// 基本翻译
let hello = t("messages.hello");

// 带参数的翻译
let greeting = t_with_args("messages.greeting", &[("name", "John")]);
```

### 语言切换

```rust
use itools_i18n::{set_locale, get_locale};

// 设置语言
set_locale("zh-CN");

// 获取当前语言
let current_locale = get_locale();
```

## 高级用法

### 自定义翻译提供者

```rust
use itools_i18n::{SimpleTranslator, I18nContext};

// 创建简单翻译提供者
let mut translator = SimpleTranslator::new("en", "en");

// 添加翻译
let mut en_translations = std::collections::HashMap::new();
en_translations.insert("messages.hello".to_string(), "Hello world!".to_string());
translator.add_translations("en", en_translations);

// 创建国际化上下文
let mut context = I18nContext::new(Box::new(translator));

// 使用翻译
let hello = context.t("messages.hello");
```

### 使用 Fluent 格式

```rust
use itools_i18n::{FluentTranslator, I18nContext};
use std::path::Path;

// 创建 Fluent 翻译提供者
let mut translator = FluentTranslator::new("en", "en");

// 从文件加载翻译
let path = Path::new("locales/en.ftl");
translator.add_translations_from_file("en", path).unwrap();

// 从字符串加载翻译
let ftl_content = r#"
messages.hello = Hello world!
messages.greeting = Hello, {name}!
"#;
translator.add_translations_from_string("en", ftl_content).unwrap();

// 创建国际化上下文
let mut context = I18nContext::new(Box::new(translator));

// 使用翻译
let hello = context.t("messages.hello");
let greeting = context.t_with_args("messages.greeting", &[("name", "John")]);
```

## 配置

### 环境变量

- `LANG`: 设置默认语言
- `ITOOLS_LOCALE_EN`: 英文翻译文件路径
- `ITOOLS_LOCALE_ZH_CN`: 中文翻译文件路径
- `ITOOLS_LOCALE_{LANG}`: 其他语言翻译文件路径

### 翻译文件格式

使用 Fluent (.ftl) 格式：

```ftl
# 基本翻译
messages.hello = Hello world!

# 带参数的翻译
messages.greeting = Hello, {name}!

# 复数形式
messages.item_count = { $count ->
    [one] There is { $count } item
    *[other] There are { $count } items
}

# 性别形式
messages.greeting = { $gender ->
    [male] Hello, Mr. { $name }!
    [female] Hello, Ms. { $name }!
    *[other] Hello, { $name }!
}
```

## 性能优化

- **翻译缓存**: 使用 LRU 缓存存储翻译结果，减少重复翻译
- **资源预加载**: 启动时预加载常用语言的翻译资源
- **懒加载**: 仅在需要时加载翻译资源
- **错误处理**: 优雅处理翻译错误，回退到默认语言或键本身

## 与 Oak 系列库的集成

虽然 oak-fluent 库尚未实现，但 itools-i18n 模块设计为与 Oak 系列库保持一致的架构风格：

- **模块化设计**: 采用与 Oak 系列库相同的模块化架构
- **一致性接口**: 提供统一的翻译接口
- **可扩展性**: 支持自定义翻译提供者
- **性能优化**: 采用与 Oak 系列库相同的性能优化策略

## 示例

### 基本示例

```rust
use itools_i18n::{t, t_with_args, set_locale};

fn main() {
    // 设置语言
    set_locale("en");
    
    // 基本翻译
    println!("{}", t("messages.hello"));
    
    // 带参数的翻译
    println!("{}", t_with_args("messages.greeting", &[("name", "John")]));
    
    // 切换到中文
    set_locale("zh-CN");
    
    // 中文翻译
    println!("{}", t("messages.hello"));
    println!("{}", t_with_args("messages.greeting", &[("name", "张三")]));
}
```

### 复数形式示例

```rust
use itools_i18n::{t_with_args};

fn main() {
    // 测试复数形式
    for count in 0..5 {
        println!("{}", t_with_args("messages.item_count", &[("count", &count.to_string())]));
    }
}
```

## 故障排除

### 翻译不显示
- 检查翻译文件是否存在且格式正确
- 检查翻译键是否正确
- 检查语言设置是否正确

### 参数不替换
- 确保参数名称与 Fluent 文件中的一致
- 确保参数值类型正确

### 语言切换不生效
- 检查 `LANG` 环境变量设置
- 检查翻译文件路径是否正确
- 检查语言代码是否符合标准（如 "en", "zh-CN"）

## 贡献

欢迎贡献代码、报告问题或提出建议！

## 许可证

MIT
