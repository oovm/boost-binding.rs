#![warn(missing_docs)]

//! iTools TUI 模块
//!
//! 提供完整的终端用户界面 (TUI) 功能，包括组件系统、布局系统、事件处理和样式系统。

/// 终端操作模块
mod terminal;

/// 事件处理模块
mod event;

/// 样式系统模块
mod style;

/// 布局系统模块
mod layout;

/// 组件系统模块
mod components;

/// 工具函数模块
mod utils;

/// 渲染系统模块
mod render;

/// 终端操作相关功能
pub use terminal::*;

/// 事件处理相关功能
pub use event::*;

/// 样式系统相关功能
pub use style::*;

/// 布局系统相关功能
pub use layout::*;

/// 组件系统相关功能
pub use components::*;

/// 工具函数相关功能
pub use utils::*;
pub use utils::text;
pub use utils::color;
pub use utils::terminal_utils;
pub use utils::math;
pub use utils::select;
pub use utils::loading_animation;

/// 渲染系统相关功能
pub use render::*;
