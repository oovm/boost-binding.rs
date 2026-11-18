# boost-binding.rs

Boost 生态的**外部绑定层**：集中承载确有必要的外部库与宿主平台绑定能力。

## 边界

- 只收「OS 自带、跨平台各有一套 ABI、已有真实消费者」的绑定；每项外部依赖在 `Cargo.toml` 账本注释中说明必要性。
- 通用基础能力（fs、log、net、诊断等）归 `rust-boost`，不在本仓重复建设。
- 应用作者面与产品组装（WAE 等）在上游消费者仓，本仓不做应用约定。
- 准入与候选账本见工作区决策文档索引（Boost 生态侧）。

## Crate 家族

```text
projects/binding/
  binding.webview          # 平台无关 WebView 宿主合同（OpenOptions / WebViewBackend / WebViewHost / IPC）
  binding.webview.win32    # Win32 + WebView2 后端（webview2-com，非阻塞 pump）
  binding.webview.macos    # WKWebView 后端（objc2 家族）
  binding.webview.linux    # WebKitGTK 后端（libloading 运行时 dlopen）
```

平台无关合同与后端选择由消费者门面完成；本仓 crate 之间只有「后端 → 合同」单向依赖。

## 开发

```bash
cargo fmt --all
cargo test --workspace
```

三平台 CI 矩阵覆盖 `ubuntu-latest` / `windows-latest` / `macos-latest`。

## 许可

Apache-2.0
