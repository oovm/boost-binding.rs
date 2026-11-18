//! WebView 门面：按编译目标选定唯一后端，并导出平台无关契约。
//!
//! 下游只依赖本 crate：`webview::WebViewHost` 与 `webview::PlatformBackend`
//! 已经是当前目标的实现，不需要再写 `cfg(target_os)` 自行挑选后端。
//! 需要泛型宿主或自定义后端时，请直接依赖 `webview-types`。

#![deny(missing_docs)]

pub use webview_types::{
    AutomationError, AutomationId, Capability, CaptureOutcome, DragRect, HostFailure, IpcHandler, IpcOutcome, IpcSink, MESSAGE_HANDLER_NAME,
    OpenOptions, Platform, Result, ScriptOutcome, WebViewBackend, WebViewError, automation_codes, js_string_literal, page_receive_script,
};

#[cfg(target_os = "windows")]
pub use webview_win32::Win32Backend as PlatformBackend;

#[cfg(target_os = "macos")]
pub use webview_macos::MacosBackend as PlatformBackend;

#[cfg(target_os = "linux")]
pub use webview_linux::LinuxBackend as PlatformBackend;

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
compile_error!("`webview` 只支持 Windows / macOS / Linux；其它目标没有 WebView 后端");

/// 当前编译目标的 WebView 宿主，等价于 `webview_types::WebViewHost<PlatformBackend>`。
#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
pub type WebViewHost = webview_types::WebViewHost<PlatformBackend>;
