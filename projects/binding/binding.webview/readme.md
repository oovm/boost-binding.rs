# `binding-webview`

WAE WebView 后端合同：平台无关的 `OpenOptions` / `IpcOutcome` / `IpcHandler` / `WebViewError`，
后端 trait `WebViewBackend`，以及泛型宿主 `WebViewHost<B>`。

平台实现：

- `binding-webview-win32`（Win32 + WebView2）
- `binding-webview-macos`（WKWebView）
- `binding-webview-linux`（WebKitGTK 4.1）

`wae` 门面经 feature `webview` 以 `wae::webview` 导出，并按编译目标选定后端实例。
本 crate 不探测运行时环境，也不回退到其它平台实现。
