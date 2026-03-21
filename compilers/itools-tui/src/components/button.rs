//! 按钮组件

use crate::{event::Event, layout::Rect, render::Frame, style::Style};
use crossterm::event::KeyCode;

/// 按钮组件
pub struct Button {
    /// 按钮标签
    label: String,
    /// 按钮样式
    style: Style,
    /// 按下时的样式
    active_style: Style,
    /// 是否被按下
    is_pressed: bool,
    /// 点击回调
    on_press: Option<Box<dyn Fn()>>,
}

impl Button {
    /// 创建新的按钮
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            style: Style::new(),
            active_style: Style::new().reversed(),
            is_pressed: false,
            on_press: None,
        }
    }

    /// 设置按钮样式
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// 设置按下时的样式
    pub fn active_style(mut self, style: Style) -> Self {
        self.active_style = style;
        self
    }

    /// 设置点击回调
    pub fn on_press<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_press = Some(Box::new(f));
        self
    }

    /// 点击按钮
    pub fn press(&mut self) {
        self.is_pressed = true;
        if let Some(callback) = &self.on_press {
            callback();
        }
    }
}

impl super::Component for Button {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let style = if self.is_pressed { self.active_style.clone() } else { self.style.clone() };

        frame.render_button(&self.label, area, style);
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Enter {
                    self.press();
                    true
                }
                else {
                    false
                }
            }
            _ => false,
        }
    }
}
