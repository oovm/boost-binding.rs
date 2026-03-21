//! 组件系统模块
//!
//! 提供各种 TUI 组件，包括按钮、文本框、列表、表格等。

/// 组件 trait
pub trait Component {
    /// 渲染组件
    fn render(&self, frame: &mut Frame, area: Rect);

    /// 处理事件
    fn handle_event(&mut self, event: &Event) -> bool;
}

/// 按钮组件
pub mod button;

/// 文本框组件
pub mod input;

/// 列表组件
pub mod list;

/// 表格组件
pub mod table;

/// 面板组件
pub mod panel;

/// 进度条组件
pub mod progress_bar;

/// 加载动画组件
pub mod loading_animation;

/// 选择菜单组件
pub mod select_menu;

/// 导出组件
pub use button::Button;
pub use input::Input;
pub use list::List;
pub use loading_animation::LoadingAnimation;
pub use panel::Panel;
pub use progress_bar::ProgressBar;
pub use select_menu::SelectMenu;
pub use table::Table;

/// 从其他模块导入必要的类型
use crate::event::Event;
use crate::{layout::Rect, render::Frame};
