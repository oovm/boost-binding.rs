//! 样式系统模块
//!
//! 提供 GUI 样式管理功能，包括颜色、字体、布局样式等。

use serde::{Deserialize, Serialize};

/// 颜色结构体
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Color {
    /// 红色通道
    pub r: u8,
    /// 绿色通道
    pub g: u8,
    /// 蓝色通道
    pub b: u8,
    /// 透明度通道
    pub a: f32,
}

/// 字体结构体
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Font {
    /// 字体名称
    pub name: String,
    /// 字体大小
    pub size: f32,
    /// 是否粗体
    pub bold: bool,
    /// 是否斜体
    pub italic: bool,
    /// 是否下划线
    pub underline: bool,
}

/// 边距结构体
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Padding {
    /// 上内边距
    pub top: f32,
    /// 右内边距
    pub right: f32,
    /// 下内边距
    pub bottom: f32,
    /// 左内边距
    pub left: f32,
}

/// 边框结构体
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Border {
    /// 边框宽度
    pub width: f32,
    /// 边框颜色
    pub color: Color,
    /// 边框圆角
    pub radius: f32,
}

/// 样式结构体
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Style {
    /// 背景颜色
    pub background: Option<Color>,
    /// 前景颜色
    pub foreground: Option<Color>,
    /// 字体
    pub font: Option<Font>,
    /// 边距
    pub padding: Option<Padding>,
    /// 边框
    pub border: Option<Border>,
    /// 宽度
    pub width: Option<f32>,
    /// 高度
    pub height: Option<f32>,
    /// 水平对齐
    pub horizontal_align: Option<Alignment>,
    /// 垂直对齐
    pub vertical_align: Option<Alignment>,
}

/// 对齐方式
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// 左对齐/上对齐
    Start,
    /// 居中对齐
    Center,
    /// 右对齐/下对齐
    End,
}

impl Color {
    /// 创建新的颜色
    ///
    /// # 参数
    /// - `r`: 红色通道 (0-255)
    /// - `g`: 绿色通道 (0-255)
    /// - `b`: 蓝色通道 (0-255)
    /// - `a`: 透明度通道 (0.0-1.0)
    ///
    /// # 返回值
    /// 颜色实例
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a: a.clamp(0.0, 1.0) }
    }

    /// 创建不透明颜色
    ///
    /// # 参数
    /// - `r`: 红色通道 (0-255)
    /// - `g`: 绿色通道 (0-255)
    /// - `b`: 蓝色通道 (0-255)
    ///
    /// # 返回值
    /// 不透明颜色实例
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 1.0)
    }

    /// 创建透明颜色
    ///
    /// # 参数
    /// - `r`: 红色通道 (0-255)
    /// - `g`: 绿色通道 (0-255)
    /// - `b`: 蓝色通道 (0-255)
    /// - `a`: 透明度通道 (0.0-1.0)
    ///
    /// # 返回值
    /// 透明颜色实例
    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self::new(r, g, b, a)
    }

    /// 黑色
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 1.0 };

    /// 白色
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 1.0 };

    /// 红色
    pub const RED: Self = Self { r: 255, g: 0, b: 0, a: 1.0 };

    /// 绿色
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0, a: 1.0 };

    /// 蓝色
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255, a: 1.0 };

    /// 黄色
    pub const YELLOW: Self = Self { r: 255, g: 255, b: 0, a: 1.0 };

    /// 青色
    pub const CYAN: Self = Self { r: 0, g: 255, b: 255, a: 1.0 };

    /// 品红
    pub const MAGENTA: Self = Self { r: 255, g: 0, b: 255, a: 1.0 };

    /// 灰色
    pub const GRAY: Self = Self { r: 128, g: 128, b: 128, a: 1.0 };

    /// 浅灰色
    pub const LIGHT_GRAY: Self = Self { r: 200, g: 200, b: 200, a: 1.0 };

    /// 深灰色
    pub const DARK_GRAY: Self = Self { r: 64, g: 64, b: 64, a: 1.0 };
}

impl Font {
    /// 创建新的字体
    ///
    /// # 参数
    /// - `name`: 字体名称
    /// - `size`: 字体大小
    /// - `bold`: 是否粗体
    /// - `italic`: 是否斜体
    /// - `underline`: 是否下划线
    ///
    /// # 返回值
    /// 字体实例
    pub fn new(name: &str, size: f32, bold: bool, italic: bool, underline: bool) -> Self {
        Self { name: name.to_string(), size: size.max(1.0), bold, italic, underline }
    }

    /// 创建默认字体
    ///
    /// # 返回值
    /// 默认字体实例
    pub fn default() -> Self {
        Self::new("Arial", 14.0, false, false, false)
    }
}

impl Padding {
    /// 创建新的边距
    ///
    /// # 参数
    /// - `top`: 上内边距
    /// - `right`: 右内边距
    /// - `bottom`: 下内边距
    /// - `left`: 左内边距
    ///
    /// # 返回值
    /// 边距实例
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top: top.max(0.0), right: right.max(0.0), bottom: bottom.max(0.0), left: left.max(0.0) }
    }

    /// 创建对称边距
    ///
    /// # 参数
    /// - `vertical`: 垂直边距（上和下）
    /// - `horizontal`: 水平边距（左和右）
    ///
    /// # 返回值
    /// 对称边距实例
    pub fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self::new(vertical, horizontal, vertical, horizontal)
    }

    /// 创建统一边距
    ///
    /// # 参数
    /// - `all`: 所有方向的边距
    ///
    /// # 返回值
    /// 统一边距实例
    pub fn uniform(all: f32) -> Self {
        Self::new(all, all, all, all)
    }

    /// 默认边距
    pub const DEFAULT: Self = Self { top: 8.0, right: 8.0, bottom: 8.0, left: 8.0 };
}

impl Border {
    /// 创建新的边框
    ///
    /// # 参数
    /// - `width`: 边框宽度
    /// - `color`: 边框颜色
    /// - `radius`: 边框圆角
    ///
    /// # 返回值
    /// 边框实例
    pub fn new(width: f32, color: Color, radius: f32) -> Self {
        Self { width: width.max(0.0), color, radius: radius.max(0.0) }
    }

    /// 创建默认边框
    ///
    /// # 返回值
    /// 默认边框实例
    pub fn default() -> Self {
        Self::new(1.0, Color::GRAY, 0.0)
    }
}

impl Style {
    /// 创建新的样式
    ///
    /// # 返回值
    /// 样式实例
    pub fn new() -> Self {
        Self {
            background: None,
            foreground: None,
            font: None,
            padding: None,
            border: None,
            width: None,
            height: None,
            horizontal_align: None,
            vertical_align: None,
        }
    }

    /// 设置背景颜色
    ///
    /// # 参数
    /// - `color`: 背景颜色
    ///
    /// # 返回值
    /// 样式实例（链式调用）
    pub fn with_background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// 设置前景颜色
    ///
    /// # 参数
    /// - `color`: 前景颜色
    ///
    /// # 返回值
    /// 样式实例（链式调用）
    pub fn with_foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// 设置字体
    ///
    /// # 参数
    /// - `font`: 字体
    ///
    /// # 返回值
    /// 样式实例（链式调用）
    pub fn with_font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// 设置边距
    ///
    /// # 参数
    /// - `padding`: 边距
    ///
    /// # 返回值
    /// 样式实例（链式调用）
    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = Some(padding);
        self
    }

    /// 设置边框
    ///
    /// # 参数
    /// - `border`: 边框
    ///
    /// # 返回值
    /// 样式实例（链式调用）
    pub fn with_border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    /// 设置宽度
    ///
    /// # 参数
    /// - `width`: 宽度
    ///
    /// # 返回值
    /// 样式实例（链式调用）
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width.max(0.0));
        self
    }

    /// 设置高度
    ///
    /// # 参数
    /// - `height`: 高度
    ///
    /// # 返回值
    /// 样式实例（链式调用）
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height.max(0.0));
        self
    }

    /// 设置水平对齐
    ///
    /// # 参数
    /// - `align`: 对齐方式
    ///
    /// # 返回值
    /// 样式实例（链式调用）
    pub fn with_horizontal_align(mut self, align: Alignment) -> Self {
        self.horizontal_align = Some(align);
        self
    }

    /// 设置垂直对齐
    ///
    /// # 参数
    /// - `align`: 对齐方式
    ///
    /// # 返回值
    /// 样式实例（链式调用）
    pub fn with_vertical_align(mut self, align: Alignment) -> Self {
        self.vertical_align = Some(align);
        self
    }

    /// 默认样式
    pub fn default() -> Self {
        Self::new()
    }
}
