# 移除 serde_json 依赖 - 产品需求文档

## Overview
- **Summary**: 从 itools-config 模块中移除 serde_json 依赖，使用现有的 oak-json 库作为替代，以减少项目依赖并保持代码一致性。
- **Purpose**: 解决用户对引入 serde_json 等外部库的不满，同时保持配置管理功能的完整性。
- **Target Users**: 项目开发者和维护者。

## Goals
- 移除 serde_json 依赖
- 使用 oak-json 作为配置数据的表示和操作库
- 保持配置管理功能的完整性
- 确保代码编译通过

## Non-Goals (Out of Scope)
- 修改配置管理的核心功能
- 更改配置文件的格式或解析逻辑
- 影响其他模块的功能

## Background & Context
- 当前 itools-config 模块使用 serde_json 来表示和操作配置数据
- 项目已经依赖了 oak-json 库，该库提供了类似的功能
- 用户希望减少外部依赖，特别是避免引入 serde_json 等"垃圾库"

## Functional Requirements
- **FR-1**: 移除 serde_json 依赖
- **FR-2**: 使用 oak-json 替代 serde_json 作为配置数据的表示和操作库
- **FR-3**: 保持配置管理的所有功能不变
- **FR-4**: 确保代码编译通过

## Non-Functional Requirements
- **NFR-1**: 代码质量保持不变
- **NFR-2**: 性能保持不变
- **NFR-3**: 代码风格保持一致

## Constraints
- **Technical**: 必须使用 oak-json 作为 serde_json 的替代品
- **Dependencies**: 依赖 oak-json 库的功能

## Assumptions
- oak-json 库提供了与 serde_json 类似的功能
- 修改后的代码能够通过编译和测试

## Acceptance Criteria

### AC-1: 移除 serde_json 依赖
- **Given**: itools-config 模块当前依赖 serde_json
- **When**: 移除 serde_json 依赖并更新代码
- **Then**: 项目不再依赖 serde_json
- **Verification**: `programmatic`

### AC-2: 使用 oak-json 替代 serde_json
- **Given**: 代码中使用了 serde_json 的功能
- **When**: 替换为使用 oak-json 的相应功能
- **Then**: 代码能够正常编译和运行
- **Verification**: `programmatic`

### AC-3: 保持配置管理功能
- **Given**: 配置管理功能正常工作
- **When**: 替换 serde_json 为 oak-json
- **Then**: 配置管理功能保持不变
- **Verification**: `programmatic`

### AC-4: 代码编译通过
- **Given**: 修改后的代码
- **When**: 运行 cargo check
- **Then**: 代码编译通过，无错误
- **Verification**: `programmatic`

## Open Questions
- [ ] oak-json 是否提供了与 serde_json 完全兼容的 API？
- [ ] 是否需要修改其他模块以适应这一变化？