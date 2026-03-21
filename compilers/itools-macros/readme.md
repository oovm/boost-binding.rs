# ctmd-macros

ctmd-macros 是 ctmd 项目的宏定义模块，提供各种 procmacro 支持。

## 功能

- 提供过程宏定义
- 支持编译时代码生成
- 与其他 ctmd 组件集成

## 用法

```rust
// 示例代码
use ctmd_macros::ctmd;

#[ctmd]
fn example() {
    println!("Hello, ctmd!");
}
```
