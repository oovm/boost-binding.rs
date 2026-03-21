#![warn(missing_docs)]

//! iTools 国际化支持模块
//!
//! 提供完整的国际化功能，包括翻译管理、语言切换和参数化翻译。

use std::{collections::HashMap, num::NonZeroUsize, path::Path};
use tracing::info;

/// 国际化错误类型
#[derive(Debug)]
pub enum I18nError {
    /// 文件读取错误
    FileRead(std::io::Error),
    /// 解析错误
    ParseError(String),
    /// 不支持的文件格式
    UnsupportedFormat(String),
    /// 特性未启用
    FeatureNotEnabled(String),
    /// 翻译未找到
    TranslationNotFound(String),
}

impl std::error::Error for I18nError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            I18nError::FileRead(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for I18nError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            I18nError::FileRead(err) => write!(f, "Failed to read file: {}", err),
            I18nError::ParseError(err) => write!(f, "Failed to parse content: {}", err),
            I18nError::UnsupportedFormat(format) => write!(f, "Unsupported file format: {}", format),
            I18nError::FeatureNotEnabled(feature) => write!(f, "Feature not enabled: {}", feature),
            I18nError::TranslationNotFound(key) => write!(f, "Translation not found: {}", key),
        }
    }
}

impl From<std::io::Error> for I18nError {
    fn from(err: std::io::Error) -> Self {
        I18nError::FileRead(err)
    }
}

// 导入 oak-fluent 库
use oak_fluent::{Translator as OakFluentTranslatorCore, parse};

/// 翻译提供者 trait
pub trait TranslationProvider {
    /// 翻译一个键
    fn translate(&self, key: &str, args: Option<&HashMap<String, String>>, locale: &str) -> Result<String, I18nError>;

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
    fn translate(&self, key: &str, args: Option<&HashMap<String, String>>, locale: &str) -> Result<String, I18nError> {
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

/// Oak Fluent 翻译提供者
pub struct OakFluentTranslator {
    translators: HashMap<String, OakFluentTranslatorCore>,
    current_locale: String,
    default_locale: String,
}

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
    pub fn add_translations_from_file(&mut self, locale: &str, file_path: &Path) -> Result<(), I18nError> {
        let content = std::fs::read_to_string(file_path)?;

        // 根据文件扩展名选择解析器
        match file_path.extension().and_then(|ext| ext.to_str()) {
            Some("ftl") => self.add_translations_from_string(locale, &content),
            Some("json") => {
                #[cfg(feature = "json")]
                {
                    let translations: HashMap<String, String> =
                        oak_json::from_str(&content).map_err(|e| I18nError::ParseError(e.to_string()))?;
                    self.add_simple_translations(locale, translations)
                }
                #[cfg(not(feature = "json"))]
                {
                    Err(I18nError::FeatureNotEnabled("json".to_string()))
                }
            }
            Some("toml") => {
                #[cfg(feature = "toml")]
                {
                    let translations: HashMap<String, String> =
                        oak_toml::from_str(&content).map_err(|e| I18nError::ParseError(e.to_string()))?;
                    self.add_simple_translations(locale, translations)
                }
                #[cfg(not(feature = "toml"))]
                {
                    Err(I18nError::FeatureNotEnabled("toml".to_string()))
                }
            }
            Some("yaml") | Some("yml") => {
                #[cfg(feature = "yaml")]
                {
                    let translations: HashMap<String, String> =
                        oak_yaml::from_str(&content).map_err(|e| I18nError::ParseError(e.to_string()))?;
                    self.add_simple_translations(locale, translations)
                }
                #[cfg(not(feature = "yaml"))]
                {
                    Err(I18nError::FeatureNotEnabled("yaml".to_string()))
                }
            }
            Some(ext) => Err(I18nError::UnsupportedFormat(ext.to_string())),
            None => Err(I18nError::UnsupportedFormat("unknown".to_string())),
        }
    }

    /// 添加简单翻译（从 HashMap 加载）
    pub fn add_simple_translations(&mut self, locale: &str, translations: HashMap<String, String>) -> Result<(), I18nError> {
        // 这里我们需要将 HashMap 转换为 Fluent 格式，或者添加对简单键值对的支持
        // 为了简单起见，我们暂时将其添加到 SimpleTranslator 中
        // 后续可以考虑更复杂的实现

        // 这里我们使用一个临时的 SimpleTranslator 来处理简单的键值对翻译
        // 实际实现中，我们可能需要将这些翻译转换为 Fluent 格式

        // 由于 OakFluentTranslator 目前只支持 Fluent 格式，我们需要添加对简单键值对的支持
        // 这里我们使用一个简单的方法，将键值对转换为 Fluent 格式的字符串
        let mut ftl_content = String::new();
        for (key, value) in translations {
            ftl_content.push_str(&format!("{} = {}", key, value));
            ftl_content.push('\n');
        }

        self.add_translations_from_string(locale, &ftl_content)
    }

    /// 从字符串加载翻译
    pub fn add_translations_from_string(&mut self, locale: &str, ftl_content: &str) -> Result<(), I18nError> {
        match parse(ftl_content) {
            Ok(root) => {
                let translator = OakFluentTranslatorCore::new(root);
                self.translators.insert(locale.to_string(), translator);
                Ok(())
            }
            Err(err) => Err(I18nError::ParseError(format!("{:?}", err))),
        }
    }
}

impl TranslationProvider for OakFluentTranslator {
    fn translate(&self, key: &str, args: Option<&HashMap<String, String>>, locale: &str) -> Result<String, I18nError> {
        // 尝试从当前语言获取翻译
        if let Some(translator) = self.translators.get(locale)
            && let Some(translation) = translator.translate(key, args.unwrap_or(&HashMap::new())) {
            return Ok(translation);
        }

        // 回退到默认语言
        if locale != self.default_locale
            && let Some(translator) = self.translators.get(&self.default_locale)
            && let Some(translation) = translator.translate(key, args.unwrap_or(&HashMap::new())) {
            return Ok(translation);
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

// 实现 Send + Sync 因为 OakFluentTranslatorCore 是可克隆的且线程安全的
unsafe impl Send for OakFluentTranslator {}

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
            translation_cache: lru::LruCache::new(NonZeroUsize::new(self.cache_capacity).unwrap()),
            cache_capacity: self.cache_capacity,
        }
    }
}

impl I18nContext {
    /// 创建新的国际化上下文
    pub fn new(translator: Box<dyn TranslationProvider + Send + Sync>) -> Self {
        Self { translator, translation_cache: lru::LruCache::new(NonZeroUsize::new(1000).unwrap()), cache_capacity: 1000 }
    }

    /// 创建带有自定义缓存容量的国际化上下文
    pub fn with_cache_capacity(translator: Box<dyn TranslationProvider + Send + Sync>, capacity: usize) -> Self {
        Self {
            translator,
            translation_cache: lru::LruCache::new(NonZeroUsize::new(capacity).unwrap()),
            cache_capacity: capacity,
        }
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
        let result = match self.translator.translate(key, args, locale) {
            Ok(translation) => translation,
            Err(e) => {
                tracing::debug!("Translation error: {}, falling back to key", e);
                key.to_string()
            }
        };

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
        // 创建默认的 Oak Fluent 翻译提供者
        let mut translator = OakFluentTranslator::new("en", "en");

        // 尝试加载默认语言资源
        let default_locale = std::env::var("LANG").unwrap_or_else(|_| "en".to_string());

        // 加载英文资源
        if let Ok(english_path) = std::env::var("ITOOLS_LOCALE_EN") {
            let path = Path::new(&english_path);
            if path.exists() && let Err(e) = translator.add_translations_from_file("en", path) {
                info!("Failed to load English translations: {}", e);
            }
        }

        // 加载当前语言资源
        if default_locale != "en" && let Ok(locale_path) = std::env::var(format!("ITOOLS_LOCALE_{}", default_locale.to_uppercase().replace('-', "_"))) {
            let path = Path::new(&locale_path);
            if path.exists() && let Err(e) = translator.add_translations_from_file(&default_locale, path) {
                info!("Failed to load {} translations: {}", default_locale, e);
            }
        }

        std::sync::Mutex::new(I18nContext::new(Box::new(translator)))
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
