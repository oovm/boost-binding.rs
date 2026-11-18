//! `DesktopState` 跨线程共享状态：窗口与 WebView2 句柄、IPC 入站 / 出站队列、
//! 延迟显窗、几何诊断、导航失败记录与自动化（脚本 / 截图）结果回收。

use std::{
    path::Path,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Instant,
};

use webview_types::{AutomationError, AutomationId, CaptureOutcome, OpenOptions, Result, ScriptOutcome, WebViewError, automation_codes};
use webview2_com::{
    CapturePreviewCompletedHandler, ExecuteScriptCompletedHandler,
    Microsoft::Web::WebView2::Win32::{
        COREWEBVIEW2_CAPTURE_PREVIEW_IMAGE_FORMAT_PNG, COREWEBVIEW2_WEB_ERROR_STATUS, COREWEBVIEW2_WEB_ERROR_STATUS_CANNOT_CONNECT,
        COREWEBVIEW2_WEB_ERROR_STATUS_CERTIFICATE_COMMON_NAME_IS_INCORRECT, COREWEBVIEW2_WEB_ERROR_STATUS_CERTIFICATE_EXPIRED,
        COREWEBVIEW2_WEB_ERROR_STATUS_CERTIFICATE_IS_INVALID, COREWEBVIEW2_WEB_ERROR_STATUS_CERTIFICATE_REVOKED,
        COREWEBVIEW2_WEB_ERROR_STATUS_CLIENT_CERTIFICATE_CONTAINS_ERRORS, COREWEBVIEW2_WEB_ERROR_STATUS_CONNECTION_ABORTED,
        COREWEBVIEW2_WEB_ERROR_STATUS_CONNECTION_RESET, COREWEBVIEW2_WEB_ERROR_STATUS_DISCONNECTED,
        COREWEBVIEW2_WEB_ERROR_STATUS_ERROR_HTTP_INVALID_SERVER_RESPONSE, COREWEBVIEW2_WEB_ERROR_STATUS_HOST_NAME_NOT_RESOLVED,
        COREWEBVIEW2_WEB_ERROR_STATUS_OPERATION_CANCELED, COREWEBVIEW2_WEB_ERROR_STATUS_REDIRECT_FAILED,
        COREWEBVIEW2_WEB_ERROR_STATUS_SERVER_UNREACHABLE, COREWEBVIEW2_WEB_ERROR_STATUS_TIMEOUT,
        COREWEBVIEW2_WEB_ERROR_STATUS_UNEXPECTED_ERROR, COREWEBVIEW2_WEB_ERROR_STATUS_UNKNOWN,
        COREWEBVIEW2_WEB_ERROR_STATUS_VALID_AUTHENTICATION_CREDENTIALS_REQUIRED,
        COREWEBVIEW2_WEB_ERROR_STATUS_VALID_PROXY_AUTHENTICATION_REQUIRED, ICoreWebView2, ICoreWebView2Controller,
    },
};
use windows::{
    Win32::{
        Foundation::{HWND, RECT},
        System::Com::{IStream, STGM_CREATE, STGM_WRITE},
        UI::{
            Shell::SHCreateStreamOnFileEx,
            WindowsAndMessaging::{DestroyWindow, GetClientRect, SW_SHOW, SW_SHOWNORMAL, SetForegroundWindow, ShowWindow},
        },
    },
    core::PCWSTR,
};

use super::util::{encode_wide, host_failure};
use crate::dpi;

/// worker 线程排给窗口线程的出站动作；`pump()` 在拥有窗口的线程排空。
pub(crate) enum Outbound {
    /// 向页面投递一条消息。
    Post(String),
    /// 关闭窗口。
    Close,
}

pub(crate) struct DesktopState {
    pub(crate) options: OpenOptions,
    pub(crate) hwnd: HWND,
    pub(crate) controller: Option<ICoreWebView2Controller>,
    pub(crate) webview: Option<ICoreWebView2>,
    /// 入站 IPC：`WebMessageReceived` 直接送进 IPC worker 线程，不在窗口消息循环执行处理者。
    pub(crate) inbox_tx: mpsc::Sender<String>,
    /// 出站动作：IPC worker 与 `IpcSink` 投递到这里，`pump()` 排空。
    pub(crate) outbox_rx: mpsc::Receiver<Outbound>,
    /// 最近一次导航失败的结构化错误，用于退出时复述并解释白屏。
    pub(crate) navigation_error: Mutex<Option<WebViewError>>,
    pub(crate) alive: bool,
    /// 窗口是否已显示。窗口以隐藏状态创建，首次真实导航完成或超时兜底时显示，
    /// 避免 WebView2 浏览器进程冷启动期间用户对着纯白窗口。
    pub(crate) revealed: AtomicBool,
    /// `open()` 完成挂载的时刻，`pump()` 据此做 [`REVEAL_TIMEOUT`] 兜底显窗。
    pub(crate) open_started: Instant,
    /// 自动化：未完成的脚本任务编号与已完成结果。
    pub(crate) pending_scripts: Mutex<Vec<AutomationId>>,
    pub(crate) script_results: Mutex<Vec<ScriptOutcome>>,
    /// frameless 窗口的拖拽矩形（客户区物理像素 x/y/w/h），由页面自绘标题栏上报。
    pub(crate) drag_regions: Mutex<Vec<(i32, i32, i32, i32)>>,
    /// 自动化：未完成的截图任务与结果。
    pub(crate) pending_captures: Mutex<Vec<AutomationId>>,
    pub(crate) capture_results: Mutex<Vec<CaptureOutcome>>,
}

impl DesktopState {
    /// 首次调用把隐藏窗口显示出来并抢前台，之后为幂等空操作。
    ///
    /// 自动化打开的隐藏窗口永不显窗：`hidden` 在 `open()` 时固定，
    /// 显窗兜底与 `focus()` 同样被该分支拦下。
    pub(crate) fn reveal_once(&self, trigger: &str) {
        if self.options.hidden {
            return;
        }
        if self.revealed.swap(true, Ordering::SeqCst) {
            return;
        }
        unsafe {
            // Windows 只让第一次 ShowWindow 使用启动信息里的显示状态，隐藏或最小化启动
            // （安装器、快捷方式、按 SW_SHOWMINNOACTIVE 启动的桥接进程）会把窗口留在
            // 不可见状态。第二次调用仍使用传入的 nCmdShow，保证窗口真正显示出来。
            let _ = ShowWindow(self.hwnd, SW_SHOW);
            let _ = ShowWindow(self.hwnd, SW_SHOWNORMAL);
            let _ = SetForegroundWindow(self.hwnd);
        }
        // controller 在隐藏父窗口上创建时，WebView2 合成器保持挂起；只 ShowWindow
        // 唤不醒它，窗口显示后仍是纯白（VM/RDP 与较新 WebView2 运行时尤其稳定复现）。
        // 显窗后强制把可见性 false→true 走一遍并复位边界，让合成器重新挂接到已可见的
        // 父窗口，页面才开始出画面。
        if let Some(controller) = &self.controller {
            unsafe {
                let _ = controller.SetIsVisible(false);
                let _ = controller.SetIsVisible(true);
            }
            self.resize_webview();
        }
        dpi::diag(format!("window.reveal trigger={trigger}"));
    }

    /// 在窗口线程排空 IPC worker 与 `IpcSink` 投递的出站动作。
    pub(crate) fn drain_outbox(&self) {
        loop {
            match self.outbox_rx.try_recv() {
                Ok(Outbound::Post(json)) => {
                    let _ = self.post_message(&json);
                }
                Ok(Outbound::Close) => {
                    unsafe {
                        let _ = DestroyWindow(self.hwnd);
                    }
                    return;
                }
                Err(_) => return,
            }
        }
    }

    pub(crate) fn resize_webview(&self) {
        let Some(controller) = self.controller.as_ref() else {
            return;
        };
        unsafe {
            let mut rect = windows::Win32::Foundation::RECT::default();
            if GetClientRect(self.hwnd, &mut rect).is_err() {
                return;
            }
            let _ = controller.SetBounds(rect);
        }
        self.log_geometry("resize");
    }

    /// 打开诊断开关时打印窗口 DPI、客户区物理/逻辑尺寸与 controller 边界。
    ///
    /// WebView2 的 `SetBounds` 单位是相对父窗口客户区的**物理像素**；
    /// 客户区来自 `GetClientRect`（Per-Monitor V2 进程返回物理像素）。
    pub(crate) fn log_geometry(&self, event: &str) {
        if !dpi::diag_enabled() {
            return;
        }
        let mut client = RECT::default();
        let client_ok = unsafe { GetClientRect(self.hwnd, &mut client) }.is_ok();
        let window_dpi = dpi::window_dpi(self.hwnd);
        let mut bounds = RECT::default();
        let bounds_ok = self.controller.as_ref().map(|controller| unsafe { controller.Bounds(&mut bounds).is_ok() }).unwrap_or(false);
        let bounds_text = if bounds_ok {
            format!("{}x{}@{},{}", bounds.right - bounds.left, bounds.bottom - bounds.top, bounds.left, bounds.top)
        } else {
            "none".to_string()
        };
        let client_text =
            if client_ok { format!("{}x{}", client.right - client.left, client.bottom - client.top) } else { "unknown".to_string() };
        dpi::diag(format!(
            "dpi.geometry event={event} dpi={window_dpi} scale={:.2} client_physical={client_text} client_logical={:.1}x{:.1} controller_bounds={bounds_text}",
            f64::from(window_dpi) / 96.0,
            dpi::physical_to_dip(client.right - client.left, window_dpi),
            dpi::physical_to_dip(client.bottom - client.top, window_dpi),
        ));
    }

    /// 记录 WebView2 导航结果；失败立即输出结构化错误，并在退出时复述。
    ///
    /// 没有这条记录时，映射或路径问题只表现为 Edge 错误页，进程仍认为窗口
    /// 正常打开，CLI 只打印 `closed (win32)`。
    pub(crate) fn report_navigation(&self, success: bool, status: COREWEBVIEW2_WEB_ERROR_STATUS, url: &str) {
        if success {
            dpi::diag(format!("navigation.completed ok url={url}"));
            return;
        }
        // 初始 `about:blank` 被真实导航取代、以及 SPA 回退主动取消的导航都会以
        // `operation_canceled` 收尾，这不是页面加载失败，不制造噪音也不进退出摘要。
        if status == COREWEBVIEW2_WEB_ERROR_STATUS_OPERATION_CANCELED || url == "about:blank" {
            dpi::diag(format!("navigation.canceled url={url} web_error_status={}", web_error_status_name(status)));
            return;
        }
        let error = WebViewError::host(
            "webview2_navigation_failed",
            "navigate",
            format!("页面未加载: url={url} web_error_status={}({})", web_error_status_name(status), status.0),
        )
        .with_attempted(vec![url.to_string()]);
        dpi::emit(format!("navigation.failed {error}"));
        // 同一次加载可能产生多个 NavigationCompleted（首次尝试与重试），
        // 退出摘要取最后一个，它才是最终失败原因。
        *self.navigation_error.lock().expect("navigation lock") = Some(error);
    }

    pub(crate) fn post_message(&self, json: &str) -> Result<()> {
        let Some(webview) = self.webview.as_ref() else {
            return Err(WebViewError::host("webview_not_ready", "post_message", "webview 尚未就绪"));
        };
        let wide = encode_wide(json);
        unsafe {
            webview.PostWebMessageAsString(PCWSTR(wide.as_ptr())).map_err(|e| host_failure("post_message_failed", "post_message", &e))?;
        }
        Ok(())
    }

    /// 发起页面脚本执行；结果在 `pump()` 驱动下经 [`Self::take_script_result`] 回收。
    pub(crate) fn evaluate_script(&self, id: AutomationId, script: &str) -> Result<()> {
        let Some(webview) = self.webview.as_ref() else {
            return Err(WebViewError::host("webview_not_ready", "evaluate", "webview 尚未就绪"));
        };
        self.pending_scripts.lock().expect("pending scripts lock").push(id);
        let state_addr = self as *const DesktopState as usize;
        let handler = ExecuteScriptCompletedHandler::create(Box::new(move |error_code, json| {
            let desktop = unsafe { &*(state_addr as *const DesktopState) };
            let outcome = match error_code {
                Ok(()) => ScriptOutcome::success(id, parse_script_json(&json)),
                Err(error) => ScriptOutcome::failure(id, AutomationError::new(automation_codes::SCRIPT_ERROR, error.message())),
            };
            desktop.finish_script(outcome);
            Ok(())
        }));
        let wide = encode_wide(script);
        unsafe { webview.ExecuteScript(PCWSTR(wide.as_ptr()), &handler).map_err(|e| host_failure("script_eval_failed", "evaluate", &e)) }
    }

    /// 发起截图；PNG 由 `CapturePreview` 写入目标文件，结果经 [`Self::take_capture_result`] 回收。
    pub(crate) fn capture_preview(&self, id: AutomationId, path: &Path) -> Result<()> {
        let Some(webview) = self.webview.as_ref() else {
            return Err(WebViewError::host("webview_not_ready", "screenshot", "webview 尚未就绪"));
        };
        // 隐藏窗口从未走显窗路径，合成器可能还没挂上父窗口：第一次 CapturePreview
        // 会一直等不到首帧。这里复用显窗时的可见性翻转与边界复位唤醒合成器。
        if let Some(controller) = &self.controller {
            unsafe {
                let _ = controller.SetIsVisible(false);
                let _ = controller.SetIsVisible(true);
            }
            self.resize_webview();
        }
        let wide_path = encode_wide(&path.display().to_string());
        let stream: IStream = unsafe {
            SHCreateStreamOnFileEx(PCWSTR(wide_path.as_ptr()), (STGM_CREATE | STGM_WRITE).0, 0, true, None::<&IStream>)
                .map_err(|e| host_failure("capture_stream_failed", "screenshot", &e))?
        };
        self.pending_captures.lock().expect("pending captures lock").push(id);
        let state_addr = self as *const DesktopState as usize;
        let target = path.to_path_buf();
        let handler = CapturePreviewCompletedHandler::create(Box::new(move |error_code| {
            let desktop = unsafe { &*(state_addr as *const DesktopState) };
            let outcome = match error_code {
                Ok(()) => CaptureOutcome { id, path: Some(target.clone()), error: None },
                Err(error) => {
                    CaptureOutcome { id, path: None, error: Some(AutomationError::new(automation_codes::CAPTURE_FAILED, error.message())) }
                }
            };
            desktop.finish_capture(outcome);
            Ok(())
        }));
        unsafe {
            webview
                .CapturePreview(COREWEBVIEW2_CAPTURE_PREVIEW_IMAGE_FORMAT_PNG, &stream, &handler)
                .map_err(|e| host_failure("capture_failed", "screenshot", &e))
        }
    }

    /// 记录脚本结果并移出待办。
    fn finish_script(&self, outcome: ScriptOutcome) {
        let id = outcome.id;
        self.pending_scripts.lock().expect("pending scripts lock").retain(|pending| *pending != id);
        self.script_results.lock().expect("script results lock").push(outcome);
    }

    /// 记录截图结果并移出待办。
    fn finish_capture(&self, outcome: CaptureOutcome) {
        let id = outcome.id;
        self.pending_captures.lock().expect("pending captures lock").retain(|pending| *pending != id);
        self.capture_results.lock().expect("capture results lock").push(outcome);
    }

    /// 取走指定脚本结果；尚未完成返回 `None`。窗口销毁后未完成项以 `window_closed` 结清。
    pub(crate) fn take_script_result(&self, id: AutomationId) -> Option<ScriptOutcome> {
        take_outcome(&self.script_results, &self.pending_scripts, id, !self.alive, |id| {
            ScriptOutcome::failure(id, AutomationError::new(automation_codes::WINDOW_CLOSED, "窗口已销毁，脚本任务被取消"))
        })
    }

    /// 取走指定截图结果；语义与 [`Self::take_script_result`] 一致。
    pub(crate) fn take_capture_result(&self, id: AutomationId) -> Option<CaptureOutcome> {
        take_outcome(&self.capture_results, &self.pending_captures, id, !self.alive, |id| CaptureOutcome {
            id,
            path: None,
            error: Some(AutomationError::new(automation_codes::WINDOW_CLOSED, "窗口已销毁，截图任务被取消")),
        })
    }

    /// 打开 WebView2 DevTools（独立窗口）。
    ///
    /// 可见控制台不再是调试入口：`F12` 与 `WEBVIEW_DIAG=1` 都经这里打开
    /// DevTools，页面控制台、网络与 DOM 检查照旧可用。
    pub(crate) fn open_dev_tools(&self, reason: &str) {
        let Some(webview) = self.webview.as_ref() else {
            dpi::emit(format!("devtools.open_failed reason={reason} error=webview_not_ready"));
            return;
        };
        match unsafe { webview.OpenDevToolsWindow() } {
            Ok(()) => dpi::emit(format!("devtools.open reason={reason}")),
            Err(error) => dpi::emit(format!("devtools.open_failed reason={reason} hresult={:#x} message={}", error.code().0, error.message())),
        }
    }
}

/// 从结果队列取走 `id`；窗口已销毁且任务仍挂起时合成 `window_closed`。
fn take_outcome<T>(
    results: &Mutex<Vec<T>>,
    pending: &Mutex<Vec<AutomationId>>,
    id: AutomationId,
    closed: bool,
    on_closed: impl FnOnce(AutomationId) -> T,
) -> Option<T>
where
    T: HasAutomationId,
{
    {
        let mut results = results.lock().expect("automation results lock");
        if let Some(index) = results.iter().position(|item| item.automation_id() == id) {
            return Some(results.remove(index));
        }
    }
    if !closed {
        return None;
    }
    let mut pending = pending.lock().expect("automation pending lock");
    let index = pending.iter().position(|item| *item == id)?;
    pending.remove(index);
    Some(on_closed(id))
}

/// 自动化结果都能给出自己的任务编号。
trait HasAutomationId {
    fn automation_id(&self) -> AutomationId;
}

impl HasAutomationId for ScriptOutcome {
    fn automation_id(&self) -> AutomationId {
        self.id
    }
}

impl HasAutomationId for CaptureOutcome {
    fn automation_id(&self) -> AutomationId {
        self.id
    }
}

/// `ExecuteScript` 返回的是表达式 JSON 文本；空串与非法 JSON 都按字符串/null 归一，不丢结果。
fn parse_script_json(json: &str) -> serde_json::Value {
    if json.trim().is_empty() {
        return serde_json::Value::Null;
    }
    match serde_json::from_str(json) {
        Ok(value) => value,
        Err(_) => serde_json::Value::String(json.to_owned()),
    }
}

/// `COREWEBVIEW2_WEB_ERROR_STATUS` 的可读名字，避免日志里只剩一个裸数字。
fn web_error_status_name(status: COREWEBVIEW2_WEB_ERROR_STATUS) -> &'static str {
    match status {
        COREWEBVIEW2_WEB_ERROR_STATUS_UNKNOWN => "unknown",
        COREWEBVIEW2_WEB_ERROR_STATUS_CERTIFICATE_COMMON_NAME_IS_INCORRECT => "certificate_common_name_is_incorrect",
        COREWEBVIEW2_WEB_ERROR_STATUS_CERTIFICATE_EXPIRED => "certificate_expired",
        COREWEBVIEW2_WEB_ERROR_STATUS_CLIENT_CERTIFICATE_CONTAINS_ERRORS => "client_certificate_contains_errors",
        COREWEBVIEW2_WEB_ERROR_STATUS_CERTIFICATE_REVOKED => "certificate_revoked",
        COREWEBVIEW2_WEB_ERROR_STATUS_CERTIFICATE_IS_INVALID => "certificate_is_invalid",
        COREWEBVIEW2_WEB_ERROR_STATUS_SERVER_UNREACHABLE => "server_unreachable",
        COREWEBVIEW2_WEB_ERROR_STATUS_TIMEOUT => "timeout",
        COREWEBVIEW2_WEB_ERROR_STATUS_ERROR_HTTP_INVALID_SERVER_RESPONSE => "error_http_invalid_server_response",
        COREWEBVIEW2_WEB_ERROR_STATUS_CONNECTION_ABORTED => "connection_aborted",
        COREWEBVIEW2_WEB_ERROR_STATUS_CONNECTION_RESET => "connection_reset",
        COREWEBVIEW2_WEB_ERROR_STATUS_DISCONNECTED => "disconnected",
        COREWEBVIEW2_WEB_ERROR_STATUS_CANNOT_CONNECT => "cannot_connect",
        COREWEBVIEW2_WEB_ERROR_STATUS_HOST_NAME_NOT_RESOLVED => "host_name_not_resolved",
        COREWEBVIEW2_WEB_ERROR_STATUS_OPERATION_CANCELED => "operation_canceled",
        COREWEBVIEW2_WEB_ERROR_STATUS_REDIRECT_FAILED => "redirect_failed",
        COREWEBVIEW2_WEB_ERROR_STATUS_UNEXPECTED_ERROR => "unexpected_error",
        COREWEBVIEW2_WEB_ERROR_STATUS_VALID_AUTHENTICATION_CREDENTIALS_REQUIRED => "valid_authentication_credentials_required",
        COREWEBVIEW2_WEB_ERROR_STATUS_VALID_PROXY_AUTHENTICATION_REQUIRED => "valid_proxy_authentication_required",
        _ => "unrecognized_status",
    }
}
