//! `platform_module!` 的门控语义：条件为假时既声明模块，也不产生 re-export。

// 恒假条件：若宏无条件展开 `mod never_defined;`，本文件会因找不到源文件而编译失败。
binding_macro::platform_module!(#[cfg(any())] mod never_defined { MISSING });

// 恒假条件下的公开模块同样不应出现在本 crate 的命名空间里。
binding_macro::platform_module!(
    #[cfg(any())]
    pub mod never_public;
);

#[test]
fn gated_items_are_absent_when_the_condition_does_not_hold() {
    // 能编译到此处即证明上面的门控生效。
}
