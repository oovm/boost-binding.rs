# `binding-webview-win32`

自研 Win32 + WebView2 后端，实现 `binding-webview` 的 `WebViewBackend`（`Win32Backend`），
提供非阻塞的 `pump`。

由 WAE 门面经 feature `webview` 以 `wae::webview` 导出；Windows 上默认即该后端。
非 Windows 目标上本 crate 不提供能力，也不会被门面选中。
