# ctmd-config

ctmd-config 是 ctmd 项目的配置管理模块，用于处理编译器的配置信息。

## 功能

- 提供配置文件的解析和管理
- 支持配置项的验证和默认值设置
- 与其他 ctmd 组件集成

## 依赖

- ctmd-macros

## 用法

```rust
// 示例代码
use ctmd_config::load_config;

fn main() {
    let config = load_config("config.toml").expect("加载配置失败");
    println!("配置: {:?}", config);
}
```
