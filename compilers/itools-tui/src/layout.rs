//! 布局系统模块
//!
//! 提供布局相关功能，包括垂直布局、水平布局和网格布局。

/// 布局约束
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constraint {
    /// 固定大小
    Length(u16),
    /// 剩余空间的比例
    Ratio(u32, u32),
    /// 尽可能小
    Min(u16),
    /// 尽可能大
    Max(u16),
    /// 填充剩余空间
    Fill,
}

/// 布局方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// 垂直方向
    Vertical,
    /// 水平方向
    Horizontal,
}

/// 布局
pub struct Layout {
    direction: Direction,
    constraints: Vec<Constraint>,
}

impl Layout {
    /// 创建垂直布局
    pub fn vertical(constraints: Vec<Constraint>) -> Self {
        Self { direction: Direction::Vertical, constraints }
    }

    /// 创建水平布局
    pub fn horizontal(constraints: Vec<Constraint>) -> Self {
        Self { direction: Direction::Horizontal, constraints }
    }

    /// 分割区域
    pub fn split(&self, area: Rect) -> Vec<Rect> {
        match self.direction {
            Direction::Vertical => self.split_vertical(area),
            Direction::Horizontal => self.split_horizontal(area),
        }
    }

    /// 垂直分割
    fn split_vertical(&self, area: Rect) -> Vec<Rect> {
        let mut result = Vec::new();
        let mut remaining_height = area.height;
        let mut y = area.y;

        // 计算固定长度和最小长度
        let mut fixed_length = 0;
        let mut min_length = 0;

        for constraint in &self.constraints {
            match constraint {
                Constraint::Length(len) => fixed_length += len,
                Constraint::Min(len) => min_length += len,
                _ => {}
            }
        }

        // 计算可分配的空间
        let available_space = remaining_height.saturating_sub(fixed_length);

        // 计算比例总和
        let mut ratio_sum = 0;
        for constraint in &self.constraints {
            if let Constraint::Ratio(numerator, denominator) = constraint {
                ratio_sum += numerator * 1000 / denominator;
            }
        }

        // 分割空间
        for constraint in &self.constraints {
            let height = match constraint {
                Constraint::Length(len) => *len,
                Constraint::Min(len) => *len,
                Constraint::Max(len) => std::cmp::min(*len, remaining_height),
                Constraint::Ratio(numerator, denominator) => {
                    if ratio_sum > 0 {
                        (available_space as u32 * numerator * 1000 / denominator / ratio_sum) as u16
                    }
                    else {
                        0
                    }
                }
                Constraint::Fill => remaining_height,
            };

            if height > 0 {
                result.push(Rect { x: area.x, y, width: area.width, height });
                y += height;
                remaining_height -= height;
            }
        }

        result
    }

    /// 水平分割
    fn split_horizontal(&self, area: Rect) -> Vec<Rect> {
        let mut result = Vec::new();
        let mut remaining_width = area.width;
        let mut x = area.x;

        // 计算固定长度和最小长度
        let mut fixed_length = 0;
        let mut min_length = 0;

        for constraint in &self.constraints {
            match constraint {
                Constraint::Length(len) => fixed_length += len,
                Constraint::Min(len) => min_length += len,
                _ => {}
            }
        }

        // 计算可分配的空间
        let available_space = remaining_width.saturating_sub(fixed_length);

        // 计算比例总和
        let mut ratio_sum = 0;
        for constraint in &self.constraints {
            if let Constraint::Ratio(numerator, denominator) = constraint {
                ratio_sum += numerator * 1000 / denominator;
            }
        }

        // 分割空间
        for constraint in &self.constraints {
            let width = match constraint {
                Constraint::Length(len) => *len,
                Constraint::Min(len) => *len,
                Constraint::Max(len) => std::cmp::min(*len, remaining_width),
                Constraint::Ratio(numerator, denominator) => {
                    if ratio_sum > 0 {
                        (available_space as u32 * numerator * 1000 / denominator / ratio_sum) as u16
                    }
                    else {
                        0
                    }
                }
                Constraint::Fill => remaining_width,
            };

            if width > 0 {
                result.push(Rect { x, y: area.y, width, height: area.height });
                x += width;
                remaining_width -= width;
            }
        }

        result
    }
}

/// 矩形区域
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    /// x 坐标
    pub x: u16,
    /// y 坐标
    pub y: u16,
    /// 宽度
    pub width: u16,
    /// 高度
    pub height: u16,
}

impl Rect {
    /// 创建新的矩形
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    /// 从大小创建矩形
    pub fn from_size(width: u16, height: u16) -> Self {
        Self { x: 0, y: 0, width, height }
    }

    /// 检查点是否在矩形内
    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    /// 缩小矩形
    pub fn shrink(&self, margin: u16) -> Self {
        Self {
            x: self.x + margin,
            y: self.y + margin,
            width: self.width.saturating_sub(2 * margin),
            height: self.height.saturating_sub(2 * margin),
        }
    }
}
