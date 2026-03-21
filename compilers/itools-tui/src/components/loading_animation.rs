//! 加载动画组件

use crate::{layout::Rect, render::Frame, style::Style};
use std::time::Duration;

/// 加载动画组件
pub struct LoadingAnimation {
    /// 加载消息
    message: String,
    /// 动画帧
    frames: Vec<String>,
    /// 当前帧索引
    current_frame: usize,
    /// 动画速度
    frame_duration: Duration,
    /// 样式
    style: Style,
}

impl LoadingAnimation {
    /// 创建新的加载动画
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            frames: vec!["|".to_string(), "/".to_string(), "-".to_string(), "\\".to_string()],
            current_frame: 0,
            frame_duration: Duration::from_millis(100),
            style: Style::new(),
        }
    }

    /// 设置动画帧
    pub fn frames(mut self, frames: Vec<String>) -> Self {
        self.frames = frames;
        self
    }

    /// 设置动画速度
    pub fn frame_duration(mut self, duration: Duration) -> Self {
        self.frame_duration = duration;
        self
    }

    /// 设置样式
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// 更新动画帧
    pub fn tick(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.frames.len();
    }

    /// 获取当前帧
    pub fn current_frame(&self) -> &str {
        &self.frames[self.current_frame]
    }
}

impl super::Component for LoadingAnimation {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_loading_animation(&self.message, self.current_frame(), area, self.style.clone());
    }

    fn handle_event(&mut self, _event: &crate::event::Event) -> bool {
        false
    }
}
