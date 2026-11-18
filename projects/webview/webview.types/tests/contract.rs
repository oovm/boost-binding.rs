//! `webview` 合同层的集成验收：选项默认值、后端默认能力的结构化失败、
//! 错误形状（capability / platform / code / stage / hresult / attempted）与
//! 回投脚本转义。

use webview_types::{
    Capability, IpcHandler, IpcOutcome, IpcSink, OpenOptions, Platform, Result, WebViewBackend, WebViewError, WebViewHost, automation_codes,
    page_receive_script,
};

struct NoopHandler;

impl IpcHandler for NoopHandler {
    fn on_ipc(&mut self, _message: &str, _sink: &IpcSink) -> IpcOutcome {
        IpcOutcome::Ignored
    }
}

/// 未接线自动化的后端：默认方法必须给出结构化 `Unsupported`，不得伪造成功。
struct DefaultBackend {
    alive: bool,
}

impl WebViewBackend for DefaultBackend {
    const NAME: &'static str = "test";
    const PLATFORM: Platform = Platform::Linux;

    fn open(_options: OpenOptions, _handler: Box<dyn IpcHandler>) -> Result<Self> {
        Ok(Self { alive: true })
    }

    fn pump(&mut self) -> bool {
        self.alive
    }

    fn close(&mut self) -> Result<()> {
        self.alive = false;
        Ok(())
    }

    fn post_message(&self, _json: &str) -> Result<()> {
        Ok(())
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn focus(&mut self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn open_options_defaults_are_stable() {
    let from_dir = OpenOptions::from_content_dir("dist", "App");
    assert_eq!(from_dir.title, "App");
    assert_eq!(from_dir.width, 1100);
    assert_eq!(from_dir.height, 720);
    assert_eq!(from_dir.content_dir.as_deref(), Some("dist"));
    assert!(from_dir.url.is_none());
    assert!(!from_dir.accept_file_drop);
    assert!(!from_dir.hidden);

    let from_url = OpenOptions::from_url("https://example.com", "App");
    assert!(from_url.content_dir.is_none());
    assert_eq!(from_url.url.as_deref(), Some("https://example.com"));
    assert!(!from_url.accept_file_drop);
    assert!(from_url.with_hidden(true).hidden);
}

#[test]
fn automation_defaults_report_structured_unsupported() {
    let mut host = WebViewHost::<DefaultBackend>::open(OpenOptions::from_url("https://example.com", "App"), Box::new(NoopHandler))
        .expect("open test backend");

    let error = host.evaluate("1 + 1").expect_err("evaluate must be unsupported");
    assert_eq!(error.capability(), Some(Capability::ScriptEval));
    assert_eq!(error.platform(), Some(Platform::Linux));

    let error = host.screenshot(std::path::Path::new("shot.png")).expect_err("screenshot must be unsupported");
    assert_eq!(error.capability(), Some(Capability::Screenshot));

    assert!(host.poll_script_result(1).is_none());
    assert!(host.poll_capture_result(1).is_none());
}

#[test]
fn closed_window_rejects_new_automation_calls() {
    let mut host = WebViewHost::<DefaultBackend>::open(OpenOptions::from_url("https://example.com", "App"), Box::new(NoopHandler))
        .expect("open test backend");
    host.close().expect("close");
    let error = host.evaluate("1").expect_err("closed window must reject evaluate");
    assert_eq!(error.code(), Some(automation_codes::WINDOW_CLOSED));
}

#[test]
fn handler_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Box<dyn IpcHandler>>();
    let _ = NoopHandler;
}

#[test]
fn page_receive_script_escapes_js_metacharacters() {
    assert_eq!(page_receive_script("a\"b"), "if (window.__webviewReceive) window.__webviewReceive(\"a\\\"b\");");
    assert_eq!(page_receive_script("line\nline"), "if (window.__webviewReceive) window.__webviewReceive(\"line\\nline\");");
    assert_eq!(page_receive_script("a\\b"), "if (window.__webviewReceive) window.__webviewReceive(\"a\\\\b\");");
    assert!(page_receive_script("\u{2028}").contains("\\u2028"));
    assert!(page_receive_script("\u{1}").contains("\\u0001"));
}

#[test]
fn unsupported_error_carries_capability_and_platform() {
    let error = WebViewError::unsupported(Capability::EventPump, Platform::Linux);
    assert_eq!(error.capability(), Some(Capability::EventPump));
    assert_eq!(error.platform(), Some(Platform::Linux));
    match &error {
        WebViewError::Unsupported { capability, platform } => {
            assert_eq!(*capability, Capability::EventPump);
            assert_eq!(*platform, Platform::Linux);
        }
        WebViewError::Host(failure) => panic!("unexpected host error: {failure:?}"),
        WebViewError::Message(message) => panic!("unexpected message error: {message}"),
    }
    assert!(error.to_string().contains("EventPump"));

    let runtime = WebViewError::Message("boom".into());
    assert_eq!(runtime.capability(), None);
    assert_eq!(runtime.platform(), None);
}

#[test]
fn host_error_carries_code_stage_hresult_and_attempted() {
    let error = WebViewError::host("webview2_runtime_missing", "resolve_runtime", "missing runtime")
        .with_hresult(0x8000_4005u32 as i32)
        .with_attempted(["HKLM\\EdgeUpdate", "HKCU\\EdgeUpdate"]);
    assert_eq!(error.code(), Some("webview2_runtime_missing"));
    assert_eq!(error.stage(), Some("resolve_runtime"));
    assert_eq!(error.hresult(), Some(0x8000_4005u32 as i32));
    assert_eq!(error.capability(), None);
    assert_eq!(error.platform(), None);
    let text = error.to_string();
    assert!(text.contains("webview2_runtime_missing (stage resolve_runtime): missing runtime"));
    assert!(text.contains("hresult=0x80004005"));
    assert!(text.contains("tried: HKLM\\EdgeUpdate, HKCU\\EdgeUpdate"));
}
