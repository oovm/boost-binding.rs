#![warn(missing_docs)]

//! iTools 配置解析模块
//!
//! 提供基于 fragment 的配置解析功能，支持将配置文件分割成多个片段进行管理和加载。

use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs, path::Path};
use tracing::info;

/// 国际化功能 - 使用 itools-localization 模块的国际化实现
pub use itools_localization::{t, t_with_args};

/// 命令行参数解析 trait
pub trait Parse {
    /// 从命令行参数中解析当前类型
    fn parse(args: &mut Vec<String>) -> Self;
}

/// 为 String 实现 Parse trait
impl Parse for String {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t("errors.expected_string_argument"));
        }
        args.remove(0)
    }
}

/// 为 i32 实现 Parse trait
impl Parse for i32 {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t("errors.expected_integer_argument"));
        }
        args.remove(0).parse().expect(&t("errors.invalid_integer"))
    }
}

/// 为 u32 实现 Parse trait
impl Parse for u32 {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t("errors.expected_unsigned_integer_argument"));
        }
        args.remove(0).parse().expect(&t("errors.invalid_unsigned_integer"))
    }
}

/// 为 bool 实现 Parse trait
impl Parse for bool {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t("errors.expected_boolean_argument"));
        }
        args.remove(0).parse().expect(&t("errors.invalid_boolean"))
    }
}

/// 配置片段
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigFragment {
    /// 配置数据
    pub data: serde_json::Value,
    /// 优先级
    pub priority: u32,
}

/// 配置管理器
#[derive(Debug)]
pub struct ConfigManager {
    /// 配置片段
    fragments: Vec<ConfigFragment>,
    /// 合并后的配置
    merged_config: serde_json::Value,
}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new() -> Self {
        Self { fragments: Vec::new(), merged_config: serde_json::Value::Object(serde_json::Map::new()) }
    }

    /// 加载配置片段
    pub fn load_fragment(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        info!("{}", t_with_args("messages.loading_config_fragment", &[("path", &path.to_string_lossy())]));

        let content = fs::read_to_string(path)
            .map_err(|_e| format!("{}", t_with_args("errors.config_not_found", &[("path", &path.to_string_lossy())])))?;

        let data = match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str(&content)?,
            Some("toml") => {
                let toml_value: toml::Value = toml::from_str(&content)?;
                serde_json::to_value(toml_value)?
            }
            Some("yaml") | Some("yml") => {
                let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)?;
                serde_json::to_value(yaml_value)?
            }
            _ => return Err("Unsupported config file format".into()),
        };

        // 验证配置
        self.validate_config(&data)?;

        let fragment = ConfigFragment { data, priority: self.fragments.len() as u32 };

        self.fragments.push(fragment);
        self.merge_configs();

        Ok(())
    }

    /// 验证配置
    pub fn validate_config(&self, config: &serde_json::Value) -> Result<(), Box<dyn Error>> {
        // 这里可以添加配置验证逻辑
        // 例如：检查必需的配置项、验证配置值的类型和范围等

        // 示例：检查是否存在必需的配置项
        if let serde_json::Value::Object(_map) = config {
            // 这里可以添加具体的验证逻辑
        }

        Ok(())
    }

    /// 验证合并后的配置
    pub fn validate_merged_config(&self) -> Result<(), Box<dyn Error>> {
        self.validate_config(&self.merged_config)
    }

    /// 加载目录中的配置片段
    pub fn load_directory(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        if !path.exists() || !path.is_dir() {
            return Ok(());
        }

        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
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
        self.merged_config = serde_json::Value::Object(serde_json::Map::new());

        // 合并所有配置片段
        for fragment in &self.fragments {
            self.merged_config = self.merge_values(self.merged_config.clone(), fragment.data.clone());
        }
    }

    /// 合并两个值
    fn merge_values(&self, target: serde_json::Value, source: serde_json::Value) -> serde_json::Value {
        match (target, source) {
            (serde_json::Value::Object(mut target_map), serde_json::Value::Object(source_map)) => {
                for (key, source_value) in source_map {
                    if target_map.contains_key(&key) {
                        let target_value = target_map.remove(&key).unwrap();
                        target_map.insert(key, self.merge_values(target_value, source_value));
                    }
                    else {
                        target_map.insert(key, source_value);
                    }
                }
                serde_json::Value::Object(target_map)
            }
            (_, source) => source, // 其他类型直接替换
        }
    }

    /// 获取合并后的配置
    pub fn get_config(&self) -> &serde_json::Value {
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
                // 创建嵌套对象
                if !current.is_object() {
                    *current = serde_json::Value::Object(serde_json::Map::new());
                }
                let current_map = current.as_object_mut().unwrap();
                if !current_map.contains_key(*key) {
                    current_map.insert(key.to_string(), serde_json::Value::Object(serde_json::Map::new()));
                }
                current = current_map.get_mut(*key).unwrap();
            }
        }

        self.merged_config = config;
    }

    /// 解析值
    fn parse_value(&self, value: &str) -> serde_json::Value {
        // 尝试解析为不同类型
        if value == "true" {
            serde_json::Value::Bool(true)
        }
        else if value == "false" {
            serde_json::Value::Bool(false)
        }
        else if let Ok(num) = value.parse::<i64>() {
            serde_json::Value::Number(serde_json::Number::from(num))
        }
        else if let Ok(num) = value.parse::<f64>() {
            serde_json::Value::Number(serde_json::Number::from_f64(num).unwrap())
        }
        else {
            serde_json::Value::String(value.to_string())
        }
    }
}

/// iTools 命令行工具
#[derive(Debug)]
pub struct Cli {
    /// 命令
    pub command: Command,
}

/// 命令枚举
#[derive(Debug)]
pub enum Command {
    /// 初始化新项目
    Init {
        /// 项目名称
        name: String,
    },
    /// 构建项目
    Build,
    /// 开发服务器
    Dev,
    /// 检查项目
    Check,
}

/// 为 Cli 实现 Parse trait
impl Parse for Cli {
    fn parse(args: &mut Vec<String>) -> Self {
        Self { command: Command::parse(args) }
    }
}

/// 为 Command 实现 Parse trait
impl Parse for Command {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t("errors.expected_command"));
        }
        let cmd = args.remove(0);
        match cmd.as_str() {
            "init" => Self::Init { name: String::parse(args) },
            "build" => Self::Build,
            "dev" => Self::Dev,
            "check" => Self::Check,
            _ => panic!("{}", t_with_args("errors.unknown_command", &[("command", &cmd)])),
        }
    }
}

/// 初始化 CLI 环境
pub fn init_cli() {
    // 初始化日志
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();

    info!("{}", t("messages.cli_initialized"));
}

/// TUI 功能模块
pub mod tui {
    use std::{
        io::{self, Write},
        thread,
        time::Duration,
    };

    /// 进度条
    pub struct ProgressBar {
        total: u64,
        current: u64,
        width: u16,
    }

    impl ProgressBar {
        /// 创建新的进度条
        pub fn new(total: u64, width: u16) -> Self {
            Self { total, current: 0, width }
        }

        /// 更新进度
        pub fn update(&mut self, current: u64) {
            self.current = current;
            self.draw();
        }

        /// 增加进度
        pub fn inc(&mut self, delta: u64) {
            self.current += delta;
            self.draw();
        }

        /// 完成进度
        pub fn finish(&mut self) {
            self.current = self.total;
            self.draw();
            println!();
        }

        /// 绘制进度条
        fn draw(&self) {
            let percentage = if self.total > 0 { (self.current as f64 / self.total as f64) * 100.0 } else { 0.0 };

            let filled = (percentage / 100.0 * self.width as f64) as u16;
            let empty = self.width - filled;

            print!("\r[");
            print!("{}", "=".repeat(filled as usize));
            print!("{}", " ".repeat(empty as usize));
            print!("}}] {:.1}%", percentage);
            io::stdout().flush().unwrap();
        }
    }

    /// 交互式选择菜单
    pub fn select<T>(items: &[T], prompt: &str) -> Option<usize>
    where
        T: std::fmt::Display,
    {
        println!("{}", prompt);
        for (i, item) in items.iter().enumerate() {
            println!("{}: {}", i + 1, item);
        }

        loop {
            print!("请选择 (1-{}): ", items.len());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<usize>() {
                Ok(choice) if choice >= 1 && choice <= items.len() => {
                    return Some(choice - 1);
                }
                _ => {
                    println!("无效的选择，请重新输入");
                }
            }
        }
    }

    /// 显示加载动画
    pub fn loading_animation(duration: Duration, message: &str) {
        let chars = ["|", "/", "-", "\\"];
        let start = std::time::Instant::now();

        print!("{}", message);
        io::stdout().flush().unwrap();

        let mut i = 0;
        while start.elapsed() < duration {
            print!("\r{}{}", message, chars[i % chars.len()]);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
            i += 1;
        }

        print!("\r{}{}", message, " ");
        println!();
    }
}

/// 执行命令
pub fn execute_command(cli: Cli) -> Result<(), Box<dyn Error>> {
    match cli.command {
        Command::Init { name } => {
            info!("{}", t_with_args("messages.initializing_project", &[("name", &name)]));
            // 测试进度条
            let mut pb = tui::ProgressBar::new(100, 50);
            for i in 0..=100 {
                pb.update(i);
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            pb.finish();
            Ok(())
        }
        Command::Build => {
            info!("{}", t("messages.building_project"));
            // 测试加载动画
            tui::loading_animation(std::time::Duration::from_secs(3), "构建中...");
            Ok(())
        }
        Command::Dev => {
            info!("{}", t("messages.starting_dev_server"));
            // 测试交互式选择菜单
            let items = ["选项 1", "选项 2", "选项 3"];
            if let Some(choice) = tui::select(&items, "请选择一个选项:") {
                info!("选择了: {}", items[choice]);
            }
            Ok(())
        }
        Command::Check => {
            info!("{}", t("messages.checking_project"));
            // 测试进度条
            let mut pb = tui::ProgressBar::new(50, 30);
            for i in 0..=50 {
                pb.update(i);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            pb.finish();
            Ok(())
        }
    }
}

/// 测试宏功能
pub mod test_macro;
pub use test_macro::test_macro;
