#![warn(missing_docs)]

//! iTools 核心功能模块
//! 
//! 提供核心功能，包括可本地化错误类型、命令行解析、工具函数和通用组件。

/// 可本地化错误 trait
/// 
/// 实现此 trait 的错误类型可以自动生成本地化的错误消息
pub trait LocalizableError: std::error::Error {
    /// 获取错误的本地化键
    /// 
    /// # Returns
    /// - 错误的本地化键字符串
    fn localization_key(&self) -> &str;

    /// 获取错误的本地化消息
    /// 
    /// # Arguments
    /// - `locale`: 语言代码，如 "en" 或 "zh-CN"，
    ///   传空字符串 "" 使用全局设置的语言
    /// 
    /// # Returns
    /// - 本地化的错误消息字符串
    fn localize(&self, locale: &str) -> String;
}

/// 错误类型宏
/// 
/// 用于创建可本地化的错误类型
/// 
/// # 语法
/// 
/// 支持两种形式：
/// 1. 简单形式（无参数）：`Variant => "key"`
/// 2. 带参数形式：`Variant(ty1, ty2) => "key"`
#[macro_export]
macro_rules! localizable_error {
    ($name:ident {
        $($variant:ident $(($($field:ty),*))? => $key:expr),* $(,)?
    }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $name {
            $($variant $(($($field),*))?,
            )*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.localize(""))
            }
        }

        impl std::error::Error for $name {}

        impl $crate::LocalizableError for $name {
            fn localization_key(&self) -> &str {
                match self {
                    $(
                        Self::$variant $(($($field),*))? => $key,
                    )*
                }
            }

            fn localize(&self, _locale: &str) -> String {
                match self {
                    $(
                        Self::$variant $(($($field),*))? => {
                            $(let ($($field),*) = ($($field),*);)?
                            itools_localization::t($key)
                        }
                    )*
                }
            }
        }
    };
}

/// 命令行参数解析模块
pub mod cli {
    /// 命令行参数解析 trait
    pub trait Parse {
        /// 从命令行参数中解析当前类型
        fn parse(args: &mut Vec<String>) -> Self;
    }

    /// 为 String 实现 Parse trait
    impl Parse for String {
        fn parse(args: &mut Vec<String>) -> Self {
            if args.is_empty() {
                panic!("Expected string argument");
            }
            args.remove(0)
        }
    }

    /// 为 i32 实现 Parse trait
    impl Parse for i32 {
        fn parse(args: &mut Vec<String>) -> Self {
            if args.is_empty() {
                panic!("Expected integer argument");
            }
            args.remove(0).parse().expect("Invalid integer")
        }
    }

    /// 为 u32 实现 Parse trait
    impl Parse for u32 {
        fn parse(args: &mut Vec<String>) -> Self {
            if args.is_empty() {
                panic!("Expected unsigned integer argument");
            }
            args.remove(0).parse().expect("Invalid unsigned integer")
        }
    }

    /// 为 bool 实现 Parse trait
    impl Parse for bool {
        fn parse(args: &mut Vec<String>) -> Self {
            if args.is_empty() {
                panic!("Expected boolean argument");
            }
            args.remove(0).parse().expect("Invalid boolean")
        }
    }

    /// 命令行参数解析器
    /// 
    /// 提供通用的命令行参数解析功能，支持自定义命令和参数。
    pub struct ArgParser {
        /// 命令映射
        commands: std::collections::HashMap<String, Box<dyn Fn(&mut Vec<String>) -> Result<(), String>>>,
    }

    impl ArgParser {
        /// 创建新的参数解析器
        pub fn new() -> Self {
            Self {
                commands: std::collections::HashMap::new(),
            }
        }
        
        /// 添加命令处理函数
        pub fn add_command<F>(&mut self, name: &str, handler: F)
        where
            F: Fn(&mut Vec<String>) -> Result<(), String> + 'static,
        {
            self.commands.insert(name.to_string(), Box::new(handler));
        }
        
        /// 解析并执行命令
        pub fn parse(&self, args: &mut Vec<String>) -> Result<(), String> {
            if args.is_empty() {
                return Err("No command provided".to_string());
            }
            
            let cmd = args.remove(0);
            if let Some(handler) = self.commands.get(&cmd) {
                handler(args)
            } else {
                Err(format!("Unknown command: {}", cmd))
            }
        }
    }

    /// 命令行选项解析
    /// 
    /// 用于解析命令行选项，如 `--verbose`、`--help` 等。
    pub struct OptionParser {
        /// 选项映射
        options: std::collections::HashMap<String, bool>,
        /// 带值的选项映射
        value_options: std::collections::HashMap<String, Option<String>>,
    }

    impl OptionParser {
        /// 创建新的选项解析器
        pub fn new() -> Self {
            Self {
                options: std::collections::HashMap::new(),
                value_options: std::collections::HashMap::new(),
            }
        }
        
        /// 解析选项
        pub fn parse(&mut self, args: &mut Vec<String>) {
            let mut i = 0;
            while i < args.len() {
                let arg = &args[i];
                if arg.starts_with("--") {
                    let option = arg.trim_start_matches("--");
                    if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                        // 带值的选项
                        self.value_options.insert(option.to_string(), Some(args[i + 1].clone()));
                        args.remove(i + 1);
                        args.remove(i);
                    } else {
                        // 开关选项
                        self.options.insert(option.to_string(), true);
                        args.remove(i);
                    }
                } else {
                    i += 1;
                }
            }
        }
        
        /// 检查选项是否存在
        pub fn has_option(&self, name: &str) -> bool {
            self.options.contains_key(name)
        }
        
        /// 获取选项值
        pub fn get_option_value(&self, name: &str) -> Option<&String> {
            self.value_options.get(name).and_then(|v| v.as_ref())
        }
    }

    /// 命令行工具辅助函数
    pub mod utils {
        /// 分割命令行参数
        pub fn split_args(input: &str) -> Vec<String> {
            let mut args = Vec::new();
            let mut current = String::new();
            let mut in_quotes = false;
            let mut quote_char = '"';
            
            for c in input.chars() {
                match c {
                    '"' | '\'' if !in_quotes => {
                        in_quotes = true;
                        quote_char = c;
                    }
                    c if in_quotes && c == quote_char => {
                        in_quotes = false;
                    }
                    ' ' if !in_quotes => {
                        if !current.is_empty() {
                            args.push(current);
                            current = String::new();
                        }
                    }
                    _ => {
                        current.push(c);
                    }
                }
            }
            
            if !current.is_empty() {
                args.push(current);
            }
            
            args
        }
        
        /// 格式化错误信息
        pub fn format_error(message: &str) -> String {
            format!("Error: {}", message)
        }
    }
}
