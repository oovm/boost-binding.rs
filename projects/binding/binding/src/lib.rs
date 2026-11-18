//! 外部绑定层门面：按编译目标导出当前平台可用的宿主底座与绑定样板宏。
//!
//! 下游依赖本 crate 即可拿到全部绑定层入口：
//!
//! - [`platform_module!`]：平台门控模块声明与公开项 re-export。
//! - [`dbus`]：D-Bus 客户端底座，`marshal` 跨平台可用，`conn` 仅 Linux。
//! - [`com`]：Windows COM 运行时底座，仅 Windows 存在。
//!
//! 平台差异只体现在 `com` 是否存在这一处；需要更细的控制时，可直接依赖
//! `binding-com` / `binding-dbus`。

#![deny(missing_docs)]

pub use binding_macro::platform_module;

/// D-Bus 客户端底座：线协议编解码跨平台，会话总线连接层仅 Linux。
pub use binding_dbus as dbus;

/// Windows COM 运行时底座：apartment 守卫、错误转换与任务内存回收。
#[cfg(windows)]
pub use binding_com as com;
