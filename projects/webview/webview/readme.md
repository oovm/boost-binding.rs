# `webview`

WebView 门面：导出 `webview-types` 的平台无关契约，并按编译目标自动选定唯一后端。

- Windows → `webview-win32`（Win32 + WebView2）
- macOS → `webview-macos`（WKWebView）
- Linux → `webview-linux`（WebKitGTK 4.1）
- 其它目标在编译期显式失败

下游只需依赖本 crate：

```rust
let host = webview::WebViewHost::open(options, Box::new(handler))?;
```

`webview::WebViewHost` 与 `webview::PlatformBackend` 已是当前目标的实现，
调用方不需要写 `cfg(target_os)` 自行挑选后端。需要泛型宿主或自定义后端时，
依赖 `webview-types` 即可。
