//! WebView2 Runtime 探测、环境与 controller 创建及事件桥接。

use std::{ffi::c_void, path::Path, sync::mpsc};

use webview::{Result, WebViewError};
use webview2_com::{
    AcceleratorKeyPressedEventHandler, CoreWebView2EnvironmentOptions, CreateCoreWebView2ControllerCompletedHandler,
    CreateCoreWebView2EnvironmentCompletedHandler,
    Microsoft::Web::WebView2::Win32::{
        COREWEBVIEW2_KEY_EVENT_KIND, COREWEBVIEW2_KEY_EVENT_KIND_KEY_DOWN, COREWEBVIEW2_WEB_ERROR_STATUS,
        COREWEBVIEW2_WEB_ERROR_STATUS_OPERATION_CANCELED, CreateCoreWebView2EnvironmentWithOptions,
        GetAvailableCoreWebView2BrowserVersionString, ICoreWebView2Controller, ICoreWebView2Controller4, ICoreWebView2Environment,
        ICoreWebView2EnvironmentOptions,
    },
    NavigationCompletedEventHandler, WebMessageReceivedEventHandler, wait_with_pump,
};
use windows::{
    Win32::{
        Foundation::{E_POINTER, E_UNEXPECTED, HWND},
        System::Com::CoTaskMemFree,
        UI::Input::KeyboardAndMouse::VK_F12,
    },
    core::{Interface, PCWSTR, PWSTR},
};

use super::EventRegistrationToken;
use super::spa::{attach_spa_fallback, resolve_navigate_url};
use super::state::DesktopState;
use super::util::{encode_wide, host_failure};
use crate::dpi;

/// WebView2 Evergreen Runtime 的常见探测位置，用于「运行时缺失」的排障提示。
fn runtime_probe_paths() -> Vec<String> {
    let mut paths = vec![
        r"HKLM\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}\pv".to_string(),
        r"HKCU\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}\pv".to_string(),
    ];
    if let Some(local) = std::env::var_os("LOCALAPPDATA") {
        paths.push(format!("{}\\Microsoft\\EdgeWebView\\Application", Path::new(&local).display()));
    }
    paths
}

/// 创建环境前先确认 WebView2 Runtime 存在；缺失直接给结构化错误，不让进程静默退出。
fn webview2_runtime_version() -> Result<String> {
    let mut version = PWSTR::null();
    let probe = unsafe { GetAvailableCoreWebView2BrowserVersionString(PCWSTR::null(), &mut version) };
    if probe.is_ok() {
        let text = unsafe { version.to_string() }.unwrap_or_default();
        if !version.is_null() {
            unsafe { CoTaskMemFree(Some(version.as_ptr() as *const c_void)) };
        }
        dpi::diag(format!("webview2.runtime version={text}"));
        return Ok(text);
    }

    let error = probe.expect_err("probe failed");
    Err(WebViewError::host(
        "webview2_runtime_missing",
        "resolve_runtime",
        "未检测到 WebView2 Runtime，页面无法渲染；请安装 Microsoft Edge WebView2 Runtime（Evergreen）后重试",
    )
    .with_hresult(error.code().0)
    .with_attempted(runtime_probe_paths()))
}

pub(crate) fn create_environment() -> Result<ICoreWebView2Environment> {
    let _ = webview2_runtime_version()?;
    let (tx, rx) = mpsc::channel();
    // VM / RDP 会话里 Chromium 的窗口遮挡检测会把窗口误判为完全遮挡，暂停合成，
    // 窗口显示后永远白屏。禁用 `CalculateNativeWinOcclusion` 是官方 workaround。
    let options = CoreWebView2EnvironmentOptions::default();
    unsafe {
        options.set_additional_browser_arguments("--disable-features=CalculateNativeWinOcclusion".to_string());
    }
    let options: ICoreWebView2EnvironmentOptions = options.into();
    unsafe {
        CreateCoreWebView2EnvironmentWithOptions(
            PCWSTR::null(),
            PCWSTR::null(),
            Some(&options),
            &CreateCoreWebView2EnvironmentCompletedHandler::create(Box::new(move |error_code, environment| {
                error_code?;
                tx.send(environment.ok_or_else(|| windows::core::Error::from(E_POINTER))).map_err(|_| windows::core::Error::from(E_UNEXPECTED))
            })),
        )
        .map_err(|e| host_failure("webview2_environment_failed", "create_environment", &e))?;
    }
    wait_with_pump(rx)
        .map_err(|e| WebViewError::host("webview2_environment_failed", "create_environment", format!("等待环境创建消息失败: {e:?}")))?
        .map_err(|e| host_failure("webview2_environment_failed", "create_environment", &e))
}

pub(crate) fn create_controller(hwnd: HWND, environment: &ICoreWebView2Environment) -> Result<ICoreWebView2Controller> {
    let (tx, rx) = mpsc::channel();
    let handler = CreateCoreWebView2ControllerCompletedHandler::create(Box::new(move |error_code, controller| {
        error_code?;
        tx.send(controller.ok_or_else(|| windows::core::Error::from(E_POINTER))).map_err(|_| windows::core::Error::from(E_UNEXPECTED))
    }));
    unsafe {
        environment
            .CreateCoreWebView2Controller(hwnd, &handler)
            .map_err(|e| host_failure("webview2_controller_failed", "create_controller", &e))?;
    }
    wait_with_pump(rx)
        .map_err(|e| WebViewError::host("webview2_controller_failed", "create_controller", format!("等待 controller 创建消息失败: {e:?}")))?
        .map_err(|e| host_failure("webview2_controller_failed", "create_controller", &e))
}

pub(crate) fn attach_webview(state: *mut DesktopState, controller: ICoreWebView2Controller) -> Result<()> {
    let webview = unsafe { controller.CoreWebView2().map_err(|e| host_failure("webview2_controller_failed", "read_core_webview", &e))? };

    // 拖放走宿主 `WM_DROPFILES` 才能拿到完整本地路径；Chromium 自身吞掉 drop 时
    // 页面只能看到文件名。关掉外部 drop 失败不阻断开窗，旧 Runtime 降级为默认行为。
    if unsafe { (*state).options.accept_file_drop } {
        match controller.cast::<ICoreWebView2Controller4>() {
            Ok(controller4) => {
                if let Err(error) = unsafe { controller4.SetAllowExternalDrop(false) } {
                    dpi::diag(format!("external_drop.disable_failed hresult={:#x}", error.code().0));
                }
            }
            Err(error) => dpi::diag(format!("external_drop.interface_missing hresult={:#x}", error.code().0)),
        }
    }

    let state_addr = state as usize;
    let mut token = EventRegistrationToken::default();
    let message_handler = WebMessageReceivedEventHandler::create(Box::new(move |_, args| {
        let Some(args) = args else {
            return Ok(());
        };
        let mut message = windows::core::PWSTR::null();
        unsafe {
            args.TryGetWebMessageAsString(&mut message)?;
        }
        let text = unsafe { message.to_string().unwrap_or_default() };
        let desktop = unsafe { &*(state_addr as *const DesktopState) };
        // 入站消息直接交给 IPC worker 线程；窗口消息循环不再执行处理者。
        let _ = desktop.inbox_tx.send(text);
        Ok(())
    }));

    unsafe {
        webview
            .add_WebMessageReceived(&message_handler, &mut token)
            .map_err(|e| host_failure("webview2_message_channel_failed", "attach_message_channel", &e))?;
    }

    // 导航失败是「窗口在、页面白」的唯一可观测信号，必须订阅并结构化记录。
    let mut navigation_token = EventRegistrationToken::default();
    let navigation_handler = NavigationCompletedEventHandler::create(Box::new(move |sender, args| {
        let (Some(sender), Some(args)) = (sender, args) else {
            return Ok(());
        };
        let mut success = windows::core::BOOL::default();
        unsafe {
            args.IsSuccess(&mut success)?;
        }
        let mut status = COREWEBVIEW2_WEB_ERROR_STATUS::default();
        unsafe {
            args.WebErrorStatus(&mut status)?;
        }
        let mut source = PWSTR::null();
        let url = unsafe { sender.Source(&mut source) }.ok().and_then(|()| unsafe { source.to_string() }.ok()).unwrap_or_default();
        let desktop = unsafe { &*(state_addr as *const DesktopState) };
        desktop.report_navigation(success.as_bool(), status, &url);
        // 延迟显窗：首次「真实」导航完成才显示窗口。`about:blank` 的初始文档
        // 与 SPA 回退主动取消的导航不算真实完成，显示它们只会重新露出白屏；
        // 导航失败（非取消）也要显示，否则用户看不到错误页。
        if url != "about:blank" && status != COREWEBVIEW2_WEB_ERROR_STATUS_OPERATION_CANCELED {
            desktop.reveal_once("navigation");
        }
        Ok(())
    }));

    unsafe {
        webview
            .add_NavigationCompleted(&navigation_handler, &mut navigation_token)
            .map_err(|e| host_failure("webview2_navigation_events_failed", "attach_navigation_events", &e))?;
    }

    // F12 是宿主约定：WebView2 先把按键交给宿主（`AcceleratorKeyPressed`），
    // 页面有焦点时也只有这里能拦到。DevTools 由此打开，不再依赖可见控制台。
    let mut accelerator_token = EventRegistrationToken::default();
    let accelerator_handler = AcceleratorKeyPressedEventHandler::create(Box::new(move |_, args| {
        let Some(args) = args else {
            return Ok(());
        };
        let mut kind = COREWEBVIEW2_KEY_EVENT_KIND::default();
        unsafe {
            args.KeyEventKind(&mut kind)?;
        }
        if kind != COREWEBVIEW2_KEY_EVENT_KIND_KEY_DOWN {
            return Ok(());
        }
        let mut key = 0u32;
        unsafe {
            args.VirtualKey(&mut key)?;
        }
        if key != u32::from(VK_F12.0) {
            return Ok(());
        }
        let desktop = unsafe { &*(state_addr as *const DesktopState) };
        desktop.open_dev_tools("f12");
        unsafe {
            args.SetHandled(true)?;
        }
        Ok(())
    }));

    unsafe {
        controller
            .add_AcceleratorKeyPressed(&accelerator_handler, &mut accelerator_token)
            .map_err(|e| host_failure("webview2_accelerator_failed", "attach_accelerator", &e))?;
    }

    // 虚拟主机把不存在的无扩展名路径当作连接失败，SPA 直达路由因此白屏。
    // 回退挂在导航事件上，覆盖首屏、重载与站内链接跳转。
    attach_spa_fallback(state, &webview)?;

    let desktop = unsafe { &mut *state };
    let navigate_url = resolve_navigate_url(&desktop.options, &webview)?;

    desktop.controller = Some(controller);
    desktop.webview = Some(webview.clone());
    desktop.resize_webview();

    let wide_url = encode_wide(&navigate_url);
    unsafe {
        webview
            .Navigate(PCWSTR(wide_url.as_ptr()))
            .map_err(|e| host_failure("webview2_navigate_failed", "navigate", &e).with_attempted(vec![navigate_url.clone()]))?;
    }

    Ok(())
}
