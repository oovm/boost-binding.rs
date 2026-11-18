//! DPI 换算与感知状态诊断的集成验收。非 Windows 宿主上整个文件不编译。

#![cfg(windows)]

use webview_win32::dpi::{Awareness, dip_to_physical, physical_to_dip};
use windows::Win32::UI::HiDpi::DPI_AWARENESS;

#[test]
fn dip_scales_match_windows_display_scales() {
    // 1100 × 720 DIP 在四档常见缩放下的物理尺寸。
    assert_eq!(dip_to_physical(1100, 96), 1100);
    assert_eq!(dip_to_physical(1100, 120), 1375);
    assert_eq!(dip_to_physical(1100, 144), 1650);
    assert_eq!(dip_to_physical(1100, 168), 1925);
    assert_eq!(dip_to_physical(1100, 175), 2005);
    assert_eq!(dip_to_physical(720, 96), 720);
    assert_eq!(dip_to_physical(720, 120), 900);
    assert_eq!(dip_to_physical(720, 144), 1080);
    assert_eq!(dip_to_physical(720, 168), 1260);
    assert_eq!(dip_to_physical(720, 175), 1313);
}

#[test]
fn physical_to_dip_inverts_scaling() {
    assert!((physical_to_dip(1650, 144) - 1100.0).abs() < 0.001);
    assert!((physical_to_dip(1800, 144) - 1200.0).abs() < 0.001);
}

#[test]
fn unaware_report_is_bitmap_scaled() {
    let unaware = Awareness { requested: false, observed: DPI_AWARENESS(0) };
    assert_eq!(unaware.name(), "unaware");
    assert!(unaware.is_bitmap_scaled());

    let v2 = Awareness { requested: true, observed: DPI_AWARENESS(2) };
    assert_eq!(v2.name(), "per-monitor-v2");
    assert!(!v2.is_bitmap_scaled());
}
