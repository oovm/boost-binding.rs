//! 渲染系统模块
//!
//! 提供渲染相关功能，包括组件渲染、屏幕绘制等。

use crate::{layout::Rect, style::Style};
use crossterm::{cursor, style as crossterm_style};
use std::io::{self, Write};

/// 帧
pub struct Frame {
    stdout: io::Stdout,
}

impl Frame {
    /// 创建新的帧
    pub fn new() -> Self {
        Self { stdout: io::stdout() }
    }

    /// 渲染按钮
    pub fn render_button(&mut self, label: &str, area: Rect, style: Style) {
        // 计算按钮内容的位置
        let content_x = area.x + 1;
        let content_y = area.y + 1;
        let content_width = area.width.saturating_sub(2);

        // 绘制边框
        self.draw_border(area, style.clone());

        // 绘制按钮标签
        let padded_label = self.pad_text(label, content_width as usize);
        use crossterm::style::PrintStyledContent;
        write!(
            self.stdout,
            "{}{}",
            cursor::MoveTo(content_x, content_y),
            PrintStyledContent(crossterm_style::StyledContent::new(style.to_crossterm_style(), padded_label)),
        )
        .unwrap();

        self.stdout.flush().unwrap();
    }

    /// 渲染输入框
    pub fn render_input(
        &mut self,
        text: &str,
        cursor_pos: usize,
        area: Rect,
        style: Style,
        cursor_style: Style,
        focused: bool,
    ) {
        // 计算输入框内容的位置
        let content_x = area.x + 1;
        let content_y = area.y + 1;
        let content_width = area.width.saturating_sub(2);

        // 绘制边框
        self.draw_border(area, style.clone());

        // 绘制输入文本
        let truncated_text = self.truncate_text(text, content_width as usize);
        use crossterm::style::PrintStyledContent;
        write!(
            self.stdout,
            "{}{}",
            cursor::MoveTo(content_x, content_y),
            PrintStyledContent(crossterm_style::StyledContent::new(style.to_crossterm_style(), truncated_text)),
        )
        .unwrap();

        // 绘制光标
        if focused {
            let cursor_display_pos = std::cmp::min(cursor_pos, content_width as usize);
            write!(
                self.stdout,
                "{}{}",
                cursor::MoveTo(content_x + cursor_display_pos as u16, content_y),
                PrintStyledContent(crossterm_style::StyledContent::new(cursor_style.to_crossterm_style(), " ")),
            )
            .unwrap();
        }

        self.stdout.flush().unwrap();
    }

    /// 渲染列表
    pub fn render_list(&mut self, items: &[crate::components::list::ListItem], selected: usize, area: Rect, style: Style) {
        // 绘制边框
        self.draw_border(area, style.clone());

        // 计算列表内容的位置
        let content_x = area.x + 1;
        let content_y = area.y + 1;
        let content_width = area.width.saturating_sub(2);
        let content_height = area.height.saturating_sub(2);

        // 渲染列表项
        for (i, item) in items.iter().enumerate().take(content_height as usize) {
            let item_style = if i == selected { item.get_selected_style().clone() } else { item.get_style().clone() };

            use crossterm::style::PrintStyledContent;
            let padded_text = self.pad_text(item.text(), content_width as usize);

            write!(
                self.stdout,
                "{}{}",
                cursor::MoveTo(content_x, content_y + i as u16),
                PrintStyledContent(crossterm_style::StyledContent::new(item_style.to_crossterm_style(), padded_text)),
            )
            .unwrap();
        }

        self.stdout.flush().unwrap();
    }

    /// 渲染表格
    pub fn render_table(
        &mut self,
        header: Option<&crate::components::table::Row>,
        rows: &[crate::components::table::Row],
        selected: usize,
        area: Rect,
        style: Style,
    ) {
        // 绘制边框
        self.draw_border(area, style.clone());

        // 计算表格内容的位置
        let content_x = area.x + 1;
        let content_y = area.y + 1;
        let content_width = area.width.saturating_sub(2);
        let content_height = area.height.saturating_sub(2);

        // 计算列宽
        let column_count = rows.first().map(|row| row.cells().len()).unwrap_or(0);
        let column_width = if column_count > 0 { content_width / column_count as u16 } else { content_width };

        // 渲染表头
        if let Some(header_row) = header {
            for (i, cell) in header_row.cells().iter().enumerate() {
                use crossterm::style::PrintStyledContent;
                let padded_text = self.pad_text(cell.content(), column_width as usize);

                write!(
                    self.stdout,
                    "{}{}",
                    cursor::MoveTo(content_x + i as u16 * column_width, content_y),
                    PrintStyledContent(crossterm_style::StyledContent::new(cell.get_style().to_crossterm_style(), padded_text)),
                )
                .unwrap();
            }
        }

        // 渲染表格行
        let start_y = content_y + if header.is_some() { 1 } else { 0 };
        for (i, row) in rows.iter().enumerate().take((content_height - header.is_some() as u16) as usize) {
            let row_style = if i == selected { row.get_selected_style().clone() } else { row.get_style().clone() };

            for (j, cell) in row.cells().iter().enumerate() {
                use crossterm::style::PrintStyledContent;
                let padded_text = self.pad_text(cell.content(), column_width as usize);

                write!(
                    self.stdout,
                    "{}{}",
                    cursor::MoveTo(content_x + j as u16 * column_width, start_y + i as u16),
                    PrintStyledContent(crossterm_style::StyledContent::new(row_style.to_crossterm_style(), padded_text)),
                )
                .unwrap();
            }
        }

        self.stdout.flush().unwrap();
    }

    /// 渲染面板
    pub fn render_panel(&mut self, title: Option<&str>, area: Rect, style: Style, border_style: Style) {
        // 绘制边框
        self.draw_border(area, border_style);

        // 绘制标题
        if let Some(title) = title {
            let title_x = area.x + 2;
            let title_y = area.y;
            use crossterm::style::PrintStyledContent;

            write!(
                self.stdout,
                "{}{}",
                cursor::MoveTo(title_x, title_y),
                PrintStyledContent(crossterm_style::StyledContent::new(style.to_crossterm_style(), title.to_string())),
            )
            .unwrap();
        }

        self.stdout.flush().unwrap();
    }

    /// 渲染进度条
    pub fn render_progress_bar(&mut self, current: u64, total: u64, width: u16, area: Rect, style: Style, filled_style: Style) {
        // 计算进度条位置
        let bar_x = area.x + 1;
        let bar_y = area.y + 1;
        let bar_width = width.min(area.width.saturating_sub(2));

        // 绘制边框
        self.draw_border(area, style.clone());

        // 计算填充长度
        let percentage = if total > 0 { (current as f64 / total as f64) * 100.0 } else { 0.0 };
        let filled_length = (percentage / 100.0 * bar_width as f64) as u16;
        let empty_length = bar_width - filled_length;

        use crossterm::style::PrintStyledContent;
        // 绘制进度条
        write!(
            self.stdout,
            "{}{}{}",
            cursor::MoveTo(bar_x, bar_y),
            PrintStyledContent(crossterm_style::StyledContent::new(
                filled_style.to_crossterm_style(),
                "=".repeat(filled_length as usize)
            )),
            PrintStyledContent(crossterm_style::StyledContent::new(
                style.to_crossterm_style(),
                " ".repeat(empty_length as usize)
            )),
        )
        .unwrap();

        // 绘制百分比
        let percentage_text = format!("{:.1}%", percentage);
        let text_x = area.x + (area.width - percentage_text.len() as u16) / 2;
        let text_y = area.y + 1;

        write!(
            self.stdout,
            "{}{}",
            cursor::MoveTo(text_x, text_y),
            PrintStyledContent(crossterm_style::StyledContent::new(style.to_crossterm_style(), percentage_text)),
        )
        .unwrap();

        self.stdout.flush().unwrap();
    }

    /// 渲染加载动画
    pub fn render_loading_animation(&mut self, message: &str, frame: &str, area: Rect, style: Style) {
        // 计算动画位置
        let content_x = area.x + 1;
        let content_y = area.y + 1;

        // 绘制边框
        self.draw_border(area, style.clone());

        use crossterm::style::PrintStyledContent;
        // 绘制消息和动画帧
        let animation_text = format!("{} {}", message, frame);
        write!(
            self.stdout,
            "{}{}",
            cursor::MoveTo(content_x, content_y),
            PrintStyledContent(crossterm_style::StyledContent::new(style.to_crossterm_style(), animation_text)),
        )
        .unwrap();

        self.stdout.flush().unwrap();
    }

    /// 渲染选择菜单
    pub fn render_select_menu(
        &mut self,
        prompt: &str,
        items: &[crate::components::select_menu::SelectItem],
        selected: usize,
        area: Rect,
        style: Style,
    ) {
        // 绘制边框
        self.draw_border(area, style.clone());

        // 计算菜单内容的位置
        let content_x = area.x + 1;
        let content_y = area.y + 1;
        let content_width = area.width.saturating_sub(2);

        // 绘制提示
        use crossterm::style::PrintStyledContent;
        write!(
            self.stdout,
            "{}{}",
            cursor::MoveTo(content_x, content_y),
            PrintStyledContent(crossterm_style::StyledContent::new(style.to_crossterm_style(), prompt.to_string())),
        )
        .unwrap();

        // 绘制选择项
        for (i, item) in items.iter().enumerate() {
            let item_style = if i == selected { item.get_selected_style().clone() } else { item.get_style().clone() };

            use crossterm::style::PrintStyledContent;
            let item_text = format!("[{}] {}", if i == selected { "x" } else { " " }, item.text());
            let padded_text = self.pad_text(&item_text, content_width as usize);

            write!(
                self.stdout,
                "{}{}",
                cursor::MoveTo(content_x, content_y + 2 + i as u16),
                PrintStyledContent(crossterm_style::StyledContent::new(item_style.to_crossterm_style(), padded_text)),
            )
            .unwrap();
        }

        self.stdout.flush().unwrap();
    }

    /// 绘制边框
    fn draw_border(&mut self, area: Rect, style: Style) {
        use crossterm::style::PrintStyledContent;

        // 绘制上边框
        write!(
            self.stdout,
            "{}{}",
            cursor::MoveTo(area.x, area.y),
            PrintStyledContent(crossterm_style::StyledContent::new(style.to_crossterm_style(), "┌")),
        )
        .unwrap();
        write!(
            self.stdout,
            "{}{}",
            cursor::MoveTo(area.x + 1, area.y),
            PrintStyledContent(crossterm_style::StyledContent::new(
                style.to_crossterm_style(),
                "─".repeat((area.width - 2) as usize)
            )),
        )
        .unwrap();
        write!(
            self.stdout,
            "{}{}",
            cursor::MoveTo(area.x + area.width - 1, area.y),
            PrintStyledContent(crossterm_style::StyledContent::new(style.to_crossterm_style(), "┐")),
        )
        .unwrap();

        // 绘制左右边框
        for y in area.y + 1..area.y + area.height - 1 {
            write!(
                self.stdout,
                "{}{}",
                cursor::MoveTo(area.x, y),
                PrintStyledContent(crossterm_style::StyledContent::new(style.to_crossterm_style(), "│")),
            )
            .unwrap();
            write!(
                self.stdout,
                "{}{}",
                cursor::MoveTo(area.x + area.width - 1, y),
                PrintStyledContent(crossterm_style::StyledContent::new(style.to_crossterm_style(), "│")),
            )
            .unwrap();
        }

        // 绘制下边框
        write!(
            self.stdout,
            "{}{}",
            cursor::MoveTo(area.x, area.y + area.height - 1),
            PrintStyledContent(crossterm_style::StyledContent::new(style.to_crossterm_style(), "└")),
        )
        .unwrap();
        write!(
            self.stdout,
            "{}{}",
            cursor::MoveTo(area.x + 1, area.y + area.height - 1),
            PrintStyledContent(crossterm_style::StyledContent::new(
                style.to_crossterm_style(),
                "─".repeat((area.width - 2) as usize)
            )),
        )
        .unwrap();
        write!(
            self.stdout,
            "{}{}",
            cursor::MoveTo(area.x + area.width - 1, area.y + area.height - 1),
            PrintStyledContent(crossterm_style::StyledContent::new(style.to_crossterm_style(), "┘")),
        )
        .unwrap();

        self.stdout.flush().unwrap();
    }

    /// 填充文本
    fn pad_text(&self, text: &str, width: usize) -> String {
        if text.len() >= width { text.chars().take(width).collect() } else { format!("{:<width$}", text, width = width) }
    }

    /// 截断文本
    fn truncate_text(&self, text: &str, width: usize) -> String {
        text.chars().take(width).collect()
    }
}
