//! `Win32Backend`：窗口创建、COM 初始化、消息泵与 `WebViewBackend` trait 实现。

use std::{
    path::Path,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Instant,
};

use webview_types::{
    AutomationId, CaptureOutcome, DragRect, IpcHandler, IpcOutcome, IpcSink, OpenOptions, Platform, Result, ScriptOutcome, WebViewBackend,
    WebViewError,
};
use windows::{
    Win32::{
        Foundation::{ERROR_CLASS_ALREADY_EXISTS, GetLastError, HWND},
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Shell::DragAcceptFiles,
            WindowsAndMessaging::{
                CS_HREDRAW, CS_VREDRAW, CreateWindowExW, DestroyWindow, DispatchMessageW, IDC_ARROW, IsIconic, IsWindow, IsZoomed, LoadCursorW,
                MSG, PM_REMOVE, PeekMessageW, RegisterClassW, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, SetForegroundWindow, ShowWindow,
                TranslateMessage, WINDOW_EX_STYLE, WM_QUIT, WNDCLASSW, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_MAXIMIZEBOX, WS_MINIMIZEBOX,
                WS_OVERLAPPEDWINDOW, WS_POPUP, WS_SYSMENU, WS_THICKFRAME,
            },
        },
    },
    core::{PCWSTR, w},
};

use super::REVEAL_TIMEOUT;
use super::VIRTUAL_HOST;
use super::state::{DesktopState, Outbound};
use super::util::{encode_wide, host_failure};
use super::webview2::{attach_webview, create_controller, create_environment};
use super::window::{apply_window_icon, wnd_proc};
use crate::{console, dpi};

/// Win32 + WebView2 后端。
pub struct Win32Backend {
    state: Box<DesktopState>,
    quit: bool,
    pumps: u64,
}

impl Win32Backend {
    /// 记录退出原因；首两次 pump 就退出属于异常，必须留下结构化痕迹。
    fn report_exit(&self, reason: &'static str) {
        let is_window = unsafe { IsWindow(Some(self.state.hwnd)).as_bool() };
        let navigation = self.state.navigation_error.lock().expect("navigation lock").as_ref().map(ToString::to_string);
        let line = format!(
            "pump.exit reason={reason} pumps={} hwnd={:?} is_window={is_window} alive={} quit={} navigation_error=\"{}\"",
            self.pumps,
            self.state.hwnd,
            self.state.alive,
            self.quit,
            navigation.as_deref().unwrap_or("none"),
        );
        dpi::diag(line.clone());
        if self.pumps <= 2 {
            dpi::emit(format!("unexpected_exit {line}"));
        }
    }
}

impl WebViewBackend for Win32Backend {
    const NAME: &'static str = "win32";
    const PLATFORM: Platform = Platform::Windows;

    fn open(options: OpenOptions, handler: Box<dyn IpcHandler>) -> Result<Self> {
        // 先装文件日志再收控制台：隐藏之后 stderr 不再有人看得见。
        dpi::install_log_sink();
        let console = console::hide_orphan_console();
        dpi::emit(format!("console.window present={} attached_processes={} hidden={}", console.window, console.processes, console.hidden));

        // 必须在任何窗口创建之前声明 Per-Monitor V2；晚于窗口创建则无效，
        // 进程仍会按位图整体拉伸，页面与文字发虚。
        let awareness = dpi::ensure_process_dpi_awareness();

        unsafe {
            // COM apartment 初始化经 `binding-com` 统一：S_FALSE（同模型已初始化）与
            // ModeConflict（已按其它模型初始化）都不是致命错误——WebView2 需要 STA，
            // 若线程已按 MTA 初始化仍可继续，并在 controller 阶段如实报错。
            // 守卫被 forget：apartment 归属本线程直至进程退出，与消息泵生命周期一致。
            match binding_com::Apartment::sta() {
                Ok(apartment) => std::mem::forget(apartment),
                Err(binding_com::ApartmentState::ModeConflict) => {}
                Err(binding_com::ApartmentState::Failed(error)) => {
                    let error = windows::core::Error::from_hresult(error.hresult());
                    return Err(host_failure("com_initialize_failed", "initialize_com", &error));
                }
            }

            let class_name = w!("WEBVIEW_HOST");
            let instance = GetModuleHandleW(None).map_err(|e| host_failure("window_class_failed", "load_module", &e))?;

            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW).map_err(|e| host_failure("window_class_failed", "load_cursor", &e))?,
                hInstance: instance.into(),
                lpszClassName: class_name,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wnd_proc),
                ..Default::default()
            };
            // 同一进程内重复打开时类已存在，只有其它失败才算错误。
            if RegisterClassW(&wc) == 0 {
                let last = GetLastError();
                if last != ERROR_CLASS_ALREADY_EXISTS {
                    return Err(WebViewError::host("window_class_failed", "register_class", "RegisterClassW 失败").with_hresult(last.0 as i32));
                }
            }

            let title = encode_wide(&options.title);
            // 作者面传逻辑像素（DIP），窗口与 WebView2 边界全部按物理像素处理。
            let initial_dpi = dpi::system_dpi();
            // frameless：只摘掉 caption，保留缩放边框、最小化/最大化按钮样式与系统菜单，
            // 拖拽与双击最大化由页面标题栏上报的 `drag_regions` 命中测试承担。
            let style = if options.decorations {
                WS_OVERLAPPEDWINDOW
            } else {
                // Win11 会给 overlapped 窗口静默补回 WS_CAPTION；WS_POPUP 不受影响。
                // 保留 WS_THICKFRAME 与 min/max box：缩放边框与 Snap 仍可用，
                // 拖拽与双击最大化由页面标题栏上报的 `drag_regions` 命中测试承担。
                WS_POPUP | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_SYSMENU | WS_CLIPSIBLINGS | WS_CLIPCHILDREN
            };
            let (width, height) = dpi::client_to_window_size(options.width, options.height, style, WINDOW_EX_STYLE(0), initial_dpi);
            let (width, height) = match dpi::primary_work_area() {
                Some(area) => (width.min(area.right - area.left), height.min(area.bottom - area.top)),
                None => (width, height),
            };
            let origin = dpi::dip_to_physical(100, initial_dpi);
            let (inbox_tx, inbox_rx) = mpsc::channel::<String>();
            let (outbox_tx, outbox_rx) = mpsc::channel::<Outbound>();
            let mut box_state = Box::new(DesktopState {
                options,
                hwnd: HWND::default(),
                controller: None,
                webview: None,
                inbox_tx,
                outbox_rx,
                navigation_error: Mutex::new(None),
                alive: true,
                revealed: AtomicBool::new(false),
                open_started: Instant::now(),
                pending_scripts: Mutex::new(Vec::new()),
                script_results: Mutex::new(Vec::new()),
                drag_regions: Mutex::new(Vec::new()),
                pending_captures: Mutex::new(Vec::new()),
                capture_results: Mutex::new(Vec::new()),
            });

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                class_name,
                PCWSTR(title.as_ptr()),
                style,
                origin,
                origin,
                width,
                height,
                None,
                None,
                Some(instance.into()),
                Some(box_state.as_mut() as *mut DesktopState as *mut _),
            )
            .map_err(|e| host_failure("window_create_failed", "create_window", &e).with_attempted(vec![format!("class={}", VIRTUAL_HOST)]))?;

            box_state.hwnd = hwnd;
            apply_window_icon(hwnd, box_state.options.content_dir.as_deref());
            if box_state.options.accept_file_drop {
                DragAcceptFiles(hwnd, true);
            }
            dpi::diag(format!(
                "dpi.window created hwnd={hwnd:?} awareness={} window_dpi={} requested_physical={width}x{height}",
                awareness.name(),
                dpi::window_dpi(hwnd),
            ));
            // 窗口以隐藏状态创建：WebView2 环境、controller 与首屏导航完成前，
            // 显示出来的只会是一块纯白窗口。显窗由 DesktopState::reveal_once 在
            // 首次真实导航完成时执行，pump() 里有 REVEAL_TIMEOUT 兜底。

            let environment = create_environment()?;
            let controller = create_controller(hwnd, &environment)?;
            attach_webview(box_state.as_mut(), controller)?;

            if dpi::diag_enabled() {
                box_state.open_dev_tools("diag");
            }

            // IPC worker 线程：处理者在专用线程执行，窗口消息循环不再被业务阻塞；
            // 出站动作经 outbox 回到窗口线程，由 `pump()` 排空投递。
            let sink = {
                let tx = outbox_tx.clone();
                IpcSink::new(move |json| {
                    tx.send(Outbound::Post(json)).map_err(|_| WebViewError::host("webview_not_ready", "post_message", "窗口已销毁"))
                })
            };
            let push_close = {
                let tx = outbox_tx;
                move || tx.send(Outbound::Close).is_ok()
            };
            std::thread::Builder::new()
                .name("webview-ipc".to_string())
                .spawn(move || {
                    let mut handler = handler;
                    for message in inbox_rx.iter() {
                        match handler.on_ipc(&message, &sink) {
                            IpcOutcome::ReplyToPage(json) => {
                                let _ = sink.post(&json);
                            }
                            IpcOutcome::CloseWindow => {
                                let _ = push_close();
                            }
                            IpcOutcome::Ignored => {}
                        }
                    }
                })
                .map_err(|error| WebViewError::Message(format!("IPC worker 线程启动失败: {error}")))?;

            Ok(Self { state: box_state, quit: false, pumps: 0 })
        }
    }

    fn pump(&mut self) -> bool {
        self.pumps += 1;

        if self.quit || !self.state.alive {
            self.quit = true;
            self.report_exit("state_not_alive");
            return false;
        }

        if !unsafe { IsWindow(Some(self.state.hwnd)).as_bool() } {
            self.quit = true;
            self.state.alive = false;
            self.report_exit("window_not_found");
            return false;
        }

        unsafe {
            let mut msg = MSG::default();
            // 先抽线程级 WM_QUIT，再处理窗口消息。
            if PeekMessageW(&mut msg, None, WM_QUIT, WM_QUIT, PM_REMOVE).as_bool() {
                self.quit = true;
                self.state.alive = false;
                self.report_exit("thread_wm_quit");
                return false;
            }
            while PeekMessageW(&mut msg, Some(self.state.hwnd), 0, 0, PM_REMOVE).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
                if !self.state.alive {
                    self.quit = true;
                    self.report_exit("window_destroyed");
                    return false;
                }
            }
            // WebView2 / COM also posts thread messages with hwnd=NULL (not WM_QUIT).
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                if msg.message == WM_QUIT {
                    self.quit = true;
                    self.state.alive = false;
                    self.report_exit("dispatched_wm_quit");
                    return false;
                }
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
                if !self.state.alive {
                    self.quit = true;
                    self.report_exit("window_destroyed");
                    return false;
                }
            }
        }

        // 在窗口线程排空 IPC worker 的出站动作（回复、迟到投递、关闭）。
        self.state.drain_outbox();

        // 兜底显窗：导航迟迟不完成（网络挂起、页面永不触发完成事件）也要在
        // 超时后把窗口亮出来，用户至少能看见窗口并自行关闭。
        if !self.state.revealed.load(Ordering::SeqCst) && self.state.open_started.elapsed() >= REVEAL_TIMEOUT {
            self.state.reveal_once("timeout");
        }

        self.state.alive && !self.quit
    }

    fn close(&mut self) -> Result<()> {
        if !self.state.alive {
            return Ok(());
        }
        unsafe {
            let _ = DestroyWindow(self.state.hwnd);
        }
        // Pump once so WM_DESTROY runs.
        let _ = self.pump();
        Ok(())
    }

    fn post_message(&self, json: &str) -> Result<()> {
        self.state.post_message(json)
    }

    fn is_alive(&self) -> bool {
        self.state.alive && !self.quit
    }

    fn focus(&mut self) -> Result<()> {
        if !self.is_alive() {
            return Err(WebViewError::host("window_not_alive", "focus", "窗口已关闭，无法唤醒"));
        }
        let hwnd = self.state.hwnd;
        let known = unsafe { IsWindow(Some(hwnd)) }.as_bool();
        if !known {
            return Err(WebViewError::host("window_not_alive", "focus", "窗口句柄已失效"));
        }
        unsafe {
            if IsIconic(hwnd).as_bool() {
                let _ = ShowWindow(hwnd, SW_RESTORE);
            }
            // 已显示的窗口只抢前台；`reveal_once` 对已显示窗口是空操作。
            let _ = SetForegroundWindow(hwnd);
        }
        // 二实例聚焦：窗口尚未显示（首屏仍在加载）时同样走显窗路径，
        // 并保持 `revealed` 同步，避免超时兜底重复显窗。
        self.state.reveal_once("focus");
        Ok(())
    }

    fn evaluate(&mut self, id: AutomationId, script: &str) -> Result<()> {
        self.state.evaluate_script(id, script)
    }

    fn take_script_result(&mut self, id: AutomationId) -> Option<ScriptOutcome> {
        self.state.take_script_result(id)
    }

    fn screenshot(&mut self, id: AutomationId, path: &Path) -> Result<()> {
        self.state.capture_preview(id, path)
    }

    fn take_capture_result(&mut self, id: AutomationId) -> Option<CaptureOutcome> {
        self.state.take_capture_result(id)
    }

    fn minimize(&mut self) -> Result<()> {
        if !self.is_alive() {
            return Err(WebViewError::host("window_not_alive", "minimize", "窗口已关闭，无法最小化"));
        }
        unsafe {
            let _ = ShowWindow(self.state.hwnd, SW_MINIMIZE);
        }
        Ok(())
    }

    fn toggle_maximize(&mut self) -> Result<()> {
        if !self.is_alive() {
            return Err(WebViewError::host("window_not_alive", "toggle_maximize", "窗口已关闭，无法切换最大化"));
        }
        let hwnd = self.state.hwnd;
        let next = if unsafe { IsZoomed(hwnd) }.as_bool() { SW_RESTORE } else { SW_MAXIMIZE };
        unsafe {
            let _ = ShowWindow(hwnd, next);
        }
        Ok(())
    }

    fn set_drag_regions(&mut self, rects: &[DragRect]) -> Result<()> {
        if !self.is_alive() {
            return Err(WebViewError::host("window_not_alive", "set_drag_regions", "窗口已关闭，无法设置拖拽区域"));
        }
        let dpi = dpi::window_dpi(self.state.hwnd);
        let converted: Vec<(i32, i32, i32, i32)> = rects
            .iter()
            .map(|rect| {
                (
                    dpi::dip_to_physical(rect.x, dpi),
                    dpi::dip_to_physical(rect.y, dpi),
                    dpi::dip_to_physical(rect.width, dpi),
                    dpi::dip_to_physical(rect.height, dpi),
                )
            })
            .collect();
        *self.state.drag_regions.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = converted;
        Ok(())
    }
}
