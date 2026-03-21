#![warn(missing_docs)]

//! iTools 核心功能模块
//!
//! 提供核心功能，包括可本地化错误类型、工具函数和通用组件。

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
