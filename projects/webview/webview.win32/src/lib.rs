//! Win32 + WebView2 后端实现：[`Win32Backend`]。
//!
//! 非 Windows 目标上本 crate 不提供任何能力，且不会被门面选中。

#![deny(missing_docs)]

binding_macro::platform_module!(
    #[cfg(windows)]
    mod console;
);

binding_macro::platform_module!(
    /// 宿主 DPI 感知、单位换算与诊断输出工具：供后端、探针示例与验收测试使用。
    #[cfg(windows)]
    pub mod dpi;
);

binding_macro::platform_module!(#[cfg(windows)] mod win32 { Win32Backend });
