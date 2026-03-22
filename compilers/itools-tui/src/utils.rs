//! 工具函数模块
//!
//! 提供各种 TUI 开发中常用的工具函数和实用工具。

use crate::style::Color;
use std::io::Write;

/// 文本处理工具
pub mod text {
    /// 截断文本到指定长度
    ///
    /// # Arguments
    /// * `text` - 要截断的文本
    /// * `max_length` - 最大长度
    ///
    /// # Returns
    /// 截断后的文本
    pub fn truncate(text: &str, max_length: usize) -> String {
        text.chars().take(max_length).collect()
    }

    /// 填充文本到指定长度
    ///
    /// # Arguments
    /// * `text` - 要填充的文本
    /// * `length` - 目标长度
    /// * `fill_char` - 填充字符
    ///
    /// # Returns
    /// 填充后的文本
    pub fn pad(text: &str, length: usize, fill_char: char) -> String {
        if text.len() >= length {
            text.chars().take(length).collect()
        }
        else {
            format!("{}{}", text, fill_char.to_string().repeat(length - text.len()))
        }
    }

    /// 居中文本
    ///
    /// # Arguments
    /// * `text` - 要居中的文本
    /// * `width` - 总宽度
    ///
    /// # Returns
    /// 居中后的文本
    pub fn center(text: &str, width: usize) -> String {
        if text.len() >= width {
            text.chars().take(width).collect()
        }
        else {
            let padding = (width - text.len()) / 2;
            format!("{}{}{}", " ".repeat(padding), text, " ".repeat(width - text.len() - padding))
        }
    }
}

/// 颜色转换工具
pub mod color {
    use super::Color;
    use crossterm::style as crossterm_style;

    /// 将 RGB 值转换为 Color 枚举
    ///
    /// # Arguments
    /// * `r` - 红色通道 (0-255)
    /// * `g` - 绿色通道 (0-255)
    /// * `b` - 蓝色通道 (0-255)
    ///
    /// # Returns
    /// Color 枚举值
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::Rgb(r, g, b)
    }

    /// 将 Color 转换为 crossterm 颜色
    ///
    /// # Arguments
    /// * `color` - Color 枚举值
    ///
    /// # Returns
    /// crossterm 颜色
    pub fn to_crossterm_color(color: Color) -> crossterm_style::Color {
        color.into()
    }
}

/// 终端操作工具
pub mod terminal_utils {
    use crossterm::{cursor, terminal as crossterm_terminal};
    use std::io::{self, Result as CrosstermResult, Write};

    /// 清除屏幕
    ///
    /// # Returns
    /// 操作结果
    #[allow(dead_code)]
    pub fn clear() -> CrosstermResult<()> {
        let mut stdout = io::stdout();
        write!(stdout, "{}", crossterm_terminal::Clear(crossterm_terminal::ClearType::All))?;
        stdout.flush()?;
        Ok(())
    }

    /// 移动光标
    ///
    /// # Arguments
    /// * `x` - x 坐标
    /// * `y` - y 坐标
    ///
    /// # Returns
    /// 操作结果
    #[allow(dead_code)]
    pub fn move_cursor(x: u16, y: u16) -> CrosstermResult<()> {
        let mut stdout = io::stdout();
        write!(stdout, "{}", cursor::MoveTo(x, y))?;
        stdout.flush()?;
        Ok(())
    }

    /// 隐藏光标
    ///
    /// # Returns
    /// 操作结果
    #[allow(dead_code)]
    pub fn hide_cursor() -> CrosstermResult<()> {
        let mut stdout = io::stdout();
        write!(stdout, "{}", cursor::Hide)?;
        stdout.flush()?;
        Ok(())
    }

    /// 显示光标
    ///
    /// # Returns
    /// 操作结果
    #[allow(dead_code)]
    pub fn show_cursor() -> CrosstermResult<()> {
        let mut stdout = io::stdout();
        write!(stdout, "{}", cursor::Show)?;
        stdout.flush()?;
        Ok(())
    }
}

/// 数学工具
pub mod math {
    /// 限制值在指定范围内
    ///
    /// # Arguments
    /// * `value` - 要限制的值
    /// * `min` - 最小值
    /// * `max` - 最大值
    ///
    /// # Returns
    /// 限制后的值
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        }
        else if value > max {
            max
        }
        else {
            value
        }
    }

    /// 线性插值
    ///
    /// # Arguments
    /// * `start` - 起始值
    /// * `end` - 结束值
    /// * `t` - 插值参数 (0.0-1.0)
    ///
    /// # Returns
    /// 插值结果
    pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
        start + (end - start) * t
    }
}

/// 交互式选择菜单
pub fn select<T>(items: &[T], prompt: &str) -> Option<usize>
where
    T: std::fmt::Display,
{
    println!("{}", prompt);
    for (i, item) in items.iter().enumerate() {
        println!("{}: {}", i + 1, item);
    }

    loop {
        print!("请选择 (1-{}): ", items.len());
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(choice) if choice >= 1 && choice <= items.len() => {
                return Some(choice - 1);
            }
            _ => {
                println!("无效的选择，请重新输入");
            }
        }
    }
}

/// 显示加载动画
pub fn loading_animation(duration: std::time::Duration, message: &str) {
    let chars = ["|", "/", "-", "\\"];
    let start = std::time::Instant::now();

    print!("{}", message);
    std::io::stdout().flush().unwrap();

    let mut i = 0;
    while start.elapsed() < duration {
        print!("\r{}{}", message, chars[i % chars.len()]);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        i += 1;
    }

    print!("\r{}{}", message, " ");
    println!();
}
