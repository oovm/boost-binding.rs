//! 面板组件

use crate::{event::Event, layout::Rect, render::Frame, style::Style};

/// 面板组件
pub struct Panel {
    /// 面板标题
    title: Option<String>,
    /// 面板样式
    style: Style,
    /// 边框样式
    border_style: Style,
    /// 内部组件
    content: Box<dyn super::Component>,
}

impl Panel {
    /// 创建新的面板
    pub fn new(content: impl super::Component + 'static) -> Self {
        Self { title: None, style: Style::new(), border_style: Style::new(), content: Box::new(content) }
    }

    /// 设置面板标题
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// 设置面板样式
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// 设置边框样式
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }
}

impl super::Component for Panel {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_panel(self.title.as_deref(), area, self.style.clone(), self.border_style.clone());

        // 渲染内部组件
        let content_area = area.shrink(1);
        self.content.render(frame, content_area);
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        self.content.handle_event(event)
    }
}
