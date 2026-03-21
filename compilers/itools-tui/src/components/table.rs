//! 表格组件

use crate::{event::Event, layout::Rect, render::Frame, style::Style};
use crossterm::event::KeyCode;

/// 表格单元格
pub struct Cell {
    /// 单元格内容
    content: String,
    /// 单元格样式
    style: Style,
}

impl Cell {
    /// 创建新的单元格
    pub fn new(content: &str) -> Self {
        Self { content: content.to_string(), style: Style::new() }
    }

    /// 设置单元格样式
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// 获取单元格内容
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 获取单元格样式
    pub fn get_style(&self) -> &Style {
        &self.style
    }
}

/// 表格行
pub struct Row {
    /// 行单元格
    cells: Vec<Cell>,
    /// 行样式
    style: Style,
    /// 选中样式
    selected_style: Style,
}

impl Row {
    /// 创建新的行
    pub fn new(cells: Vec<Cell>) -> Self {
        Self { cells, style: Style::new(), selected_style: Style::new().reversed() }
    }

    /// 设置行样式
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// 设置选中样式
    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    /// 获取行单元格
    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    /// 获取行样式
    pub fn get_style(&self) -> &Style {
        &self.style
    }

    /// 获取选中样式
    pub fn get_selected_style(&self) -> &Style {
        &self.selected_style
    }
}

/// 表格组件
pub struct Table {
    /// 表头
    header: Option<Row>,
    /// 表格行
    rows: Vec<Row>,
    /// 选中行索引
    selected: usize,
    /// 表格样式
    style: Style,
    /// 选中回调
    on_select: Option<Box<dyn Fn(usize)>>,
}

impl Table {
    /// 创建新的表格
    pub fn new(rows: Vec<Row>) -> Self {
        Self { header: None, rows, selected: 0, style: Style::new(), on_select: None }
    }

    /// 设置表头
    pub fn header(mut self, header: Row) -> Self {
        self.header = Some(header);
        self
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

    /// 获取选中行索引
    pub fn get_selected(&self) -> usize {
        self.selected
    }

    /// 设置选中行
    pub fn set_selected(&mut self, index: usize) {
        if index < self.rows.len() {
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
                if self.selected < self.rows.len() - 1 {
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
                self.selected = self.rows.len().saturating_sub(1);
                if let Some(callback) = &self.on_select {
                    callback(self.selected);
                }
            }
            _ => {}
        }
    }
}

impl super::Component for Table {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_table(self.header.as_ref(), &self.rows, self.selected, area, self.style.clone());
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
