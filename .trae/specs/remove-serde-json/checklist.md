# 移除 serde_json 依赖 - 验证清单

- [ ] 检查 itools-config/Cargo.toml 中是否已移除 serde_json 依赖
- [ ] 检查根目录 Cargo.toml 中是否已移除 serde_json 依赖
- [ ] 检查 itools-config/src/lib.rs 中是否已将 serde_json 替换为 oak_json
- [ ] 运行 cargo check 验证代码编译通过
- [ ] 检查配置管理功能是否保持不变
- [ ] 确保没有引入新的依赖