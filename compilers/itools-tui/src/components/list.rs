//! 列表组件

use crate::{event::Event, layout::Rect, render::Frame, style::Style};
use crossterm::event::KeyCode;

/// 列表项
pub struct ListItem {
    /// 项文本
    text: String,
    /// 项样式
    style: Style,
    /// 选中样式
    selected_style: Style,
}

impl ListItem {
    /// 创建新的列表项
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

/// 列表组件
pub struct List {
    /// 列表项
    items: Vec<ListItem>,
    /// 选中索引
    selected: usize,
    /// 列表样式
    style: Style,
    /// 选中回调
    on_select: Option<Box<dyn Fn(usize)>>,
}

impl List {
    /// 创建新的列表
    pub fn new(items: Vec<ListItem>) -> Self {
        Self { items, selected: 0, style: Style::new(), on_select: None }
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

    /// 获取选中项索引
    pub fn get_selected(&self) -> usize {
        self.selected
    }

    /// 设置选中项
    pub fn set_selected(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected = index;
            if let Some(callback) = &self.on_select {
                callback(self.selected);
            }
        }
    }

    /// 处理键盘事件
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                    if let Some(callback) = &self.on_select {
                        callback(self.selected);
                    }
                }
            }
            KeyCode::Down => {
                if self.selected < self.items.len() - 1 {
                    self.selected += 1;
                    if let Some(callback) = &self.on_select {
                        callback(self.selected);
                    }
                }
            }
            KeyCode::Home => {
                self.selected = 0;
                if let Some(callback) = &self.on_select {
                    callback(self.selected);
                }
            }
            KeyCode::End => {
                self.selected = self.items.len().saturating_sub(1);
                if let Some(callback) = &self.on_select {
                    callback(self.selected);
                }
            }
            _ => {}
        }
    }
}

impl super::Component for List {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_list(&self.items, self.selected, area, self.style.clone());
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(key_event) => {
                self.handle_key(key_event.code);
                true
            }
            _ => false,
        }
    }
}
