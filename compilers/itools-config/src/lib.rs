#![warn(missing_docs)]

//! iTools 配置解析模块
//! 
//! 提供基于 fragment 的配置解析功能，支持将配置文件分割成多个片段进行管理和加载。

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, error::Error, fs, path::Path};
use tracing::info;

/// 配置错误类型
#[derive(Debug)]
pub enum ConfigError {
    /// 文件未找到
    FileNotFound(String),
    /// 解析错误
    ParseError(String),
    /// 不支持的格式
    UnsupportedFormat(String),
    /// 验证错误
    ValidationError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => write!(f, "File not found: {}", path),
            ConfigError::ParseError(msg) => write!(f, "{}", msg),
            ConfigError::UnsupportedFormat(format) => write!(f, "Unsupported config format: {}", format),
            ConfigError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for ConfigError {}

/// 配置值类型
/// 
/// 表示配置中的各种值类型，包括字符串、数字、布尔值、数组和对象
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigValue {
    /// 字符串类型
    String(String),
    /// 数字类型
    Number(f64),
    /// 布尔类型
    Boolean(bool),
    /// 数组类型
    Array(Vec<ConfigValue>),
    /// 对象类型
    Object(HashMap<String, ConfigValue>),
    /// 空值
    Null,
}

#[cfg(feature = "json")]
use oak_json::{JsonValueNode, ast::{JsonObject, JsonField, JsonString, JsonBoolean, JsonNumber}};

/// 配置片段
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigFragment {
    /// 配置数据
    pub data: ConfigValue,
    /// 优先级
    pub priority: u32,
}

/// 配置管理器
#[derive(Debug)]
pub struct ConfigManager {
    /// 配置片段
    fragments: Vec<ConfigFragment>,
    /// 合并后的配置
    merged_config: ConfigValue,
}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new() -> Self {
        Self { fragments: Vec::new(), merged_config: ConfigValue::Object(HashMap::new()) }
    }

    /// 加载配置片段
    pub fn load_fragment(&mut self, path: &Path) -> Result<(), ConfigError> {
        info!("Loading config fragment: {}", path.to_string_lossy());

        let content = fs::read_to_string(path)
            .map_err(|_e| ConfigError::FileNotFound(path.to_string_lossy().to_string()))?;

        let data = match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => {
                #[cfg(feature = "json")]
                {
                    let json_value: oak_json::JsonValueNode = oak_json::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;
                    ConfigValue::from(json_value)
                }
                #[cfg(not(feature = "json"))]
                {
                    return Err(ConfigError::UnsupportedFormat("JSON feature is not enabled".to_string()));
                }
            }
            Some("toml") => {
                #[cfg(feature = "toml")]
                {
                    let toml_value: oak_toml::TomlValue = oak_toml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;
                    ConfigValue::from(toml_value)
                }
                #[cfg(not(feature = "toml"))]
                {
                    return Err(ConfigError::UnsupportedFormat("TOML feature is not enabled".to_string()));
                }
            }
            Some("yaml") | Some("yml") => {
                #[cfg(feature = "yaml")]
                {
                    let yaml_value: oak_yaml::YamlValue = oak_yaml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;
                    ConfigValue::from(yaml_value)
                }
                #[cfg(not(feature = "yaml"))]
                {
                    return Err(ConfigError::UnsupportedFormat("YAML feature is not enabled".to_string()));
                }
            }
            _ => return Err(ConfigError::UnsupportedFormat("Unsupported config file format".to_string())),
        };

        // 验证配置
        self.validate_config(&data)?;

        let fragment = ConfigFragment { data, priority: self.fragments.len() as u32 };

        self.fragments.push(fragment);
        self.merge_configs();

        Ok(())
    }

    /// 验证配置
    pub fn validate_config(&self, config: &ConfigValue) -> Result<(), ConfigError> {
        // 这里可以添加配置验证逻辑
        // 例如：检查必需的配置项、验证配置值的类型和范围等

        // 示例：检查是否存在必需的配置项
        if let ConfigValue::Object(_map) = config {
            // 这里可以添加具体的验证逻辑
        }

        Ok(())
    }

    /// 验证合并后的配置
    pub fn validate_merged_config(&self) -> Result<(), ConfigError> {
        self.validate_config(&self.merged_config)
    }

    /// 加载目录中的配置片段
    pub fn load_directory(&mut self, path: &Path) -> Result<(), ConfigError> {
        if !path.exists() || !path.is_dir() {
            return Ok(());
        }

        let entries = fs::read_dir(path).map_err(|e| ConfigError::ParseError(e.to_string()))?;
        for entry in entries {
            let entry = entry.map_err(|e| ConfigError::ParseError(e.to_string()))?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                self.load_directory(&entry_path)?;
            }
            else if let Some(ext) = entry_path.extension() {
                if ["json", "toml", "yaml", "yml"].contains(&ext.to_str().unwrap_or("")) {
                    self.load_fragment(&entry_path)?;
                }
            }
        }

        Ok(())
    }

    /// 合并配置
    fn merge_configs(&mut self) {
        // 按优先级排序（优先级高的在后面，会覆盖前面的）
        self.fragments.sort_by(|a, b| a.priority.cmp(&b.priority));

        // 初始化合并后的配置
        self.merged_config = ConfigValue::Object(HashMap::new());

        // 合并所有配置片段
        for fragment in &self.fragments {
            self.merged_config = self.merge_values(self.merged_config.clone(), fragment.data.clone());
        }
    }

    /// 合并两个值
    fn merge_values(&self, target: ConfigValue, source: ConfigValue) -> ConfigValue {
        match (target, source) {
            (ConfigValue::Object(target_map), ConfigValue::Object(source_map)) => {
                // 构建target字段的HashMap以便快速查找
                let mut target_fields = target_map;
                
                // 处理source对象的所有字段
                for (key, source_value) in source_map {
                    if target_fields.contains_key(&key) {
                        let target_value = target_fields.remove(&key).unwrap();
                        target_fields.insert(key, self.merge_values(target_value, source_value));
                    }
                    else {
                        target_fields.insert(key, source_value);
                    }
                }
                
                ConfigValue::Object(target_fields)
            }
            (_, source) => source, // 其他类型直接替换
        }
    }

    /// 获取合并后的配置
    pub fn get_config(&self) -> &ConfigValue {
        &self.merged_config
    }

    /// 从命令行参数覆盖配置
    pub fn override_from_args(&mut self, args: &[String]) {
        // 简单实现：支持 --config.key=value 格式
        for arg in args {
            if arg.starts_with("--config.") {
                let parts: Vec<&str> = arg.split('=').collect();
                if parts.len() == 2 {
                    let key_path = parts[0].trim_start_matches("--config.");
                    let value = parts[1];
                    self.set_config_value(key_path, value);
                }
            }
        }
    }

    /// 从环境变量覆盖配置
    pub fn override_from_env(&mut self) {
        // 简单实现：支持 ITOOLS_CONFIG_ 前缀的环境变量
        for (key, value) in env::vars() {
            if key.starts_with("ITOOLS_CONFIG_") {
                let key_path = key.trim_start_matches("ITOOLS_CONFIG_").to_lowercase().replace('_', ".");
                self.set_config_value(&key_path, &value);
            }
        }
    }

    /// 添加默认配置值
    pub fn add_defaults(&mut self, defaults: ConfigValue) {
        let fragment = ConfigFragment { data: defaults, priority: 0 };
        self.fragments.push(fragment);
        self.merge_configs();
    }

    /// 设置配置值
    fn set_config_value(&mut self, key_path: &str, value: &str) {
        let keys: Vec<&str> = key_path.split('.').collect();
        let mut config = self.merged_config.clone();

        // 导航到目标位置
        let mut current = &mut config;
        for (i, key) in keys.iter().enumerate() {
            if i == keys.len() - 1 {
                // 最后一个键，设置值
                *current = self.parse_value(value);
            }
            else {
                // 确保当前是对象
                if !matches!(current, ConfigValue::Object(_)) {
                    *current = ConfigValue::Object(HashMap::new());
                }
                
                // 查找或创建字段
                if let ConfigValue::Object(current_map) = current {
                    if !current_map.contains_key(*key) {
                        current_map.insert(key.to_string(), ConfigValue::Object(HashMap::new()));
                    }
                    // 确保字段值是对象
                    if let ConfigValue::Object(_) = current_map.get(*key).unwrap() {
                        current = current_map.get_mut(*key).unwrap();
                    } else {
                        current_map.insert(key.to_string(), ConfigValue::Object(HashMap::new()));
                        current = current_map.get_mut(*key).unwrap();
                    }
                }
            }
        }

        self.merged_config = config;
    }

    /// 解析值
    fn parse_value(&self, value: &str) -> ConfigValue {
        // 尝试解析为不同类型
        if value == "true" {
            ConfigValue::Boolean(true)
        }
        else if value == "false" {
            ConfigValue::Boolean(false)
        }
        else if let Ok(num) = value.parse::<i64>() {
            ConfigValue::Number(num as f64)
        }
        else if let Ok(num) = value.parse::<f64>() {
            ConfigValue::Number(num)
        }
        else {
            ConfigValue::String(value.to_string())
        }
    }
}
