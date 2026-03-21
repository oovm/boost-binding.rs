# iTools Localization

🌍 iTools Localization 是 iTools 项目的国际化支持模块，提供完整的国际化功能，包括翻译管理、语言切换和参数化翻译。

## 特性

- ✨ 支持多语言翻译
- ✨ 参数化翻译
- ✨ 复数形式支持
- ✨ 性别形式支持
- ✨ 语言自动检测和切换
- ✨ 翻译缓存机制
- ✨ 错误处理和回退机制
- ✨ 支持 Fluent 格式

## 使用方法

### 基本使用

```rust
use itools_localization::{t, t_with_args};

// 基本翻译
let hello = t("messages.hello");

// 带参数的翻译
let greeting = t_with_args("messages.greeting", &[("name", "John")]);
```

### 语言切换

```rust
use itools_localization::{set_locale, get_locale};

// 设置语言
set_locale("zh-CN");

// 获取当前语言
let current_locale = get_locale();
```

### 自定义翻译提供者

```rust
use itools_localization::{SimpleTranslator, I18nContext};

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
use itools_localization::{FluentTranslator, I18nContext};
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

## 注意事项

- 翻译文件格式支持 Fluent (.ftl) 格式
- 翻译键应该使用点分隔的命名空间格式（如 "messages.hello"）
- 参数名称需要与 Fluent 文件中的占位符一致
- 语言代码应该符合标准格式（如 "en", "zh-CN"）
- 翻译不显示时，请检查翻译文件是否存在且格式正确
- 语言切换不生效时，请检查 `LANG` 环境变量设置
- 性能优化包括翻译缓存、资源预加载和懒加载