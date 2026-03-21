//! 事件处理模块
//!
//! 提供 GUI 事件系统，包括事件定义、分发和处理。

use serde::{Deserialize, Serialize};

/// 事件类型
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum EventType {
    /// 鼠标事件
    Mouse,
    /// 键盘事件
    Keyboard,
    /// 窗口事件
    Window,
    /// 自定义事件
    Custom(String),
}

/// 鼠标事件
#[derive(Debug, Deserialize, Serialize)]
pub struct MouseEvent {
    /// 鼠标位置 x 坐标
    pub x: i32,
    /// 鼠标位置 y 坐标
    pub y: i32,
    /// 鼠标按钮
    pub button: MouseButton,
    /// 事件类型
    pub event_type: MouseEventType,
}

/// 鼠标按钮
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum MouseButton {
    /// 左键
    Left,
    /// 右键
    Right,
    /// 中键
    Middle,
    /// 其他按钮
    Other(u32),
}

/// 鼠标事件类型
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum MouseEventType {
    /// 按下
    Press,
    /// 释放
    Release,
    /// 移动
    Move,
    /// 滚轮
    Wheel,
}

/// 键盘事件
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyboardEvent {
    /// 按键
    pub key: Key,
    /// 事件类型
    pub event_type: KeyboardEventType,
    /// 是否按下修饰键
    pub modifiers: Modifiers,
}

/// 按键
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Key {
    /// 字母键
    Alpha(char),
    /// 数字键
    Digit(u32),
    /// 功能键
    Function(u32),
    /// 方向键
    Arrow(ArrowKey),
    /// 特殊键
    Special(SpecialKey),
}

/// 方向键
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ArrowKey {
    /// 上
    Up,
    /// 下
    Down,
    /// 左
    Left,
    /// 右
    Right,
}

/// 特殊键
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum SpecialKey {
    /// Enter 键
    Enter,
    /// Tab 键
    Tab,
    /// Shift 键
    Shift,
    /// Ctrl 键
    Ctrl,
    /// Alt 键
    Alt,
    /// Space 键
    Space,
    /// Escape 键
    Escape,
    /// Backspace 键
    Backspace,
    /// Delete 键
    Delete,
    /// Home 键
    Home,
    /// End 键
    End,
    /// Page Up 键
    PageUp,
    /// Page Down 键
    PageDown,
}

/// 键盘事件类型
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum KeyboardEventType {
    /// 按下
    Press,
    /// 释放
    Release,
    /// 重复
    Repeat,
}

/// 修饰键状态
#[derive(Debug, Deserialize, Serialize)]
pub struct Modifiers {
    /// 是否按下 Shift
    pub shift: bool,
    /// 是否按下 Ctrl
    pub ctrl: bool,
    /// 是否按下 Alt
    pub alt: bool,
    /// 是否按下 Meta
    pub meta: bool,
}

/// 窗口事件
#[derive(Debug, Deserialize, Serialize)]
pub struct WindowEvent {
    /// 事件类型
    pub event_type: WindowEventType,
    /// 窗口大小（如果适用）
    pub size: Option<(u32, u32)>,
}

/// 窗口事件类型
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum WindowEventType {
    /// 窗口创建
    Created,
    /// 窗口关闭
    Closed,
    /// 窗口调整大小
    Resized,
    /// 窗口移动
    Moved,
    /// 窗口激活
    Activated,
    /// 窗口失活
    Deactivated,
    /// 窗口最小化
    Minimized,
    /// 窗口最大化
    Maximized,
    /// 窗口还原
    Restored,
}

/// 事件结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    /// 事件类型
    pub event_type: EventType,
    /// 事件数据
    pub data: EventData,
}

/// 事件数据
#[derive(Debug, Deserialize, Serialize)]
pub enum EventData {
    /// 鼠标事件数据
    Mouse(MouseEvent),
    /// 键盘事件数据
    Keyboard(KeyboardEvent),
    /// 窗口事件数据
    Window(WindowEvent),
    /// 自定义事件数据
    Custom(serde_json::Value),
}

/// 事件处理器
pub type EventHandler = Box<dyn Fn(&Event) -> bool + Send + Sync>;

/// 事件管理器
pub struct EventManager {
    /// 事件处理器映射
    handlers: Vec<(EventType, EventHandler)>,
}

impl EventManager {
    /// 创建新的事件管理器
    ///
    /// # 返回值
    /// 事件管理器实例
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    /// 添加事件处理器
    ///
    /// # 参数
    /// - `event_type`: 事件类型
    /// - `handler`: 事件处理器
    pub fn add_handler(&mut self, event_type: EventType, handler: EventHandler) {
        self.handlers.push((event_type, handler));
    }

    /// 分发事件
    ///
    /// # 参数
    /// - `event`: 事件
    ///
    /// # 返回值
    /// 是否有处理器处理了该事件
    pub fn dispatch(&self, event: &Event) -> bool {
        let mut handled = false;
        for (event_type, handler) in &self.handlers {
            if *event_type == event.event_type || matches!(event_type, EventType::Custom(_)) {
                if handler(event) {
                    handled = true;
                }
            }
        }
        handled
    }
}
