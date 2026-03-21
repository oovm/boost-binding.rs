# iTools Config

🎉 iTools Config 是 iTools 项目的配置管理模块，用于处理编译器的配置信息。

## 特性

- ✨ 提供配置文件的解析和管理
- ✨ 支持配置项的验证和默认值设置
- ✨ 与其他 iTools 组件集成

## 使用方法

```rust
// 示例代码
use itools_config::load_config;

fn main() {
    let config = load_config("config.toml").expect("加载配置失败");
    println!("配置: {:?}", config);
}
```

## 注意事项

- 配置文件格式支持 TOML、JSON 和 YAML
- 请确保配置文件路径正确
- 部分配置项有默认值，可根据需要覆盖