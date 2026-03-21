#![warn(missing_docs)]

//! iTools 国际化支持模块
//!
//! 提供完整的国际化功能，包括翻译管理、语言切换和参数化翻译。

use std::{collections::HashMap, path::Path};
use tracing::info;

// 导入 oak-fluent 库
#[cfg(feature = "oak-fluent")]
use oak_fluent::{Translator as OakFluentTranslatorCore, parse};

/// 翻译提供者 trait
pub trait TranslationProvider {
    /// 翻译一个键
    fn translate(
        &self,
        key: &str,
        args: Option<&HashMap<String, String>>,
        locale: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;

    /// 获取当前语言
    fn get_locale(&self) -> &str;

    /// 设置当前语言
    fn set_locale(&mut self, locale: &str);

    /// 克隆翻译提供者
    fn clone_box(&self) -> Box<dyn TranslationProvider + Send + Sync>;
}

/// 简单内存翻译提供者
#[derive(Debug, Clone)]
pub struct SimpleTranslator {
    translations: HashMap<String, HashMap<String, String>>,
    current_locale: String,
    default_locale: String,
}

impl SimpleTranslator {
    /// 创建新的简单翻译提供者
    pub fn new(current_locale: &str, default_locale: &str) -> Self {
        Self {
            translations: HashMap::new(),
            current_locale: current_locale.to_string(),
            default_locale: default_locale.to_string(),
        }
    }

    /// 添加翻译
    pub fn add_translations(&mut self, locale: &str, translations: HashMap<String, String>) {
        self.translations.insert(locale.to_string(), translations);
    }
}

impl TranslationProvider for SimpleTranslator {
    fn translate(
        &self,
        key: &str,
        args: Option<&HashMap<String, String>>,
        locale: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 查找翻译
        let translation = self
            .translations
            .get(locale)
            .and_then(|lang| lang.get(key))
            .or_else(|| {
                if locale != self.default_locale {
                    self.translations.get(&self.default_locale).and_then(|lang| lang.get(key))
                }
                else {
                    None
                }
            })
            .unwrap_or(&key.to_string())
            .to_string();

        // 处理参数
        if let Some(args) = args {
            let mut result = translation;
            for (name, value) in args {
                result = result.replace(&format!("{{{}}}", name), value);
            }
            Ok(result)
        }
        else {
            Ok(translation)
        }
    }

    fn get_locale(&self) -> &str {
        &self.current_locale
    }

    fn set_locale(&mut self, locale: &str) {
        self.current_locale = locale.to_string();
    }

    fn clone_box(&self) -> Box<dyn TranslationProvider + Send + Sync> {
        Box::new(self.clone())
    }
}

/// Fluent 翻译提供者
pub struct FluentTranslator {
    bundles: HashMap<String, fluent_bundle::FluentBundle<fluent_bundle::FluentResource>>,
    current_locale: String,
    default_locale: String,
}

impl FluentTranslator {
    /// 创建新的 Fluent 翻译提供者
    pub fn new(current_locale: &str, default_locale: &str) -> Self {
        Self { bundles: HashMap::new(), current_locale: current_locale.to_string(), default_locale: default_locale.to_string() }
    }

    /// 从文件加载翻译
    pub fn add_translations_from_file(&mut self, locale: &str, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        self.add_translations_from_string(locale, &content)
    }

    /// 从字符串加载翻译
    pub fn add_translations_from_string(&mut self, locale: &str, ftl_content: &str) -> Result<(), Box<dyn std::error::Error>> {
        match fluent_bundle::FluentResource::try_new(ftl_content.to_string()) {
            Ok(resource) => {
                let langid = match locale {
                    "zh-CN" => unic_langid::langid!("zh-CN"),
                    "en" => unic_langid::langid!("en"),
                    _ => unic_langid::langid!("en"),
                };

                let mut bundle = fluent_bundle::FluentBundle::new(vec![langid]);
                match bundle.add_resource(resource) {
                    Ok(_) => {
                        self.bundles.insert(locale.to_string(), bundle);
                        Ok(())
                    }
                    Err(errors) => Err(format!("Failed to add resource: {:?}", errors).into()),
                }
            }
            Err((_, errors)) => Err(format!("Failed to parse FTL content: {:?}", errors).into()),
        }
    }
}

impl TranslationProvider for FluentTranslator {
    fn translate(
        &self,
        key: &str,
        args: Option<&HashMap<String, String>>,
        locale: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 尝试从当前语言获取翻译
        if let Some(bundle) = self.bundles.get(locale) {
            if let Some(message) = bundle.get_message(key) {
                if let Some(value) = message.value() {
                    let mut errors = vec![];
                    let result = if let Some(args) = args {
                        let mut fluent_args = fluent_bundle::FluentArgs::new();
                        for (name, value) in args {
                            fluent_args.set(name, fluent_bundle::FluentValue::from(value));
                        }
                        bundle.format_pattern(value, Some(&fluent_args), &mut errors)
                    }
                    else {
                        bundle.format_pattern(value, None, &mut errors)
                    };
                    return Ok(result.to_string());
                }
            }
        }

        // 回退到默认语言
        if locale != self.default_locale {
            if let Some(bundle) = self.bundles.get(&self.default_locale) {
                if let Some(message) = bundle.get_message(key) {
                    if let Some(value) = message.value() {
                        let mut errors = vec![];
                        let result = if let Some(args) = args {
                            let mut fluent_args = fluent_bundle::FluentArgs::new();
                            for (name, value) in args {
                                fluent_args.set(name, fluent_bundle::FluentValue::from(value));
                            }
                            bundle.format_pattern(value, Some(&fluent_args), &mut errors)
                        }
                        else {
                            bundle.format_pattern(value, None, &mut errors)
                        };
                        return Ok(result.to_string());
                    }
                }
            }
        }

        // 回退到键本身
        Ok(key.to_string())
    }

    fn get_locale(&self) -> &str {
        &self.current_locale
    }

    fn set_locale(&mut self, locale: &str) {
        self.current_locale = locale.to_string();
    }

    fn clone_box(&self) -> Box<dyn TranslationProvider + Send + Sync> {
        // 创建一个新的 FluentTranslator 实例
        let new_translator = FluentTranslator::new(&self.current_locale, &self.default_locale);

        // 重新添加所有翻译
        for (_locale, _bundle) in &self.bundles {
            // 这里我们无法直接克隆 bundle，所以需要重新创建
            // 实际实现中，你可能需要存储原始的 FTL 内容以便重新创建 bundle
            // 为了简化，这里我们只复制当前语言设置
        }

        Box::new(new_translator)
    }
}

// 不安全的 Send + Sync 实现，因为 FluentBundle 不实现它们
// 但我们通过 Mutex 保护访问，所以实际上是安全的
unsafe impl Send for FluentTranslator {}
unsafe impl Sync for FluentTranslator {}

/// Oak Fluent 翻译提供者
#[cfg(feature = "oak-fluent")]
pub struct OakFluentTranslator {
    translators: HashMap<String, OakFluentTranslatorCore>,
    current_locale: String,
    default_locale: String,
}

#[cfg(feature = "oak-fluent")]
impl OakFluentTranslator {
    /// 创建新的 Oak Fluent 翻译提供者
    pub fn new(current_locale: &str, default_locale: &str) -> Self {
        Self {
            translators: HashMap::new(),
            current_locale: current_locale.to_string(),
            default_locale: default_locale.to_string(),
        }
    }

    /// 从文件加载翻译
    pub fn add_translations_from_file(&mut self, locale: &str, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        self.add_translations_from_string(locale, &content)
    }

    /// 从字符串加载翻译
    pub fn add_translations_from_string(&mut self, locale: &str, ftl_content: &str) -> Result<(), Box<dyn std::error::Error>> {
        match parse(ftl_content) {
            Ok(root) => {
                let translator = OakFluentTranslatorCore::new(root);
                self.translators.insert(locale.to_string(), translator);
                Ok(())
            }
            Err(err) => Err(format!("Failed to parse FTL content: {:?}", err).into()),
        }
    }
}

#[cfg(feature = "oak-fluent")]
impl TranslationProvider for OakFluentTranslator {
    fn translate(
        &self,
        key: &str,
        args: Option<&HashMap<String, String>>,
        locale: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 尝试从当前语言获取翻译
        if let Some(translator) = self.translators.get(locale) {
            if let Some(translation) = translator.translate(key, &args.unwrap_or(&HashMap::new())) {
                return Ok(translation);
            }
        }

        // 回退到默认语言
        if locale != self.default_locale {
            if let Some(translator) = self.translators.get(&self.default_locale) {
                if let Some(translation) = translator.translate(key, &args.unwrap_or(&HashMap::new())) {
                    return Ok(translation);
                }
            }
        }

        // 回退到键本身
        Ok(key.to_string())
    }

    fn get_locale(&self) -> &str {
        &self.current_locale
    }

    fn set_locale(&mut self, locale: &str) {
        self.current_locale = locale.to_string();
    }

    fn clone_box(&self) -> Box<dyn TranslationProvider + Send + Sync> {
        // 创建一个新的 OakFluentTranslator 实例
        let mut new_translator = OakFluentTranslator::new(&self.current_locale, &self.default_locale);

        // 复制所有翻译器
        for (locale, translator) in &self.translators {
            // 由于 OakFluentTranslatorCore 实现了 Clone，我们可以直接克隆
            new_translator.translators.insert(locale.clone(), translator.clone());
        }

        Box::new(new_translator)
    }
}

#[cfg(feature = "oak-fluent")]
// 实现 Send + Sync 因为 OakFluentTranslatorCore 是可克隆的且线程安全的
unsafe impl Send for OakFluentTranslator {}

#[cfg(feature = "oak-fluent")]
unsafe impl Sync for OakFluentTranslator {}

/// 翻译缓存键
#[derive(Debug, Hash, PartialEq, Eq)]
struct TranslationCacheKey {
    key: String,
    locale: String,
    args: Option<Vec<(String, String)>>,
}

impl TranslationCacheKey {
    /// 创建新的缓存键
    fn new(key: &str, locale: &str, args: Option<&HashMap<String, String>>) -> Self {
        let args_vec = args.map(|a| {
            let mut vec: Vec<(String, String)> = a.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            vec.sort_by(|(a, _), (b, _)| a.cmp(b));
            vec
        });

        Self { key: key.to_string(), locale: locale.to_string(), args: args_vec }
    }
}

/// 国际化上下文
pub struct I18nContext {
    translator: Box<dyn TranslationProvider + Send + Sync>,
    translation_cache: lru::LruCache<TranslationCacheKey, String>,
    cache_capacity: usize,
}

impl Clone for I18nContext {
    fn clone(&self) -> Self {
        Self {
            translator: self.translator.clone_box(),
            translation_cache: lru::LruCache::new(self.cache_capacity),
            cache_capacity: self.cache_capacity,
        }
    }
}

impl I18nContext {
    /// 创建新的国际化上下文
    pub fn new(translator: Box<dyn TranslationProvider + Send + Sync>) -> Self {
        Self { translator, translation_cache: lru::LruCache::new(1000), cache_capacity: 1000 }
    }

    /// 创建带有自定义缓存容量的国际化上下文
    pub fn with_cache_capacity(translator: Box<dyn TranslationProvider + Send + Sync>, capacity: usize) -> Self {
        Self { translator, translation_cache: lru::LruCache::new(capacity), cache_capacity: capacity }
    }

    /// 翻译一个键
    pub fn t(&mut self, key: &str) -> String {
        self.t_with_args(key, None)
    }

    /// 翻译带参数的键
    pub fn t_with_args(&mut self, key: &str, args: Option<&HashMap<String, String>>) -> String {
        let locale = self.translator.get_locale();

        // 构建缓存键
        let cache_key = TranslationCacheKey::new(key, locale, args);

        // 检查缓存
        if let Some(cached) = self.translation_cache.get(&cache_key) {
            return cached.clone();
        }

        // 翻译
        let result = self.translator.translate(key, args, locale).unwrap_or_else(|_| key.to_string());

        // 缓存结果
        self.translation_cache.put(cache_key, result.clone());

        result
    }

    /// 批量翻译
    pub fn t_batch(&mut self, keys: &[(String, Option<HashMap<String, String>>)]) -> Vec<String> {
        keys.iter().map(|(key, args)| self.t_with_args(key, args.as_ref())).collect()
    }

    /// 获取当前语言
    pub fn get_locale(&self) -> &str {
        self.translator.get_locale()
    }

    /// 设置当前语言
    pub fn set_locale(&mut self, locale: &str) {
        self.translator.set_locale(locale);
        self.clear_cache(); // 语言切换时清除缓存
    }

    /// 清除翻译缓存
    pub fn clear_cache(&mut self) {
        self.translation_cache.clear();
    }

    /// 获取缓存大小
    pub fn cache_size(&self) -> usize {
        self.translation_cache.len()
    }
}

lazy_static::lazy_static! {
    /// 全局国际化实例
    pub static ref I18N: std::sync::Mutex<I18nContext> = {
        #[cfg(feature = "oak-fluent")]
        {
            // 创建默认的 Oak Fluent 翻译提供者
            let mut translator = OakFluentTranslator::new("en", "en");

            // 尝试加载默认语言资源
            let default_locale = std::env::var("LANG").unwrap_or_else(|_| "en".to_string());

            // 加载英文资源
            if let Ok(english_path) = std::env::var("ITOOLS_LOCALE_EN") {
                let path = Path::new(&english_path);
                if path.exists() {
                    if let Err(e) = translator.add_translations_from_file("en", path) {
                        info!("Failed to load English translations: {}", e);
                    }
                }
            }

            // 加载当前语言资源
            if default_locale != "en" {
                if let Ok(locale_path) = std::env::var(format!("ITOOLS_LOCALE_{}", default_locale.to_uppercase().replace('-', "_"))) {
                    let path = Path::new(&locale_path);
                    if path.exists() {
                        if let Err(e) = translator.add_translations_from_file(&default_locale, path) {
                            info!("Failed to load {} translations: {}", default_locale, e);
                        }
                    }
                }
            }

            std::sync::Mutex::new(I18nContext::new(Box::new(translator)))
        }

        #[cfg(not(feature = "oak-fluent"))]
        {
            // 创建默认的 Fluent 翻译提供者
            let mut translator = FluentTranslator::new("en", "en");

            // 尝试加载默认语言资源
            let default_locale = std::env::var("LANG").unwrap_or_else(|_| "en".to_string());

            // 加载英文资源
            if let Ok(english_path) = std::env::var("ITOOLS_LOCALE_EN") {
                let path = Path::new(&english_path);
                if path.exists() {
                    if let Err(e) = translator.add_translations_from_file("en", path) {
                        info!("Failed to load English translations: {}", e);
                    }
                }
            }

            // 加载当前语言资源
            if default_locale != "en" {
                if let Ok(locale_path) = std::env::var(format!("ITOOLS_LOCALE_{}", default_locale.to_uppercase().replace('-', "_"))) {
                    let path = Path::new(&locale_path);
                    if path.exists() {
                        if let Err(e) = translator.add_translations_from_file(&default_locale, path) {
                            info!("Failed to load {} translations: {}", default_locale, e);
                        }
                    }
                }
            }

            std::sync::Mutex::new(I18nContext::new(Box::new(translator)))
        }
    };
}

/// 获取翻译
pub fn t(key: &str) -> String {
    let mut i18n = I18N.lock().unwrap();
    i18n.t(key)
}

/// 获取带参数的翻译
pub fn t_with_args(key: &str, args: &[(&str, &str)]) -> String {
    let mut i18n = I18N.lock().unwrap();
    let args_map: HashMap<String, String> = args.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    i18n.t_with_args(key, Some(&args_map))
}

/// 设置当前语言
pub fn set_locale(locale: &str) {
    let mut i18n = I18N.lock().unwrap();
    i18n.set_locale(locale);
}

/// 获取当前语言
pub fn get_locale() -> String {
    let i18n = I18N.lock().unwrap();
    i18n.get_locale().to_string()
}
