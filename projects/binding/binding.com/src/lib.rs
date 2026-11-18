//! Windows COM 运行时底座。
//!
//! 本 crate 是 Boost 外部绑定层的 COM 基础设施 owner：统一提供 apartment
//! 初始化守卫、`HRESULT` 错误转换与 `CoTaskMem` 内存回收，下游绑定不再
//! 各自手搓 `CoInitializeEx`、错误处理与释放逻辑。
//!
//! 非 Windows 目标上本 crate 编译为空，不提供任何能力。

#![deny(missing_docs)]

#[cfg(windows)]
mod com;

#[cfg(windows)]
pub use com::{Apartment, ApartmentState, CoTaskMem, ComError, create_instance};
