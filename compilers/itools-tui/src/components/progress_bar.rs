//! 进度条组件

use crate::{layout::Rect, render::Frame, style::Style};

/// 进度条组件
pub struct ProgressBar {
    /// 总进度
    total: u64,
    /// 当前进度
    current: u64,
    /// 进度条宽度
    width: u16,
    /// 进度条样式
    style: Style,
    /// 填充样式
    filled_style: Style,
}

impl ProgressBar {
    /// 创建新的进度条
    pub fn new(total: u64, width: u16) -> Self {
        Self { total, current: 0, width, style: Style::new(), filled_style: Style::new().reversed() }
    }

    /// 设置样式
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// 设置填充样式
    pub fn filled_style(mut self, style: Style) -> Self {
        self.filled_style = style;
        self
    }

    /// 更新进度
    pub fn update(&mut self, current: u64) {
        self.current = current;
    }

    /// 增加进度
    pub fn inc(&mut self, delta: u64) {
        self.current += delta;
    }

    /// 完成进度
    pub fn finish(&mut self) {
        self.current = self.total;
    }

    /// 获取当前进度百分比
    pub fn percentage(&self) -> f64 {
        if self.total > 0 { (self.current as f64 / self.total as f64) * 100.0 } else { 0.0 }
    }
}

impl super::Component for ProgressBar {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_progress_bar(self.current, self.total, self.width, area, self.style.clone(), self.filled_style.clone());
    }

    fn handle_event(&mut self, _event: &crate::event::Event) -> bool {
        false
    }
}
