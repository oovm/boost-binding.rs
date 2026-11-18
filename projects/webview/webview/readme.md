# `webview`

WebView 宿主合同：平台无关的 `OpenOptions` / `IpcOutcome` / `IpcHandler` / `WebViewError`，
后端 trait `WebViewBackend`，以及泛型宿主 `WebViewHost<B>`。

平台实现：

- `webview-win32`（Win32 + WebView2）
- `webview-macos`（WKWebView）
- `webview-linux`（WebKitGTK 4.1）

消费者门面（应用框架）按 feature 与编译目标选定后端实例后导出。
本 crate 不探测运行时环境，也不回退到其它平台实现。
