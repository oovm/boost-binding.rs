#![warn(missing_docs)]

//! iTools GUI 模块
//!
//! 提供完整的图形用户界面 (GUI) 功能，参考 Tauri 技术方案但采用自研实现。

/// 应用程序模块
mod app;

/// 窗口模块
mod window;

/// 事件处理模块
mod event;

/// 样式系统模块
mod style;

/// 组件系统模块
mod components;

/// 渲染系统模块
mod render;

/// 工具函数模块
mod utils;

/// 应用程序相关功能
pub use app::{App, AppConfig};

/// 窗口相关功能
pub use window::{Window, WindowConfig, WindowHandle};

/// 事件处理相关功能
pub use event::*;

/// 样式系统相关功能
pub use style::*;

/// 组件系统相关功能
pub use components::*;

/// 渲染系统相关功能
pub use render::*;

/// 工具函数相关功能
pub use utils::*;
