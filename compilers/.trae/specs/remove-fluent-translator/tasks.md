# 移除 FluentTranslator，全面转向 OakFluentTranslator - 实现计划

## [x] Task 1: 查看 itools-localization 的 Cargo.toml 文件
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - 查看当前的依赖情况，确认需要移除的外部依赖
  - 了解 oak-fluent 功能是否完整
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**:
  - `programmatic` TR-1.1: 确认 Cargo.toml 中包含 fluent-bundle、unic-langid 等依赖
- **Notes**: 确保 oak-fluent 能够完全替代这些外部依赖

## [x] Task 2: 移除 FluentTranslator 结构体及其实现
- **Priority**: P0
- **Depends On**: Task 1
- **Description**:
  - 从 lib.rs 文件中移除 FluentTranslator 结构体
  - 移除相关的方法实现和 unsafe impl Send/Sync
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-2.1: 确认代码库中不再包含 FluentTranslator 结构体
  - `programmatic` TR-2.2: 确认代码库中不再包含 FluentTranslator 的方法实现
- **Notes**: 注意保留 TranslationProvider trait，确保 API 兼容性

## [x] Task 3: 移除对外部依赖的引用
- **Priority**: P0
- **Depends On**: Task 2
- **Description**:
  - 移除对 fluent-bundle、unic-langid 等外部依赖的引用
  - 清理相关的导入语句
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**:
  - `programmatic` TR-3.1: 确认代码中不再引用 fluent-bundle
  - `programmatic` TR-3.2: 确认代码中不再引用 unic-langid
- **Notes**: 确保只移除与 FluentTranslator 相关的依赖引用

## [x] Task 4: 修改全局 I18N 实例初始化
- **Priority**: P0
- **Depends On**: Task 3
- **Description**:
  - 移除条件编译，直接使用 OakFluentTranslator
  - 简化 I18N 实例的初始化代码
- **Acceptance Criteria Addressed**: AC-3, AC-4
- **Test Requirements**:
  - `programmatic` TR-4.1: 确认全局 I18N 实例始终使用 OakFluentTranslator
  - `programmatic` TR-4.2: 确认不再使用条件编译选择翻译提供者
- **Notes**: 保持 API 接口不变，确保现有代码无需修改

## [x] Task 5: 更新 Cargo.toml 文件
- **Priority**: P0
- **Depends On**: Task 3
- **Description**:
  - 从 Cargo.toml 中移除 fluent-bundle、unic-langid 等依赖
  - 确保 oak-fluent 依赖正确配置
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**:
  - `programmatic` TR-5.1: 确认 Cargo.toml 中不再包含 fluent-bundle 依赖
  - `programmatic` TR-5.2: 确认 Cargo.toml 中不再包含 unic-langid 依赖
- **Notes**: 确保其他依赖项保持不变

## [x] Task 6: 运行测试，确保功能正常
- **Priority**: P0
- **Depends On**: Task 4, Task 5
- **Description**:
  - 运行 itools-localization 模块的测试
  - 确保所有测试通过
  - 验证国际化功能正常工作
- **Acceptance Criteria Addressed**: AC-4, AC-5
- **Test Requirements**:
  - `programmatic` TR-6.1: 所有测试通过
  - `programmatic` TR-6.2: 国际化功能正常工作
- **Notes**: 确保构建过程无错误

## [x] Task 7: 清理代码，移除不必要的注释和条件编译
- **Priority**: P1
- **Depends On**: Task 6
- **Description**:
  - 移除与 FluentTranslator 相关的注释
  - 清理不必要的条件编译代码
  - 确保代码结构清晰
- **Acceptance Criteria Addressed**: NFR-1, NFR-4
- **Test Requirements**:
  - `human-judgment` TR-7.1: 代码结构清晰，无不必要的注释
  - `human-judgment` TR-7.2: 无不必要的条件编译代码
- **Notes**: 保持代码风格一致，符合项目规范