# iTools Macros

🔧 iTools Macros 是 iTools 项目的宏定义模块，提供各种 proc macro 支持。

## 功能

- 提供命令行参数解析宏
- 提供错误处理宏
- 提供翻译宏 `t!`（支持重载）

## 翻译宏用法

### 基本翻译

```rust
use itools::t;

let greeting = t!("hello");
println!("{}", greeting);
```

### 带参数的翻译

```rust
use itools::t;

let greeting = t!("greeting", &[("name", "World")]);
println!("{}", greeting);
```

### 命令行参数解析宏

```rust
#[itools_macros::Command]
pub struct Cli {
    /// 命令
    pub command: Command,
}

#[itools_macros::Command]
pub enum Command {
    /// 初始化新项目
    Init {
        /// 项目名称
        name: String,
    },
    /// 构建项目
    Build,
}
```

### 错误处理宏

```rust
#[itools_macros::Error]
pub enum AppError {
    /// 网络错误
    #[error("network_error")]
    NetworkError,
    /// 文件未找到
    #[error("file_not_found")]
    FileNotFound { path: String },
}
```

## 注意事项

- 宏定义需要在 `#[proc_macro_attribute]` 标注的函数中使用
- 翻译宏依赖于 iTools 的本地化模块
- 命令行参数解析宏会自动生成相应的解析代码
- 错误处理宏会自动生成实现 `Error` trait 的代码