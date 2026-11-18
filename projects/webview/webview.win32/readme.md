# `webview-win32`

自研 Win32 + WebView2 后端，实现 `webview-types` 的 `WebViewBackend`（`Win32Backend`），
提供非阻塞的 `pump`。

由 `webview` 门面按编译目标选定导出；Windows 上默认即该后端。
非 Windows 目标上本 crate 不提供能力，也不会被 `webview` 门面选中。
