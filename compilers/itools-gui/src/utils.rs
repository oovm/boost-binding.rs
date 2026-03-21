//! 工具函数模块
//!
//! 提供 GUI 相关的工具函数和辅助方法。

/// 计算两点之间的距离
///
/// # 参数
/// - `x1`: 第一个点的 x 坐标
/// - `y1`: 第一个点的 y 坐标
/// - `x2`: 第二个点的 x 坐标
/// - `y2`: 第二个点的 y 坐标
///
/// # 返回值
/// 两点之间的距离
pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// 限制值在指定范围内
///
/// # 参数
/// - `value`: 要限制的值
/// - `min`: 最小值
/// - `max`: 最大值
///
/// # 返回值
/// 限制后的值
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

/// 线性插值
///
/// # 参数
/// - `start`: 起始值
/// - `end`: 结束值
/// - `t`: 插值参数 (0.0-1.0)
///
/// # 返回值
/// 插值结果
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * clamp(t, 0.0, 1.0)
}

/// 从 0-255 范围的 RGB 值转换为 0-1 范围的浮点数
///
/// # 参数
/// - `value`: 0-255 范围的 RGB 值
///
/// # 返回值
/// 0-1 范围的浮点数
pub fn rgb_to_float(value: u8) -> f32 {
    value as f32 / 255.0
}

/// 从 0-1 范围的浮点数转换为 0-255 范围的 RGB 值
///
/// # 参数
/// - `value`: 0-1 范围的浮点数
///
/// # 返回值
/// 0-255 范围的 RGB 值
pub fn float_to_rgb(value: f32) -> u8 {
    (clamp(value, 0.0, 1.0) * 255.0).round() as u8
}

/// 检查点是否在矩形内
///
/// # 参数
/// - `x`: 点的 x 坐标
/// - `y`: 点的 y 坐标
/// - `rect_x`: 矩形的 x 坐标
/// - `rect_y`: 矩形的 y 坐标
/// - `rect_width`: 矩形的宽度
/// - `rect_height`: 矩形的高度
///
/// # 返回值
/// 点是否在矩形内
pub fn point_in_rect(x: f32, y: f32, rect_x: f32, rect_y: f32, rect_width: f32, rect_height: f32) -> bool {
    x >= rect_x && x <= rect_x + rect_width && y >= rect_y && y <= rect_y + rect_height
}

/// 生成唯一 ID
///
/// # 返回值
/// 唯一 ID 字符串
pub fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("{:x}", timestamp)
}

/// 格式化错误信息
///
/// # 参数
/// - `error`: 错误
///
/// # 返回值
/// 格式化后的错误信息
pub fn format_error(error: &dyn std::error::Error) -> String {
    let mut message = error.to_string();
    let mut current_error = error.source();
    while let Some(cause) = current_error {
        message.push_str(&format!("\nCaused by: {}", cause));
        current_error = cause.source();
    }
    message
}
