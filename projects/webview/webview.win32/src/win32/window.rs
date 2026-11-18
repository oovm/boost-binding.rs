//! `WNDPROC` 窗口过程：激活焦点转移、尺寸 / DPI 变化、拖放、frameless 命中测试
//! 与销毁收尾；另含拖放路径解析与窗口图标加载。

use std::{ffi::c_void, path::Path};

use webview2_com::Microsoft::Web::WebView2::Win32::COREWEBVIEW2_MOVE_FOCUS_REASON_PROGRAMMATIC;
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM},
        Graphics::Gdi::ScreenToClient,
        UI::{
            Input::KeyboardAndMouse::VK_F12,
            Shell::{DragFinish, DragQueryFileW, HDROP},
            WindowsAndMessaging::{
                DefWindowProcW, DestroyWindow, GWLP_USERDATA, GetWindowLongPtrW, HTCAPTION, HTCLIENT, ICON_BIG, ICON_SMALL, IMAGE_ICON,
                LR_LOADFROMFILE, LoadImageW, PostQuitMessage, SWP_NOACTIVATE, SWP_NOZORDER, SendMessageW, SetWindowLongPtrW, SetWindowPos,
                WM_ACTIVATE, WM_CLOSE, WM_CREATE, WM_DESTROY, WM_DPICHANGED, WM_DROPFILES, WM_KEYDOWN, WM_NCHITTEST, WM_SETICON, WM_SIZE,
            },
        },
    },
    core::PCWSTR,
};

use super::state::DesktopState;
use super::util::encode_wide;
use crate::dpi;

/// 从 `WM_DROPFILES` 的 `HDROP` 句柄取出全部完整本地路径。
fn dropped_paths(hdrop: HDROP) -> Vec<String> {
    let mut paths = Vec::new();
    unsafe {
        let count = DragQueryFileW(hdrop, u32::MAX, None);
        paths.reserve(count as usize);
        for index in 0..count {
            let length = DragQueryFileW(hdrop, index, None);
            if length == 0 {
                continue;
            }
            let mut buffer = vec![0u16; length as usize + 1];
            let written = DragQueryFileW(hdrop, index, Some(buffer.as_mut_slice()));
            if written == 0 {
                continue;
            }
            buffer.truncate(written as usize);
            paths.push(String::from_utf16_lossy(&buffer));
        }
    }
    paths
}

/// 从内容目录读取 `app.ico` 并设为窗口图标（标题栏与任务栏）。文件缺失时静默跳过，
/// 保持无图标而非报错。宿主此前从不 `WM_SETICON`，窗口左上角因此一直是空白。
pub(crate) fn apply_window_icon(hwnd: HWND, content_dir: Option<&str>) {
    let Some(dir) = content_dir else {
        return;
    };
    let candidate = Path::new(dir).join("app.ico");
    if !candidate.is_file() {
        return;
    }
    let path_w = encode_wide(&candidate.display().to_string());
    unsafe {
        let name = PCWSTR(path_w.as_ptr());
        // 加载失败不再静默：白屏无图标时这条是唯一线索。
        match LoadImageW(None, name, IMAGE_ICON, 16, 16, LR_LOADFROMFILE) {
            Ok(icon) => {
                let _ = SendMessageW(hwnd, WM_SETICON, Some(WPARAM(ICON_SMALL as usize)), Some(LPARAM(icon.0 as isize)));
            }
            Err(error) => eprintln!("webview: app.ico small icon load failed: {error}"),
        }
        match LoadImageW(None, name, IMAGE_ICON, 32, 32, LR_LOADFROMFILE) {
            Ok(icon) => {
                let _ = SendMessageW(hwnd, WM_SETICON, Some(WPARAM(ICON_BIG as usize)), Some(LPARAM(icon.0 as isize)));
            }
            Err(error) => eprintln!("webview: app.ico big icon load failed: {error}"),
        }
    }
}

pub(crate) unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if msg == WM_CREATE {
            let create = lparam.0 as *const windows::Win32::UI::WindowsAndMessaging::CREATESTRUCTW;
            let state = (*create).lpCreateParams as *mut DesktopState;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, state as isize);
            (*state).hwnd = hwnd;
            return LRESULT(0);
        }

        let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut DesktopState;
        if state_ptr.is_null() {
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
        let state = &mut *state_ptr;

        match msg {
            // `WM_MOUSEWHEEL` 与键盘输入只发给「拥有焦点的窗口」。WebView2 不会在宿主
            // 窗口激活时自动把焦点移进网页内容：不处理激活的话，焦点留在顶层 frame
            // （`GetGUIThreadInfo` 实测 `hwndFocus` 为 frame 或 null），滚轮消息被
            // `DefWindowProcW` 丢弃，页面无法滚动。先让 `DefWindowProcW` 完成默认激活
            // （否则焦点会被它抢回 frame），再把焦点移进内容；`PROGRAMMATIC` 会恢复
            // 页面内上次聚焦的元素，不会重置光标位置。
            WM_ACTIVATE => {
                let result = DefWindowProcW(hwnd, msg, wparam, lparam);
                if (wparam.0 & 0xFFFF) != 0
                    && let Some(controller) = state.controller.as_ref()
                    && let Err(error) = controller.MoveFocus(COREWEBVIEW2_MOVE_FOCUS_REASON_PROGRAMMATIC)
                {
                    dpi::diag(format!("move_focus.failed hresult={:#x}", error.code().0));
                }
                result
            }
            WM_SIZE => {
                state.resize_webview();
                LRESULT(0)
            }
            WM_DPICHANGED => {
                // 跨显示器（或系统缩放变化）后使用系统建议矩形，客户区随之变为新
                // DPI 下的物理像素，WM_SIZE 会重新计算 WebView2 边界。
                let suggested = &*(lparam.0 as *const RECT);
                let _ = SetWindowPos(
                    hwnd,
                    None,
                    suggested.left,
                    suggested.top,
                    suggested.right - suggested.left,
                    suggested.bottom - suggested.top,
                    SWP_NOZORDER | SWP_NOACTIVATE,
                );
                state.log_geometry("wm_dpichanged");
                LRESULT(0)
            }
            WM_DROPFILES => {
                let hdrop = HDROP(lparam.0 as *mut c_void);
                let paths = dropped_paths(hdrop);
                DragFinish(hdrop);
                if !paths.is_empty() {
                    let json = serde_json::json!({ "type": "fileDrop", "paths": paths }).to_string();
                    if let Err(error) = state.post_message(&json) {
                        dpi::diag(format!("fileDrop.post_failed {error}"));
                    }
                }
                LRESULT(0)
            }
            // 启动后键盘焦点停在顶层窗口（WebView2 子窗口未取得焦点），
            // `AcceleratorKeyPressed` 收不到按键。这条分支覆盖该实测路径，
            // 与 accelerator 分支互补，任一焦点位置按 F12 都能开 DevTools。
            WM_KEYDOWN if wparam.0 == usize::from(VK_F12.0) => {
                state.open_dev_tools("f12");
                LRESULT(0)
            }
            WM_NCHITTEST => {
                let result = DefWindowProcW(hwnd, msg, wparam, lparam);
                // 只把默认「客户区」命中当作拖拽候选；缩放边框等非客户区结果原样保留。
                if result.0 != HTCLIENT as isize || state.options.decorations {
                    return result;
                }
                let regions = state.drag_regions.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                if regions.is_empty() {
                    return result;
                }
                // LPARAM 低 16 位 / 次 16 位是屏幕坐标（有符号），先转客户区再比对矩形。
                let x = (lparam.0 & 0x0000_FFFF) as u16 as i16 as i32;
                let y = ((lparam.0 >> 16) & 0x0000_FFFF) as u16 as i16 as i32;
                let mut point = POINT { x, y };
                let _ = ScreenToClient(hwnd, &mut point);
                for (rx, ry, rw, rh) in regions.iter() {
                    if point.x >= *rx && point.x < *rx + *rw && point.y >= *ry && point.y < *ry + *rh {
                        return LRESULT(HTCAPTION as isize);
                    }
                }
                result
            }
            WM_CLOSE => {
                let _ = DestroyWindow(hwnd);
                LRESULT(0)
            }
            WM_DESTROY => {
                state.alive = false;
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
