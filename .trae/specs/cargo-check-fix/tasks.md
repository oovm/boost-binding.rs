# iTools - 修复 Cargo Check 错误 - 实现计划

## [ ] 任务 1: 修复 macOS 平台的 Error 类型错误
- **优先级**: P0
- **依赖**: 无
- **描述**:
  - 修改 `itools-notify/src/platform/macos.rs` 文件
  - 将 `Error` 类型从 `Box<dyn std::error::Error>` 改为 `std::io::Error`
  - 修复方法签名以匹配 `Watcher` trait
  - 修复未使用的变量警告
- **验收标准**: AC-1
- **测试要求**:
  - `programmatic` TR-1.1: 运行 `cargo check` 命令，确保不再出现 `Error` 类型不匹配的错误
  - `human-judgement` TR-1.2: 检查代码是否符合 Rust 编码规范
- **说明**: 确保 `MacOSWatcher` 的 `Error` 类型与其他平台一致

## [ ] 任务 2: 修复 Windows 平台的 HANDLE Send 安全性问题
- **优先级**: P0
- **依赖**: 无
- **描述**:
  - 修改 `itools-notify/src/platform/windows.rs` 文件
  - 创建 `SendableHandle` 包装器结构体
  - 为 `SendableHandle` 实现 `Send` trait
  - 确保 `OPEN_EXISTING` 导入位置正确
- **验收标准**: AC-2
- **测试要求**:
  - `programmatic` TR-2.1: 运行 `cargo check` 命令，确保不再出现 `HANDLE` 类型 `Send` 安全性的错误
  - `human-judgement` TR-2.2: 检查代码是否符合 Rust 编码规范
- **说明**: 确保 `HANDLE` 类型可以安全地在线程间传递

## [ ] 任务 3: 移除 serde_json 依赖
- **优先级**: P0
- **依赖**: 无
- **描述**:
  - 修改 `Cargo.toml` 文件，移除 `serde_json` 依赖
  - 修改 `compilers/itools-config/Cargo.toml` 文件，移除 `serde_json` 依赖
  - 修改 `compilers/itools-config/src/lib.rs` 文件，使用 `oak_json` 替代 `serde_json`
  - 修复类型转换和借用问题
- **验收标准**: AC-3
- **测试要求**:
  - `programmatic` TR-3.1: 检查 `Cargo.toml` 文件，确保不再包含 `serde_json` 依赖
  - `programmatic` TR-3.2: 运行 `cargo check` 命令，确保代码能够正常编译
  - `human-judgement` TR-3.3: 检查代码是否符合 Rust 编码规范
- **说明**: 使用 `oak_json::JsonValueNode` 替代 `serde_json::Value`，使用 `oak_json::JsonObject` 替代 `serde_json::Map`

## [ ] 任务 4: 验证项目通过 cargo check
- **优先级**: P0
- **依赖**: 任务 1, 任务 2, 任务 3
- **描述**:
  - 在项目根目录运行 `cargo check` 命令
  - 确保所有模块都能通过验证
  - 修复可能出现的其他错误
- **验收标准**: AC-4
- **测试要求**:
  - `programmatic` TR-4.1: 运行 `cargo check` 命令，确保命令执行成功，没有错误
  - `human-judgement` TR-4.2: 检查代码是否符合 Rust 编码规范
- **说明**: 确保项目能够成功编译和构建
