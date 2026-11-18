//! Linux 后端集成验收：`file://` URL 编码、运行库缺失的结构化失败与
//! GTK3 / WebKitGTK 4.1 符号解析。非 Linux 宿主上整个文件不编译。

#![cfg(target_os = "linux")]

use std::path::Path;

use webview_linux::{Libs, file_url, open_first};
use webview_types::WebViewError;

#[test]
fn file_url_encodes_reserved_bytes() {
    assert_eq!(file_url(Path::new("/tmp/my app/index.html")), "file:///tmp/my%20app/index.html");
    assert_eq!(file_url(Path::new("/tmp/app/#1.html")), "file:///tmp/app/%231.html");
}

#[test]
fn missing_runtime_libraries_report_a_dependency_failure() {
    let error = match open_first(&["libprobe-missing-runtime.so"]) {
        Ok(_) => panic!("a missing library must fail"),
        Err(error) => error,
    };
    assert_eq!(error.code(), Some("webkit_dependency_missing"));
    assert_eq!(error.stage(), Some("load_libraries"));
    match &error {
        WebViewError::Host(failure) => {
            assert!(failure.attempted.iter().any(|entry| entry.contains("libprobe-missing-runtime.so")));
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn gtk_and_webkit_symbols_resolve() {
    if let Err(error) = Libs::load() {
        panic!("所需 GTK3 / WebKitGTK 4.1 运行库不可用: {error}");
    }
}
