# iTools - 修复 Cargo Check 错误 - 验证清单

- [ ] 检查 `itools-notify/src/platform/macos.rs` 文件，确保 `Error` 类型已改为 `std::io::Error`
- [ ] 检查 `itools-notify/src/platform/macos.rs` 文件，确保方法签名与 `Watcher` trait 匹配
- [ ] 检查 `itools-notify/src/platform/macos.rs` 文件，确保未使用的变量警告已修复
- [ ] 检查 `itools-notify/src/platform/windows.rs` 文件，确保 `SendableHandle` 包装器已创建
- [ ] 检查 `itools-notify/src/platform/windows.rs` 文件，确保 `SendableHandle` 已实现 `Send` trait
- [ ] 检查 `itools-notify/src/platform/windows.rs` 文件，确保 `OPEN_EXISTING` 导入位置正确
- [ ] 检查 `Cargo.toml` 文件，确保已移除 `serde_json` 依赖
- [ ] 检查 `compilers/itools-config/Cargo.toml` 文件，确保已移除 `serde_json` 依赖
- [ ] 检查 `compilers/itools-config/src/lib.rs` 文件，确保已使用 `oak_json` 替代 `serde_json`
- [ ] 检查 `compilers/itools-config/src/lib.rs` 文件，确保类型转换和借用问题已修复
- [ ] 运行 `cargo check` 命令，确保项目能够成功编译
- [ ] 检查代码是否符合 Rust 编码规范和最佳实践
