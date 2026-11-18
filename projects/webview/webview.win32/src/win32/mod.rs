//! Win32 + WebView2 后端，按职责拆分：
//!
//! - `state`：`DesktopState` 跨线程共享状态、IPC 队列、显窗与自动化结果回收；
//! - `window`：`WNDPROC` 窗口过程、拖放路径解析与窗口图标；
//! - `webview2`：Runtime 探测、环境与 controller 创建及事件桥接；
//! - `spa`：虚拟主机内容映射与 SPA 直达路由回退；
//! - `backend`：`Win32Backend` 生命周期与消息泵（`WebViewBackend` 实现入口）；
//! - `util`：宽字符串编码与 HRESULT 结构化失败等小工具。

mod backend;
mod spa;
mod state;
mod util;
mod webview2;
mod window;

pub use backend::Win32Backend;

use std::time::Duration;

pub(crate) type EventRegistrationToken = i64;

pub(crate) const VIRTUAL_HOST: &str = "app.local";
/// 回退时携带原始路由的查询参数名，由前置脚本在页面脚本运行前消费。
pub(crate) const SPA_ROUTE_PARAM: &str = "__webview_route";
/// 延迟显窗的兜底时限：超过该时长首屏导航仍未完成也先把窗口显示出来，
/// 避免「进程活着但看不见窗口」；宁可短暂白屏，不要隐形窗口。
pub(crate) const REVEAL_TIMEOUT: Duration = Duration::from_secs(5);
