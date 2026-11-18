//! 跨子模块共用的小工具：UTF-16 宽字符串编码与 HRESULT 结构化失败。

use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

use webview_types::WebViewError;

pub(crate) fn encode_wide(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(std::iter::once(0)).collect()
}

pub(crate) fn host_failure(code: &'static str, stage: &'static str, error: &windows::core::Error) -> WebViewError {
    WebViewError::host(code, stage, error.message()).with_hresult(error.code().0)
}
