# `binding-macro`

外部绑定层的编译期样板宏。

绑定层各 crate 共享同一条约定：实现只在目标平台上存在，其它平台编译为空。
`platform_module!` 把该约定收敛成单一入口：

```rust
binding_macro::platform_module!(#[cfg(windows)] mod console;);
binding_macro::platform_module!(#[cfg(windows)] pub mod dpi;);
binding_macro::platform_module!(#[cfg(windows)] mod com { Apartment, ComError });
```

条件不成立时整段不产生任何代码，也不会去查找同名源文件。
