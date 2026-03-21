# iTools Core

## 概述

iTools Core 是 iTools 项目的核心功能模块，提供可本地化错误类型、工具函数和通用组件。

## 功能特性

- 可本地化的错误类型
- 通用错误处理工具
- 与 iTools 本地化功能集成
- 简单易用的错误宏
- 支持错误来源跟踪

## 安装

```bash
# 从源码构建
cargo build --package itools-core
```

## 使用方法

### 创建本地化错误

```rust
use itools_core::{LocalizedError, Result, localizable_error};

// 创建简单的本地化错误
let error = LocalizedError::simple("error.key");

// 创建带参数的错误
let error = LocalizedError::new("error.with_args", vec![("param", "value")], None);

// 使用错误宏
localizable_error! {
    MyError {
        InvalidInput => "error.invalid_input",
        FileNotFound(String) => "error.file_not_found",
    }
}

fn main() -> Result<()> {
    Ok(())
}
```

## 开发指南

### 项目结构

```
itools-core/
├── src/
│   └── lib.rs          # 库入口
├── Cargo.toml          # 项目配置
└── readme.md           # 项目文档
```

## 国际化

Core 模块支持多语言，翻译资源与 iTools 核心共享。

### 支持的语言
- 英文 (`en`)
- 中文 (`zh-CN`)

## 故障排除

## 贡献

欢迎提交 Issue 和 Pull Request 来改进 iTools Core。

## 许可证

AGPL-3.0
