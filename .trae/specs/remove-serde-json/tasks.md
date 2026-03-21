# 移除 serde_json 依赖 - 实现计划

## [x] 任务 1: 分析 oak-json 库的 API
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 分析 oak-json 库的 API，了解它如何替代 serde_json 的功能
  - 特别关注 Value 和 Map 类型的使用方式
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**:
  - `human-judgement` TR-1.1: 确认 oak-json 提供了与 serde_json 类似的功能
- **Notes**: 需要查看 oak-json 的文档或源码

## [/] 任务 2: 移除 serde_json 依赖
- **Priority**: P0
- **Depends On**: 任务 1
- **Description**:
  - 从 itools-config/Cargo.toml 中移除 serde_json 依赖
  - 从根目录 Cargo.toml 中移除 serde_json 依赖
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-2.1: 确认 Cargo.toml 中不再包含 serde_json 依赖
- **Notes**: 确保其他模块没有依赖 serde_json

## [ ] 任务 3: 替换代码中的 serde_json 为 oak-json
- **Priority**: P0
- **Depends On**: 任务 1, 任务 2
- **Description**:
  - 更新 itools-config/src/lib.rs 中的导入语句
  - 替换所有 serde_json::Value 为 oak_json::JsonValueNode
  - 替换所有 serde_json::Map 为 oak_json::JsonObject
  - 更新所有相关的方法调用和类型转换
- **Acceptance Criteria Addressed**: AC-2, AC-3
- **Test Requirements**:
  - `programmatic` TR-3.1: 代码能够正常编译
  - `programmatic` TR-3.2: 配置管理功能保持不变
- **Notes**: 需要注意 API 差异，可能需要调整代码

## [ ] 任务 4: 测试编译
- **Priority**: P1
- **Depends On**: 任务 3
- **Description**:
  - 运行 cargo check 命令验证代码编译通过
  - 检查是否有任何错误或警告
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `programmatic` TR-4.1: cargo check 命令执行成功，无错误
- **Notes**: 确保所有依赖都正确解析
  - `programmatic` TR-3.2: 配置管理功能保持不变
- **Notes**: 需要注意 API 差异，可能需要调整代码

## [ ] 任务 4: 测试编译
- **Priority**: P1
- **Depends On**: 任务 3
- **Description**:
  - 运行 cargo check 命令验证代码编译通过
  - 检查是否有任何错误或警告
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `programmatic` TR-4.1: cargo check 命令执行成功，无错误
- **Notes**: 确保所有依赖都正确解析