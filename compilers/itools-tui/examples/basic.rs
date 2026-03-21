//! 基本示例
//!
//! 展示 itools-tui 的基本使用方法，包括创建终端、渲染组件等。

use itools_tui::{
    components::{Button, Input, List, ListItem, SelectItem, SelectMenu},
    event::{Event, EventSource},
    layout::{Constraint, Layout, Rect},
    render::Frame,
    style::{Color, Modifier, Style},
    terminal::Terminal,
};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建终端
    let mut terminal = Terminal::new();
    terminal.enter_raw_mode()?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // 创建事件源
    let event_source = EventSource::new(Duration::from_millis(100));

    // 创建组件
    let mut button = Button::new("Click Me")
        .style(Style::new().fg(Color::Blue).bold())
        .active_style(Style::new().fg(Color::White).bg(Color::Blue).bold());

    let mut input = Input::new()
        .placeholder("Enter text here...")
        .style(Style::new().fg(Color::White))
        .cursor_style(Style::new().reversed())
        .focused(true);

    let items = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
        ListItem::new("Item 4"),
        ListItem::new("Item 5"),
    ];

    let mut list = List::new(items).style(Style::new().fg(Color::White)).selected_style(Style::new().fg(Color::Blue).bold());

    let select_items = vec![SelectItem::new("Option 1"), SelectItem::new("Option 2"), SelectItem::new("Option 3")];

    let mut select_menu = SelectMenu::new("Choose an option:", select_items)
        .style(Style::new().fg(Color::White))
        .selected_style(Style::new().fg(Color::Blue).bold());

    // 主循环
    loop {
        // 获取终端大小
        let size = terminal.size()?;
        let rect = Rect::from_size(size.width, size.height);

        // 创建布局
        let layout = Layout::vertical(vec![
            Constraint::Length(5),  // 按钮
            Constraint::Length(5),  // 输入框
            Constraint::Length(10), // 列表
            Constraint::Length(10), // 选择菜单
            Constraint::Fill,       // 剩余空间
        ]);

        let areas = layout.split(rect);

        // 创建帧
        let mut frame = Frame::new();

        // 渲染组件
        button.render(&mut frame, areas[0]);
        input.render(&mut frame, areas[1]);
        list.render(&mut frame, areas[2]);
        select_menu.render(&mut frame, areas[3]);

        // 处理事件
        match event_source.next()? {
            Event::Key(key_event) => {
                if key_event.code == crossterm::event::KeyCode::Esc {
                    break;
                }

                button.handle_event(&Event::Key(key_event));
                input.handle_event(&Event::Key(key_event));
                list.handle_event(&Event::Key(key_event));
                select_menu.handle_event(&Event::Key(key_event));
            }
            Event::Resize(width, height) => {
                // 处理窗口大小变化
                terminal.clear()?;
            }
            _ => {}
        }
    }

    // 清理
    terminal.show_cursor()?;
    terminal.exit_raw_mode()?;

    Ok(())
}
