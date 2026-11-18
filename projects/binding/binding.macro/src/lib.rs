//! 外部绑定层的编译期样板宏。
//!
//! 绑定层各 crate 共享同一条约定：实现只在目标平台上存在，其它平台编译为空。
//! 这里把该约定收敛成单一入口，避免每处手写成对的 `#[cfg(..)]`。

#![deny(missing_docs)]

/// 按给定条件编译声明模块，并在需要时把模块的公开项 re-export 到当前位置。
///
/// 三种形态：
///
/// ```ignore
/// // 私有模块
/// binding_macro::platform_module!(#[cfg(windows)] mod console;);
///
/// // 公开模块
/// binding_macro::platform_module!(#[cfg(windows)] pub mod dpi;);
///
/// // 私有模块 + 公开项 re-export
/// binding_macro::platform_module!(#[cfg(windows)] mod com { Apartment, ComError });
/// ```
///
/// 条件不成立时整段不产生任何代码，也不会去查找同名源文件——这正是
/// 「非本平台编译为空」的实现方式。
#[macro_export]
macro_rules! platform_module {
    ($(#[$attr:meta])* pub mod $name:ident;) => {
        $(#[$attr])*
        pub mod $name;
    };
    ($(#[$attr:meta])* mod $name:ident;) => {
        $(#[$attr])*
        mod $name;
    };
    ($(#[$attr:meta])* mod $name:ident { $($item:ident),* $(,)? }) => {
        $(#[$attr])*
        mod $name;

        $(#[$attr])*
        pub use $name::{$($item),*};
    };
}
