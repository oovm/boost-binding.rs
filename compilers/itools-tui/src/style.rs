//! 样式系统模块
//!
//! 提供样式相关功能，包括颜色、字体样式和边框样式。

use crossterm::style as crossterm_style;

/// 颜色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// 黑色
    Black,
    /// 红色
    Red,
    /// 绿色
    Green,
    /// 黄色
    Yellow,
    /// 蓝色
    Blue,
    /// 紫色
    Magenta,
    /// 青色
    Cyan,
    /// 白色
    White,
    /// 亮黑色
    BrightBlack,
    /// 亮红色
    BrightRed,
    /// 亮绿色
    BrightGreen,
    /// 亮黄色
    BrightYellow,
    /// 亮蓝色
    BrightBlue,
    /// 亮紫色
    BrightMagenta,
    /// 亮青色
    BrightCyan,
    /// 亮白色
    BrightWhite,
    /// RGB 颜色
    Rgb(u8, u8, u8),
}

impl From<Color> for crossterm_style::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => crossterm_style::Color::Black,
            Color::Red => crossterm_style::Color::Red,
            Color::Green => crossterm_style::Color::Green,
            Color::Yellow => crossterm_style::Color::Yellow,
            Color::Blue => crossterm_style::Color::Blue,
            Color::Magenta => crossterm_style::Color::Magenta,
            Color::Cyan => crossterm_style::Color::Cyan,
            Color::White => crossterm_style::Color::White,
            Color::BrightBlack => crossterm_style::Color::AnsiValue(8),
            Color::BrightRed => crossterm_style::Color::AnsiValue(9),
            Color::BrightGreen => crossterm_style::Color::AnsiValue(10),
            Color::BrightYellow => crossterm_style::Color::AnsiValue(11),
            Color::BrightBlue => crossterm_style::Color::AnsiValue(12),
            Color::BrightMagenta => crossterm_style::Color::AnsiValue(13),
            Color::BrightCyan => crossterm_style::Color::AnsiValue(14),
            Color::BrightWhite => crossterm_style::Color::AnsiValue(15),
            Color::Rgb(r, g, b) => crossterm_style::Color::Rgb { r, g, b },
        }
    }
}

/// 字体样式修饰符
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifier {
    bits: u8,
}

impl Modifier {
    /// 空修饰符
    pub const NONE: Self = Self { bits: 0 };
    /// 粗体
    pub const BOLD: Self = Self { bits: 1 };
    /// 斜体
    pub const ITALIC: Self = Self { bits: 2 };
    /// 下划线
    pub const UNDERLINED: Self = Self { bits: 4 };
    /// 反显
    pub const REVERSED: Self = Self { bits: 8 };

    /// 检查是否包含指定修饰符
    pub fn contains(&self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    /// 添加修饰符
    pub fn insert(&mut self, other: Self) {
        self.bits |= other.bits;
    }

    /// 移除修饰符
    pub fn remove(&mut self, other: Self) {
        self.bits &= !other.bits;
    }
}

impl std::ops::BitOr for Modifier {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self { bits: self.bits | rhs.bits }
    }
}

impl std::ops::BitOrAssign for Modifier {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

/// 样式
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Style {
    /// 前景色
    pub fg: Option<Color>,
    /// 背景色
    pub bg: Option<Color>,
    /// 修饰符
    pub modifiers: Modifier,
}

impl Default for Style {
    fn default() -> Self {
        Self { fg: None, bg: None, modifiers: Modifier::NONE }
    }
}

impl Style {
    /// 创建默认样式
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置前景色
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// 设置背景色
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// 设置粗体
    pub fn bold(mut self) -> Self {
        self.modifiers.insert(Modifier::BOLD);
        self
    }

    /// 设置斜体
    pub fn italic(mut self) -> Self {
        self.modifiers.insert(Modifier::ITALIC);
        self
    }

    /// 设置下划线
    pub fn underlined(mut self) -> Self {
        self.modifiers.insert(Modifier::UNDERLINED);
        self
    }

    /// 设置反显
    pub fn reversed(mut self) -> Self {
        self.modifiers.insert(Modifier::REVERSED);
        self
    }

    /// 组合样式
    pub fn combine(&mut self, other: &Style) {
        if other.fg.is_some() {
            self.fg = other.fg;
        }
        if other.bg.is_some() {
            self.bg = other.bg;
        }
        self.modifiers |= other.modifiers;
    }

    /// 转换为 crossterm 样式
    pub fn to_crossterm_style(&self) -> crossterm_style::ContentStyle {
        let mut content_style = crossterm_style::ContentStyle::default();

        // 直接设置前景色和背景色
        if let Some(fg) = self.fg {
            content_style.foreground_color = Some(crossterm_style::Color::from(fg));
        }

        if let Some(bg) = self.bg {
            content_style.background_color = Some(crossterm_style::Color::from(bg));
        }

        // 设置修饰符
        if self.modifiers.contains(Modifier::BOLD) {
            content_style.attributes.set(crossterm_style::Attribute::Bold);
        }

        if self.modifiers.contains(Modifier::ITALIC) {
            content_style.attributes.set(crossterm_style::Attribute::Italic);
        }

        if self.modifiers.contains(Modifier::UNDERLINED) {
            content_style.attributes.set(crossterm_style::Attribute::Underlined);
        }

        if self.modifiers.contains(Modifier::REVERSED) {
            content_style.attributes.set(crossterm_style::Attribute::Reverse);
        }

        content_style
    }
}
