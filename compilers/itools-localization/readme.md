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
- ✨ 支持 JSON、TOML、YAML 格式（通过特性开关）

## 安装

在 `Cargo.toml` 文件中添加依赖：

```toml
[dependencies]
itools-localization = {
    path = "path/to/itools-localization",
    features = ["json", "toml", "yaml"] # 可选特性
}
```

## API 参考

### 核心类型

#### I18nError

国际化错误类型，包含以下变体：

- `FileRead` - 文件读取错误
- `ParseError` - 解析错误
- `UnsupportedFormat` - 不支持的文件格式
- `FeatureNotEnabled` - 特性未启用
- `TranslationNotFound` - 翻译未找到

#### TranslationProvider

翻译提供者 trait，定义了翻译功能的核心接口：

```rust
pub trait TranslationProvider {
    fn translate(
        &self,
        key: &str,
        args: Option<&HashMap<String, String>>,
        locale: &str,
    ) -> Result<String, I18nError>;

    fn get_locale(&self) -> &str;

    fn set_locale(&mut self, locale: &str);

    fn clone_box(&self) -> Box<dyn TranslationProvider + Send + Sync>;
}
```

#### SimpleTranslator

简单内存翻译提供者，适用于小型应用或测试：

```rust
pub struct SimpleTranslator {
    translations: HashMap<String, HashMap<String, String>>,
    current_locale: String,
    default_locale: String,
}
```

**方法**：
- `new(current_locale: &str, default_locale: &str) -> Self` - 创建新的简单翻译提供者
- `add_translations(&mut self, locale: &str, translations: HashMap<String, String>)` - 添加翻译

#### OakFluentTranslator

Oak Fluent 翻译提供者，支持 Fluent 格式的翻译文件：

```rust
pub struct OakFluentTranslator {
    translators: HashMap<String, OakFluentTranslatorCore>,
    current_locale: String,
    default_locale: String,
}
```

**方法**：
- `new(current_locale: &str, default_locale: &str) -> Self` - 创建新的 Oak Fluent 翻译提供者
- `add_translations_from_file(&mut self, locale: &str, file_path: &Path) -> Result<(), I18nError>` - 从文件加载翻译
- `add_simple_translations(&mut self, locale: &str, translations: HashMap<String, String>) -> Result<(), I18nError>` - 添加简单翻译
- `add_translations_from_string(&mut self, locale: &str, ftl_content: &str) -> Result<(), I18nError>` - 从字符串加载翻译

#### I18nContext

国际化上下文，提供翻译功能的高级接口：

```rust
pub struct I18nContext {
    translator: Box<dyn TranslationProvider + Send + Sync>,
    translation_cache: lru::LruCache<TranslationCacheKey, String>,
    cache_capacity: usize,
}
```

**方法**：
- `new(translator: Box<dyn TranslationProvider + Send + Sync>) -> Self` - 创建新的国际化上下文
- `with_cache_capacity(translator: Box<dyn TranslationProvider + Send + Sync>, capacity: usize) -> Self` - 创建带有自定义缓存容量的国际化上下文
- `t(&mut self, key: &str) -> String` - 翻译一个键
- `t_with_args(&mut self, key: &str, args: Option<&HashMap<String, String>>) -> String` - 翻译带参数的键
- `t_batch(&mut self, keys: &[(String, Option<HashMap<String, String>>)]) -> Vec<String>` - 批量翻译
- `get_locale(&self) -> &str` - 获取当前语言
- `set_locale(&mut self, locale: &str)` - 设置当前语言
- `clear_cache(&mut self)` - 清除翻译缓存
- `cache_size(&self) -> usize` - 获取缓存大小

### 全局函数

- `t(key: &str) -> String` - 获取翻译
- `t_with_args(key: &str, args: &[(&str, &str)]) -> String` - 获取带参数的翻译
- `set_locale(locale: &str)` - 设置当前语言
- `get_locale() -> String` - 获取当前语言

## 使用示例

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
use std::collections::HashMap;

// 创建简单翻译提供者
let mut translator = SimpleTranslator::new("en", "en");

// 添加翻译
let mut en_translations = HashMap::new();
en_translations.insert("messages.hello".to_string(), "Hello world!".to_string());
translator.add_translations("en", en_translations);

// 添加中文翻译
let mut zh_translations = HashMap::new();
zh_translations.insert("messages.hello".to_string(), "你好，世界!".to_string());
translator.add_translations("zh-CN", zh_translations);

// 创建国际化上下文
let mut context = I18nContext::new(Box::new(translator));

// 使用翻译
let hello = context.t("messages.hello");

// 切换语言
context.set_locale("zh-CN");
let hello_zh = context.t("messages.hello");
```

### 使用 Fluent 格式

```rust
use itools_localization::{OakFluentTranslator, I18nContext};
use std::path::Path;

// 创建 Fluent 翻译提供者
let mut translator = OakFluentTranslator::new("en", "en");

// 从文件加载翻译
let path = Path::new("locales/en.ftl");
translator.add_translations_from_file("en", path).unwrap();

// 从字符串加载翻译
let ftl_content = r#"
messages.hello = Hello world!
messages.greeting = Hello, {name}!
messages.apples = 
    { $count ->
        [one] There is one apple
       *[other] There are {count} apples
    }
"#;
translator.add_translations_from_string("en", ftl_content).unwrap();

// 创建国际化上下文
let mut context = I18nContext::new(Box::new(translator));

// 使用翻译
let hello = context.t("messages.hello");
let greeting = context.t_with_args("messages.greeting", &[("name", "John")]);
let apples = context.t_with_args("messages.apples", &[("count", "5")]);
```

### 使用 JSON 格式

```rust
use itools_localization::{OakFluentTranslator, I18nContext};
use std::path::Path;

// 创建 Fluent 翻译提供者
let mut translator = OakFluentTranslator::new("en", "en");

// 从 JSON 文件加载翻译（需要启用 json 特性）
let path = Path::new("locales/en.json");
translator.add_translations_from_file("en", path).unwrap();

// 创建国际化上下文
let mut context = I18nContext::new(Box::new(translator));

// 使用翻译
let hello = context.t("messages.hello");
```

### 使用 TOML 格式

```rust
use itools_localization::{OakFluentTranslator, I18nContext};
use std::path::Path;

// 创建 Fluent 翻译提供者
let mut translator = OakFluentTranslator::new("en", "en");

// 从 TOML 文件加载翻译（需要启用 toml 特性）
let path = Path::new("locales/en.toml");
translator.add_translations_from_file("en", path).unwrap();

// 创建国际化上下文
let mut context = I18nContext::new(Box::new(translator));

// 使用翻译
let hello = context.t("messages.hello");
```

### 使用 YAML 格式

```rust
use itools_localization::{OakFluentTranslator, I18nContext};
use std::path::Path;

// 创建 Fluent 翻译提供者
let mut translator = OakFluentTranslator::new("en", "en");

// 从 YAML 文件加载翻译（需要启用 yaml 特性）
let path = Path::new("locales/en.yaml");
translator.add_translations_from_file("en", path).unwrap();

// 创建国际化上下文
let mut context = I18nContext::new(Box::new(translator));

// 使用翻译
let hello = context.t("messages.hello");
```

### 批量翻译

```rust
use itools_localization::{OakFluentTranslator, I18nContext};
use std::collections::HashMap;

// 创建翻译提供者并添加翻译
let mut translator = OakFluentTranslator::new("en", "en");
let ftl_content = r#"
messages.hello = Hello world!
messages.greeting = Hello, {name}!
messages.bye = Goodbye!
"#;
translator.add_translations_from_string("en", ftl_content).unwrap();

// 创建国际化上下文
let mut context = I18nContext::new(Box::new(translator));

// 批量翻译
let mut args = HashMap::new();
args.insert("name".to_string(), "John".to_string());

let translations = context.t_batch(&[
    ("messages.hello".to_string(), None),
    ("messages.greeting".to_string(), Some(args)),
    ("messages.bye".to_string(), None),
]);

for translation in translations {
    println!("{}", translation);
}
```

## 最佳实践

### 翻译文件组织

- **推荐结构**：
  ```
  locales/
  ├── en.ftl       # 英文翻译
  ├── zh-CN.ftl    # 中文简体翻译
  ├── fr.ftl       # 法语翻译
  └── es.ftl       # 西班牙语翻译
  ```

- **命名约定**：
  - 翻译键使用点分隔的命名空间格式（如 `messages.hello`）
  - 语言代码使用标准格式（如 `en`, `zh-CN`）
  - 翻译文件使用对应语言代码作为文件名

### 性能优化

1. **使用缓存**：
   - 默认缓存容量为 1000，可以根据应用规模调整
   - 语言切换时会自动清除缓存

2. **资源预加载**：
   - 应用启动时预加载所有翻译资源
   - 避免运行时动态加载翻译文件

3. **懒加载**：
   - 对于大型应用，可以考虑按需加载翻译资源
   - 仅加载当前语言和默认语言的翻译

### 错误处理

1. **回退机制**：
   - 当翻译未找到时，会回退到默认语言
   - 当默认语言也没有翻译时，会回退到键本身

2. **错误日志**：
   - 翻译错误会记录到调试日志中
   - 不会影响应用正常运行

### 国际化最佳实践

1. **文本提取**：
   - 使用工具自动提取代码中的文本
   - 确保所有用户可见的文本都经过翻译

2. **文化适配**：
   - 考虑不同文化的习惯和偏好
   - 避免硬编码日期、时间和数字格式

3. **复数形式**：
   - 使用 Fluent 的复数形式支持
   - 不同语言的复数规则可能不同

4. **性别形式**：
   - 使用 Fluent 的性别形式支持
   - 考虑不同语言的性别语法差异

## 常见问题

### 翻译不显示

1. **检查翻译文件**：
   - 确保翻译文件存在且格式正确
   - 确保翻译键与代码中使用的键一致

2. **检查语言设置**：
   - 确保设置了正确的语言代码
   - 确保翻译文件包含对应语言的翻译

3. **检查特性开关**：
   - 如果使用 JSON、TOML 或 YAML 格式，确保启用了对应的特性

### 语言切换不生效

1. **检查语言代码**：
   - 确保使用了正确的语言代码格式
   - 确保翻译文件使用了相同的语言代码

2. **检查环境变量**：
   - 全局实例会尝试读取 `LANG` 环境变量
   - 可以通过 `set_locale` 函数覆盖环境变量设置

3. **检查缓存**：
   - 语言切换时会自动清除缓存
   - 如果手动管理缓存，确保在语言切换后清除缓存

### 性能问题

1. **检查缓存容量**：
   - 对于大型应用，考虑增加缓存容量
   - 对于小型应用，使用默认缓存容量即可

2. **检查翻译文件大小**：
   - 避免在单个翻译文件中包含过多翻译
   - 考虑按模块拆分翻译文件

3. **检查加载方式**：
   - 预加载翻译资源，避免运行时动态加载
   - 仅加载必要的语言翻译

## 示例项目结构

```
my-app/
├── src/
│   ├── main.rs
│   └── i18n.rs
├── locales/
│   ├── en.ftl
│   ├── zh-CN.ftl
│   └── fr.ftl
└── Cargo.toml
```

### i18n.rs 示例

```rust
use itools_localization::{OakFluentTranslator, I18nContext};
use std::path::Path;

pub fn init_i18n() -> I18nContext {
    // 创建翻译提供者
    let mut translator = OakFluentTranslator::new("en", "en");

    // 加载翻译文件
    let locales_dir = Path::new("locales");
    
    // 加载英文翻译
    let en_path = locales_dir.join("en.ftl");
    if en_path.exists() {
        translator.add_translations_from_file("en", &en_path).unwrap();
    }
    
    // 加载中文翻译
    let zh_path = locales_dir.join("zh-CN.ftl");
    if zh_path.exists() {
        translator.add_translations_from_file("zh-CN", &zh_path).unwrap();
    }
    
    // 加载法语翻译
    let fr_path = locales_dir.join("fr.ftl");
    if fr_path.exists() {
        translator.add_translations_from_file("fr", &fr_path).unwrap();
    }

    // 创建国际化上下文
    I18nContext::new(Box::new(translator))
}
```

### main.rs 示例

```rust
use my_app::i18n::init_i18n;
use itools_localization::{set_locale, t, t_with_args};

fn main() {
    // 初始化国际化
    let mut context = init_i18n();

    // 使用全局函数
    println!("{}", t("messages.hello"));
    println!("{}", t_with_args("messages.greeting", &[("name", "John")]));

    // 使用上下文
    println!("{}", context.t("messages.hello"));
    
    // 切换语言
    set_locale("zh-CN");
    println!("{}", t("messages.hello"));
}
```

## 许可证

MIT
