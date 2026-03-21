//! 事件处理模块
//!
//! 提供事件处理相关功能，包括键盘事件和鼠标事件。

use crossterm::event as crossterm_event;
use std::{io::Result as CrosstermResult, time::Duration};

/// 事件类型
#[derive(Debug, Clone)]
pub enum Event {
    /// 键盘事件
    Key(crossterm_event::KeyEvent),
    /// 鼠标事件
    Mouse(crossterm_event::MouseEvent),
    /// 终端调整大小事件
    Resize(u16, u16),
    /// 超时事件
    Timeout,
}

/// 事件处理器 trait
pub trait EventHandler {
    /// 处理事件
    ///
    /// 返回 true 表示事件已处理，返回 false 表示事件未处理
    fn handle(&mut self, event: &Event) -> bool;
}

/// 事件源
pub struct EventSource {
    tick_rate: Duration,
}

impl EventSource {
    /// 创建新的事件源
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    /// 接收事件
    pub fn next(&self) -> CrosstermResult<Event> {
        if crossterm_event::poll(self.tick_rate)? {
            match crossterm_event::read()? {
                crossterm_event::Event::Key(key) => Ok(Event::Key(key)),
                crossterm_event::Event::Mouse(mouse) => Ok(Event::Mouse(mouse)),
                crossterm_event::Event::Resize(width, height) => Ok(Event::Resize(width, height)),
                _ => Ok(Event::Timeout),
            }
        }
        else {
            Ok(Event::Timeout)
        }
    }
}
