# iTools - 修复 Cargo Check 错误 - 产品需求文档

## 概述
- **摘要**: 本项目旨在修复 iTools 项目中的 `cargo check` 错误，确保项目能够成功编译和构建，同时移除不必要的依赖项并确保跨平台兼容性。
- **目的**: 解决项目编译错误，确保代码质量和可维护性，同时满足用户对无外部依赖的要求。
- **目标用户**: iTools 项目的开发者和维护者。

## 目标
- 修复所有 `cargo check` 错误，确保项目能够成功编译
- 移除不必要的 `serde_json` 依赖，使用 `oak_json` 作为替代
- 确保跨平台兼容性，特别是 Windows、macOS 和 Linux 平台
- 确保代码符合 Rust 最佳实践和编码规范

## 非目标（范围外）
- 添加新功能或特性
- 修改项目的核心架构
- 优化性能或内存使用

## 背景与上下文
- 项目之前存在 `serde_json` 依赖，用户明确要求移除该依赖
- 项目在 macOS 平台上存在错误，特别是 `Watcher` trait 的 `Error` 类型不匹配
- 项目在 Windows 平台上存在 `HANDLE` 类型的 `Send` 安全性问题

## 功能需求
- **FR-1**: 修复 `itools-notify` 模块在 macOS 平台上的错误，确保 `Error` 类型与其他平台一致
- **FR-2**: 修复 `itools-notify` 模块在 Windows 平台上的 `HANDLE` 类型 `Send` 安全性问题
- **FR-3**: 移除 `serde_json` 依赖，使用 `oak_json` 作为替代
- **FR-4**: 确保所有模块能够通过 `cargo check` 验证

## 非功能需求
- **NFR-1**: 代码质量：确保代码符合 Rust 编码规范和最佳实践
- **NFR-2**: 跨平台兼容性：确保代码在 Windows、macOS 和 Linux 平台上都能正常编译和运行
- **NFR-3**: 依赖管理：最小化依赖项，避免引入不必要的外部库

## 约束
- **技术**: Rust 语言，Cargo 构建系统
- **依赖**: 只能使用 `oak_json` 进行 JSON 解析，禁止使用 `serde_json`
- **平台**: 支持 Windows、macOS 和 Linux 平台

## 假设
- 项目已经正确配置了 `oak_json` 依赖
- 所有平台的代码都遵循相同的接口规范
- 项目使用标准的 Rust 工具链和构建流程

## 验收标准

### AC-1: 修复 macOS 平台的 Error 类型错误
- **给定**: 项目在 macOS 平台上构建时
- **当**: 运行 `cargo check` 命令
- **然后**: 不再出现 `Error` 类型不匹配的错误
- **验证**: `programmatic`
- **说明**: 确保 `MacOSWatcher` 的 `Error` 类型与其他平台一致，使用 `std::io::Error`

### AC-2: 修复 Windows 平台的 HANDLE Send 安全性问题
- **给定**: 项目在 Windows 平台上构建时
- **当**: 运行 `cargo check` 命令
- **然后**: 不再出现 `HANDLE` 类型 `Send` 安全性的错误
- **验证**: `programmatic`
- **说明**: 创建 `SendableHandle` 包装器并实现 `Send` trait

### AC-3: 移除 serde_json 依赖
- **给定**: 项目的 `Cargo.toml` 文件中
- **当**: 检查依赖项配置
- **然后**: 不再包含 `serde_json` 依赖
- **验证**: `programmatic`
- **说明**: 使用 `oak_json` 替代 `serde_json` 进行 JSON 解析

### AC-4: 项目通过 cargo check 验证
- **给定**: 完整的项目代码
- **当**: 在项目根目录运行 `cargo check` 命令
- **然后**: 命令执行成功，没有错误
- **验证**: `programmatic`
- **说明**: 确保所有模块都能通过 `cargo check` 验证

## 未解决的问题
- [ ] 无
