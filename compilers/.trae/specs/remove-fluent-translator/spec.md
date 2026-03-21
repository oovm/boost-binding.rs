# 移除 FluentTranslator，全面转向 OakFluentTranslator - 产品需求文档

## Overview
- **Summary**: 本项目旨在从 itools-localization 模块中移除 FluentTranslator 实现，全面转向使用 OakFluentTranslator，以简化代码结构并统一国际化实现。
- **Purpose**: 减少代码复杂度，移除对外部 fluent-bundle 库的依赖，统一使用项目内部的 oak-fluent 实现。
- **Target Users**: 开发人员和维护人员，以及使用 itools 国际化功能的应用。

## Goals
- 移除 FluentTranslator 结构体及其相关实现
- 确保 OakFluentTranslator 成为唯一的 Fluent 翻译实现
- 保持 API 兼容性，确保现有代码无需修改
- 移除对 fluent-bundle、unic-langid 等外部依赖
- 确保国际化功能正常工作

## Non-Goals (Out of Scope)
- 不修改 SimpleTranslator 的实现
- 不修改 TranslationProvider trait 的定义
- 不修改 I18nContext 的核心功能
- 不修改公共 API 接口

## Background & Context
- 当前 itools-localization 模块同时支持 FluentTranslator 和 OakFluentTranslator 两种实现
- FluentTranslator 使用外部的 fluent-bundle 库，而 OakFluentTranslator 使用项目内部的 oak-fluent 实现
- 为了简化代码结构和减少外部依赖，决定统一使用 OakFluentTranslator

## Functional Requirements
- **FR-1**: 移除 FluentTranslator 结构体及其所有方法实现
- **FR-2**: 移除对 fluent-bundle、unic-langid 等外部依赖
- **FR-3**: 确保 OakFluentTranslator 成为默认的翻译实现
- **FR-4**: 保持 TranslationProvider trait 不变，确保 API 兼容性
- **FR-5**: 确保全局 I18N 实例正确初始化并使用 OakFluentTranslator

## Non-Functional Requirements
- **NFR-1**: 代码结构清晰，移除不必要的条件编译
- **NFR-2**: 构建过程无错误，所有测试通过
- **NFR-3**: 性能不劣于当前实现
- **NFR-4**: 代码文档完整，符合项目规范

## Constraints
- **Technical**: 必须保持 API 兼容性，确保现有代码无需修改
- **Dependencies**: 移除对 fluent-bundle、unic-langid 等外部依赖

## Assumptions
- OakFluentTranslator 功能完整，能够满足所有国际化需求
- 现有代码主要使用全局 I18N 实例，不会直接依赖 FluentTranslator

## Acceptance Criteria

### AC-1: FluentTranslator 已被完全移除
- **Given**: 代码库中存在 FluentTranslator 实现
- **When**: 执行移除操作
- **Then**: 代码库中不再包含 FluentTranslator 结构体及其实现
- **Verification**: `programmatic`

### AC-2: 外部依赖已被移除
- **Given**: Cargo.toml 中包含 fluent-bundle、unic-langid 等依赖
- **When**: 执行依赖清理
- **Then**: Cargo.toml 中不再包含这些依赖
- **Verification**: `programmatic`

### AC-3: OakFluentTranslator 成为默认实现
- **Given**: 全局 I18N 实例使用条件编译选择翻译提供者
- **When**: 移除条件编译，直接使用 OakFluentTranslator
- **Then**: 全局 I18N 实例始终使用 OakFluentTranslator
- **Verification**: `programmatic`

### AC-4: API 兼容性保持不变
- **Given**: 现有代码使用国际化 API
- **When**: 移除 FluentTranslator 后
- **Then**: 现有代码无需修改即可正常工作
- **Verification**: `programmatic`

### AC-5: 所有测试通过
- **Given**: 项目包含国际化相关测试
- **When**: 运行测试套件
- **Then**: 所有测试通过
- **Verification**: `programmatic`

## Open Questions
- [ ] 是否需要更新项目文档，说明移除 FluentTranslator 的原因
- [ ] 是否需要在发布说明中提及此变更