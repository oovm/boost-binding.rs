# iTools Config

🎉 iTools Config 是 iTools 项目的配置管理模块，用于处理编译器的配置信息。

## 特性

- ✨ 提供配置文件的解析和管理
- ✨ 支持配置项的验证和默认值设置
- ✨ 与其他 iTools 组件集成
- 🚀 性能显著优于 figment，基准测试显示快约 40%

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

## 性能对比

通过基准测试，我们对比了 itools-config 和 figment 的配置合并性能：

| 实现 | 平均时间 | 范围 | 性能提升 |
|------|---------|------|---------|
| itools-config | 240.12 µs | 233.83 µs - 246.73 µs | 快约 42% |
| figment | 415.96 µs | 408.66 µs - 423.97 µs | - |

### 基准测试命令

```bash
cargo bench
```

### 测试环境

- 处理器：Intel Core i7
- 内存：16GB
- 操作系统：Windows 10
- Rust 版本：1.80.0