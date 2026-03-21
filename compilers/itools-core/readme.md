# iTools Core

🚀 iTools Core 是 iTools 项目的核心功能模块，提供可本地化错误类型、工具函数和通用组件。

## 特性

- ✨ 可本地化的错误类型
- ✨ 通用错误处理工具
- ✨ 与 iTools 本地化功能集成
- ✨ 简单易用的错误宏
- ✨ 支持错误来源跟踪

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

## 注意事项

- 错误消息需要在本地化资源文件中定义
- 使用错误宏时，请确保错误类型名称符合 Rust 命名规范
- 错误参数需要与本地化消息中的占位符匹配