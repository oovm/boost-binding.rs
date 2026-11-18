//! 虚拟主机内容映射与 SPA 直达路由回退。

use std::path::Path;

use webview_types::{OpenOptions, Result, WebViewError};
use webview2_com::{
    AddScriptToExecuteOnDocumentCreatedCompletedHandler,
    Microsoft::Web::WebView2::Win32::{COREWEBVIEW2_HOST_RESOURCE_ACCESS_KIND_ALLOW, ICoreWebView2, ICoreWebView2_3},
    NavigationStartingEventHandler,
};
use windows::core::{Interface, PCWSTR, PWSTR};

use super::EventRegistrationToken;
use super::SPA_ROUTE_PARAM;
use super::VIRTUAL_HOST;
use super::state::DesktopState;
use super::util::{encode_wide, host_failure};
use crate::dpi;

/// SPA 直达路由回退。
///
/// 虚拟主机把不存在的无扩展名路径当成连接失败（实测 `connection_aborted`），
/// 直达 `/charts` 这类路由会白屏。映射主机上的请求不会触发
/// `WebResourceRequested`（实测只有非映射主机触发），所以回退只能做在导航层：
/// 命中边界时取消这次导航，改为加载 `index.html`，并让文档创建后、页面脚本
/// 运行前把地址栏恢复成原始路径，前端仍按 `location.pathname` 路由。
///
/// 边界，三条同时满足才回退：
/// 1. 只处理 `app.local` 下的文档导航（文档导航只有 `GET`，子资源不经过这里）；
/// 2. 路径末段没有扩展名（`/assets/app.js` 之类原样交给虚拟主机）；
/// 3. 内容目录里确实没有该文件（真实存在的无扩展名文件照常加载）。
///
/// 挂在 `NavigationStarting` 上，因此首屏、页面重载与站内链接跳转都被覆盖；
/// 回退目标带 `.html` 扩展名，不会被再次命中，不存在回退环。
pub(crate) fn attach_spa_fallback(state: *mut DesktopState, webview: &ICoreWebView2) -> Result<()> {
    let desktop = unsafe { &*state };
    let Some(content_dir) = desktop.options.content_dir.clone() else {
        return Ok(());
    };
    let Ok(folder) = Path::new(&content_dir).canonicalize() else {
        return Ok(());
    };
    if !folder.join("index.html").is_file() {
        return Ok(());
    }

    install_spa_bootstrap(webview, &folder)?;

    let mut token = EventRegistrationToken::default();
    let folder_label = folder.display().to_string();
    let handler = NavigationStartingEventHandler::create(Box::new(move |sender, args| {
        let (Some(sender), Some(args)) = (sender, args) else {
            return Ok(());
        };
        let mut uri = PWSTR::null();
        unsafe {
            args.Uri(&mut uri)?;
        }
        let uri = unsafe { uri.to_string() }.unwrap_or_default();
        let Some(target) = spa_fallback_target(&folder, &uri) else {
            return Ok(());
        };
        unsafe {
            args.SetCancel(true)?;
            let wide = encode_wide(&target);
            sender.Navigate(PCWSTR(wide.as_ptr()))?;
        }
        dpi::diag(format!("spa.fallback uri={uri} target={target}"));
        Ok(())
    }));

    unsafe {
        webview.add_NavigationStarting(&handler, &mut token).map_err(|e| host_failure("webview2_fallback_failed", "attach_fallback", &e))?;
    }
    dpi::diag(format!("spa.fallback enabled folder={folder_label}"));
    Ok(())
}

/// 在文档脚本运行前把 `index.html?__webview_route=...` 的地址栏恢复成原始路由。
fn install_spa_bootstrap(webview: &ICoreWebView2, folder: &Path) -> Result<()> {
    let script = encode_wide(&spa_bootstrap_script());
    let handler = AddScriptToExecuteOnDocumentCreatedCompletedHandler::create(Box::new(|_, _| Ok(())));
    unsafe {
        webview
            .AddScriptToExecuteOnDocumentCreated(PCWSTR(script.as_ptr()), &handler)
            .map_err(|e| host_failure("webview2_fallback_failed", "attach_bootstrap", &e))?;
    }
    dpi::diag(format!("spa.bootstrap attached folder={}", folder.display()));
    Ok(())
}

/// 前置脚本正文：把回退标记换回原始路由，其余查询参数与片段原样保留。
fn spa_bootstrap_script() -> String {
    format!(
        r#"(function () {{
    var marker = '{SPA_ROUTE_PARAM}=';
    var search = location.search || '';
    if (search.indexOf(marker) === -1) return;
    var parts = search.slice(1).split('&');
    var route = null;
    var rest = [];
    for (var i = 0; i < parts.length; i++) {{
        var part = parts[i];
        if (!part) continue;
        if (part.indexOf(marker) === 0) {{
            try {{ route = decodeURIComponent(part.slice(marker.length)); }} catch (error) {{ return; }}
        }} else {{
            rest.push(part);
        }}
    }}
    if (!route || route.charAt(0) !== '/') return;
    history.replaceState(history.state, '', route + (rest.length ? '?' + rest.join('&') : '') + (location.hash || ''));
}})();"#
    )
}

/// 命中回退边界时返回 `index.html` 加路由标记的地址。
fn spa_fallback_target(folder: &Path, uri: &str) -> Option<String> {
    let rest = uri.strip_prefix("https://app.local")?;
    let path = rest.split(['?', '#']).next().unwrap_or(rest);
    if path.is_empty() || path.contains("..") || last_segment_has_extension(path) {
        return None;
    }
    if folder.join(path.trim_start_matches('/')).is_file() {
        return None;
    }
    Some(format!("https://{VIRTUAL_HOST}/index.html?{SPA_ROUTE_PARAM}={}", encode_route(path)))
}

/// 路径末段是否带扩展名；`/charts`、`/` 不带，`/main.js` 带。
fn last_segment_has_extension(path: &str) -> bool {
    path.rsplit('/').next().unwrap_or_default().contains('.')
}

/// 把路由路径编码进查询参数，只保留 RFC 3986 非保留字符与 `/`。
fn encode_route(path: &str) -> String {
    let mut encoded = String::with_capacity(path.len());
    for byte in path.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~' | b'/') {
            encoded.push(byte as char);
            continue;
        }
        encoded.push('%');
        encoded.push_str(&format!("{byte:02X}"));
    }
    encoded
}

pub(crate) fn resolve_navigate_url(options: &OpenOptions, webview: &ICoreWebView2) -> Result<String> {
    if let Some(dir) = options.content_dir.as_ref() {
        let folder = Path::new(dir).canonicalize().map_err(|e| {
            WebViewError::host("content_dir_invalid", "resolve_content", format!("content_dir {dir}: {e}")).with_attempted(vec![dir.clone()])
        })?;
        if !folder.is_dir() {
            return Err(WebViewError::host(
                "content_dir_invalid",
                "resolve_content",
                format!("content_dir is not a directory: {}", folder.display()),
            )
            .with_attempted(vec![folder.display().to_string()]));
        }
        let folder_s = strip_extended_path(&folder);
        let host = encode_wide(VIRTUAL_HOST);
        let folder_w = encode_wide(&folder_s);
        let webview3: ICoreWebView2_3 = webview.cast().map_err(|e| host_failure("webview2_interface_missing", "map_content_dir", &e))?;
        unsafe {
            webview3
                .SetVirtualHostNameToFolderMapping(
                    PCWSTR(host.as_ptr()),
                    PCWSTR(folder_w.as_ptr()),
                    COREWEBVIEW2_HOST_RESOURCE_ACCESS_KIND_ALLOW,
                )
                .map_err(|e| host_failure("webview2_virtual_host_failed", "map_content_dir", &e).with_attempted(vec![folder_s.clone()]))?;
        }
        if let Some(url) = options.url.clone() {
            return Ok(url);
        }
        return Ok(format!("https://{VIRTUAL_HOST}/index.html"));
    }

    options.url.clone().ok_or_else(|| WebViewError::host("content_source_missing", "resolve_content", "OpenOptions 需要 content_dir 或 url"))
}

fn strip_extended_path(path: &Path) -> String {
    let mut s = path.display().to_string();
    if let Some(rest) = s.strip_prefix(r"\\?\") {
        s = rest.to_string();
    }
    s
}
