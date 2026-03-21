//! 终端操作模块
//!
//! 提供终端相关的操作，包括终端大小获取、终端状态管理等。

use crossterm::{cursor, terminal as crossterm_terminal};
use std::io::{self, Result as CrosstermResult, Write};

/// 终端大小
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    /// 宽度
    pub width: u16,
    /// 高度
    pub height: u16,
}

/// 终端操作
pub struct Terminal {
    stdout: io::Stdout,
}

impl Terminal {
    /// 创建新的终端实例
    pub fn new() -> Self {
        Self { stdout: io::stdout() }
    }

    /// 获取终端大小
    pub fn size(&self) -> CrosstermResult<Size> {
        let (width, height) = crossterm_terminal::size()?;
        Ok(Size { width, height })
    }

    /// 进入原始模式
    pub fn enter_raw_mode(&self) -> CrosstermResult<()> {
        crossterm_terminal::enable_raw_mode()
    }

    /// 退出原始模式
    pub fn exit_raw_mode(&self) -> CrosstermResult<()> {
        crossterm_terminal::disable_raw_mode()
    }

    /// 清除屏幕
    pub fn clear(&mut self) -> CrosstermResult<()> {
        write!(self.stdout, "{}", crossterm_terminal::Clear(crossterm_terminal::ClearType::All))?;
        self.stdout.flush()?;
        Ok(())
    }

    /// 移动光标
    pub fn move_cursor(&mut self, x: u16, y: u16) -> CrosstermResult<()> {
        write!(self.stdout, "{}", cursor::MoveTo(x, y))?;
        self.stdout.flush()?;
        Ok(())
    }

    /// 隐藏光标
    pub fn hide_cursor(&mut self) -> CrosstermResult<()> {
        write!(self.stdout, "{}", cursor::Hide)?;
        self.stdout.flush()?;
        Ok(())
    }

    /// 显示光标
    pub fn show_cursor(&mut self) -> CrosstermResult<()> {
        write!(self.stdout, "{}", cursor::Show)?;
        self.stdout.flush()?;
        Ok(())
    }
}
