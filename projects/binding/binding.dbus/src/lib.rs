//! D-Bus 协议客户端底座：`binding-dbus`。
//!
//! 定位：Boost 生态外部绑定层的 Linux 总线基座。portal 文件对话框、Secret Service、
//! 桌面通知、logind、journal 等下游一律经本 crate 收发报文，不再手搓线协议。
//! 设计红线是零外部 crate 依赖：协议编解码与连接全部基于 `std` 自实现，
//! 连接层只使用 `std::os::unix`。
//!
//! 分层：
//! - [`marshal`]：D-Bus 线协议编解码，**跨平台**编译并测试（非 Linux 主机可验证协议实现）。
//! - [`conn`]：Unix socket 连接、SASL 认证、方法调用与信号读取，仅 Linux 可用
//!   （`#[cfg(target_os = "linux")]` 门控）。
//!
//! 非 Linux 平台上本 crate 只暴露 [`marshal`] 层；[`conn`] 层不参与编译，
//! 其可用性验证由 Linux CI 承担。

#![deny(missing_docs)]

pub mod marshal;

binding_macro::platform_module!(
    #[cfg(target_os = "linux")]
    pub mod conn;
);
