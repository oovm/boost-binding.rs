# boost-binding.rs

Boost 生态的**外部绑定层**：集中承载确有必要的外部库与宿主平台绑定能力。

## 边界

- 只收「OS 自带、跨平台各有一套 ABI、已有真实消费者」的绑定；每项外部依赖在 `Cargo.toml` 账本注释中说明必要性。
- 通用基础能力（fs、log、net、诊断等）归 `rust-boost`，不在本仓重复建设。
- 应用作者面与产品组装在上游消费者仓，本仓不做应用约定、不感知任何具体下游产品。
- 准入与候选账本见工作区决策文档索引（Boost 生态侧）。

## Crate 家族

```text
projects/binding/
  binding.com              # Windows COM 运行时底座（Apartment 守卫 / ComError / CoTaskMem 回收）
  binding.dbus             # Linux D-Bus 客户端底座（消息编解码跨平台可测，连接层仅 Linux）
projects/webview/
  webview                  # 门面：按编译目标选定后端并导出 WebViewHost / PlatformBackend
  webview.types            # 契约：平台无关类型、WebViewBackend trait 与泛型 WebViewHost<B>
  webview.win32            # Win32 + WebView2 后端（webview2-com + binding-com，非阻塞 pump）
  webview.macos            # WKWebView 后端（objc2 家族）
  webview.linux            # WebKitGTK 后端（libloading 运行时 dlopen）
```

`webview-types` 只放契约，`webview` 做门面并自动选后端，下游只需要依赖 `webview` 一个 crate，
不必自己写 `cfg(target_os)` 挑实现。本仓 crate 之间只有「后端 → 契约 / 底座」单向依赖。
WebView 各 crate 使用中性桥接方言（`window.__webviewReceive`、`__webview_route`、`WEBVIEW_*` 环境变量），
不感知任何具体下游产品。

## 开发

```bash
cargo fmt --all
cargo test --workspace
```

三平台 CI 矩阵覆盖 `ubuntu-latest` / `windows-latest` / `macos-latest`。

## 许可

Apache-2.0
