# `webview-types`

WebView 宿主契约：平台无关的 `OpenOptions` / `IpcOutcome` / `IpcHandler` / `WebViewError`，
后端 trait `WebViewBackend`，以及泛型宿主 `WebViewHost<B>`。

平台实现：

- `webview-win32`（Win32 + WebView2）
- `webview-macos`（WKWebView）
- `webview-linux`（WebKitGTK 4.1）

`webview` 门面已按编译目标选定后端并导出 `WebViewHost`，下游通常不需要直接依赖本 crate。
需要泛型宿主或自定义后端时才用它。本 crate 不探测运行时环境，也不回退到其它平台实现。
