//! 选择菜单组件

use crate::{event::Event, layout::Rect, render::Frame, style::Style};
use crossterm::event::KeyCode;

/// 选择菜单项
pub struct SelectItem {
    /// 项文本
    text: String,
    /// 项样式
    style: Style,
    /// 选中样式
    selected_style: Style,
}

impl SelectItem {
    /// 创建新的选择菜单项
    pub fn new(text: &str) -> Self {
        Self { text: text.to_string(), style: Style::new(), selected_style: Style::new().reversed() }
    }

    /// 设置项样式
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// 设置选中样式
    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    /// 获取项文本
    pub fn text(&self) -> &str {
        &self.text
    }

    /// 获取项样式
    pub fn get_style(&self) -> &Style {
        &self.style
    }

    /// 获取选中样式
    pub fn get_selected_style(&self) -> &Style {
        &self.selected_style
    }
}

/// 选择菜单组件
pub struct SelectMenu {
    /// 选择提示
    prompt: String,
    /// 选择项
    items: Vec<SelectItem>,
    /// 选中索引
    selected: usize,
    /// 菜单样式
    style: Style,
    /// 选中回调
    on_select: Option<Box<dyn Fn(usize)>>,
    /// 取消回调
    on_cancel: Option<Box<dyn Fn()>>,
}

impl SelectMenu {
    /// 创建新的选择菜单
    pub fn new(prompt: &str, items: Vec<SelectItem>) -> Self {
        Self { prompt: prompt.to_string(), items, selected: 0, style: Style::new(), on_select: None, on_cancel: None }
    }

    /// 设置样式
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// 设置选中回调
    pub fn on_select<F: Fn(usize) + 'static>(mut self, f: F) -> Self {
        self.on_select = Some(Box::new(f));
        self
    }

    /// 设置取消回调
    pub fn on_cancel<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_cancel = Some(Box::new(f));
        self
    }

    /// 获取选中项索引
    pub fn get_selected(&self) -> usize {
        self.selected
    }

    /// 处理键盘事件
    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                true
            }
            KeyCode::Down => {
                if self.selected < self.items.len() - 1 {
                    self.selected += 1;
                }
                true
            }
            KeyCode::Enter => {
                if let Some(callback) = &self.on_select {
                    callback(self.selected);
                }
                true
            }
            KeyCode::Esc => {
                if let Some(callback) = &self.on_cancel {
                    callback();
                }
                true
            }
            _ => false,
        }
    }
}

impl super::Component for SelectMenu {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_select_menu(&self.prompt, &self.items, self.selected, area, self.style.clone());
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(key_event) => self.handle_key(key_event.code),
            _ => false,
        }
    }
}
