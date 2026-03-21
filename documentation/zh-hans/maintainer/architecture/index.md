# iTools 架构设计文档

## 1. 项目整体架构说明

iTools 是一个命令行工具框架，采用 Rust 编写，采用模块化设计，各模块职责清晰，耦合度低。项目整体分为以下几个主要部分：

- 核心库 (`itools-core`)：负责核心业务逻辑
- 命令行工具 (`itools`)：提供命令行交互接口
- 配置管理 (`itools-config`)：处理配置相关功能
- 宏定义 (`itools-macros`)：提供命令行参数解析的宏
- 工具函数 (`itools-tools`)：提供通用工具函数
- 前端应用 (`frontends`)：提供 TypeScript 版本和网页界面

### 技术栈

- **后端**：Rust
- **前端**：TypeScript + Vue.js
- **异步运行时**：tokio
- **国际化**：内置国际化支持

## 2. 模块划分与职责说明

### 2.1 itools-core - 核心业务模块

**职责**：
- 提供核心功能实现
- 支持命令行参数解析
- 处理国际化

**核心文件**：
- `compilers/itools-core/src/lib.rs` - 模块导出和公共 API

### 2.2 itools - 命令行工具

**职责**：
- 提供命令行接口
- 解析命令行参数
- 调用核心功能执行任务

**核心文件**：
- `compilers/itools/bin/itools.rs` - 程序入口点
- `compilers/itools/src/lib.rs` - 命令行工具库

### 2.3 itools-config - 配置管理

**职责**：
- 管理项目配置
- 处理配置文件
- 提供配置相关的功能

**核心文件**：
- `compilers/itools-config/src/lib.rs` - 配置管理库
- `compilers/itools-config/src/main.rs` - 配置管理工具

### 2.4 itools-macros - 宏定义

**职责**：
- 提供命令行参数解析的宏
- 简化命令行工具的开发

**核心文件**：
- `compilers/itools-macros/src/lib.rs` - 宏定义

### 2.5 itools-tools - 工具函数

**职责**：
- 提供通用工具函数
- 支持其他模块的功能实现

**核心文件**：
- `compilers/itools-tools/src/lib.rs` - 工具函数库

### 2.6 frontends - 前端应用

**职责**：
- 提供 TypeScript 版本的 itools
- 提供项目主页

**核心文件**：
- `frontends/itools/src/index.ts` - TypeScript 版本入口
- `frontends/homepage/src/App.vue` - 项目主页应用

## 3. 数据流说明

### 3.1 整体流程

命令行工具的执行流程如下：

```
命令行参数 → 解析参数 → 执行命令 → 返回结果
```

### 3.2 详细步骤

#### 步骤 1: 解析命令行参数

1. 从命令行获取参数 (`compilers/itools/bin/itools.rs`)
2. 使用 `#[Command]` 宏解析参数
3. 生成对应的命令对象

#### 步骤 2: 执行命令

1. 根据解析出的命令类型，执行相应的处理逻辑
2. 调用核心库的功能
3. 处理命令执行结果

#### 步骤 3: 返回结果

1. 将执行结果输出到控制台
2. 处理错误情况
3. 返回适当的退出码

## 4. 核心代码位置引用

### 命令行工具
- `itools::main` - `compilers/itools/bin/itools.rs`
- `itools::lib` - `compilers/itools/src/lib.rs`

### 配置管理
- `itools_config::lib` - `compilers/itools-config/src/lib.rs`

### 宏定义
- `itools_macros::Command` - `compilers/itools-macros/src/lib.rs`

### 前端应用
- TypeScript 版本 - `frontends/itools/src/index.ts`
- 项目主页 - `frontends/homepage/src/App.vue`

## 5. 扩展开发指南

### 添加新的命令

1. 在 `compilers/itools/src/lib.rs` 中定义新的命令结构
2. 使用 `#[Command]` 宏标记
3. 实现命令的处理逻辑
4. 在主函数中添加命令的处理分支

### 添加新的功能模块

1. 在 `compilers/` 目录下创建新的模块目录
2. 实现模块的核心功能
3. 在 `Cargo.toml` 中添加依赖
4. 在需要的地方引用该模块

### 添加国际化支持

1. 在 `compilers/itools/locales/` 目录下添加语言文件
2. 在代码中使用 `lang!` 宏进行国际化
3. 确保所有用户可见的字符串都进行了国际化处理

## 6. 示例应用

项目提供了多个示例应用，展示如何使用 itools 框架：

- `examples/hello/` - 简单的 hello world 示例
- `examples/full-app/` - 完整的应用示例
- `examples/i18n/` - 国际化示例
- `examples/interactive/` - 交互式示例
- `examples/shared-dylib/` - 共享库示例
- `examples/shared-plugin/` - 共享插件示例

这些示例可以作为开发新应用的参考。