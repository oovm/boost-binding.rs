//! 文本框组件

use crate::{event::Event, layout::Rect, render::Frame, style::Style};
use crossterm::event::{KeyCode, KeyEvent};

/// 文本框组件
pub struct Input {
    /// 输入内容
    value: String,
    /// 提示文本
    placeholder: Option<String>,
    /// 文本框样式
    style: Style,
    /// 光标样式
    cursor_style: Style,
    /// 光标位置
    cursor_pos: usize,
    /// 是否获得焦点
    focused: bool,
    /// 输入回调
    on_change: Option<Box<dyn Fn(&str)>>,
}

impl Input {
    /// 创建新的文本框
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: None,
            style: Style::new(),
            cursor_style: Style::new().reversed(),
            cursor_pos: 0,
            focused: false,
            on_change: None,
        }
    }

    /// 设置初始值
    pub fn value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self.cursor_pos = self.value.len();
        self
    }

    /// 设置提示文本
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = Some(placeholder.to_string());
        self
    }

    /// 设置样式
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// 设置光标样式
    pub fn cursor_style(mut self, style: Style) -> Self {
        self.cursor_style = style;
        self
    }

    /// 设置是否获得焦点
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// 设置输入回调
    pub fn on_change<F: Fn(&str) + 'static>(mut self, f: F) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    /// 获取输入值
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// 处理键盘事件
    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Backspace => {
                if self.cursor_pos > 0 {
                    self.value.remove(self.cursor_pos - 1);
                    self.cursor_pos -= 1;
                    if let Some(callback) = &self.on_change {
                        callback(&self.value);
                    }
                }
            }
            KeyCode::Delete => {
                if self.cursor_pos < self.value.len() {
                    self.value.remove(self.cursor_pos);
                    if let Some(callback) = &self.on_change {
                        callback(&self.value);
                    }
                }
            }
            KeyCode::Left => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_pos < self.value.len() {
                    self.cursor_pos += 1;
                }
            }
            KeyCode::Home => {
                self.cursor_pos = 0;
            }
            KeyCode::End => {
                self.cursor_pos = self.value.len();
            }
            KeyCode::Char(c) => {
                self.value.insert(self.cursor_pos, c);
                self.cursor_pos += 1;
                if let Some(callback) = &self.on_change {
                    callback(&self.value);
                }
            }
            _ => {}
        }
    }
}

impl super::Component for Input {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let text =
            if self.value.is_empty() && self.placeholder.is_some() { self.placeholder.as_ref().unwrap() } else { &self.value };

        frame.render_input(text, self.cursor_pos, area, self.style.clone(), self.cursor_style.clone(), self.focused);
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        if !self.focused {
            return false;
        }

        match event {
            Event::Key(key_event) => {
                self.handle_key(*key_event);
                true
            }
            _ => false,
        }
    }
}
