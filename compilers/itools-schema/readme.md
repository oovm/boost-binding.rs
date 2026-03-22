# ITools Schema

ITools Schema 是一个 Schema 定义与构造模块，提供结构化数据的定义、验证和序列化功能。

## 功能特性

- 基于 Serde 的类型定义和序列化支持
- 模块化设计，支持按需启用序列化格式
- 轻量级核心，无强制依赖
- 易于集成到各种项目中

## 安装

将以下依赖添加到你的 `Cargo.toml` 文件中：

### 基础安装（仅核心功能）

```toml
[dependencies]
itools-schema = "0.1.0"
```

### 启用 JSON 支持

```toml
[dependencies]
itools-schema = { version = "0.1.0", features = ["json"] }
```

### 启用 YAML 支持

```toml
[dependencies]
itools-schema = { version = "0.1.0", features = ["yaml"] }
```

### 同时启用 JSON 和 YAML 支持

```toml
[dependencies]
itools-schema = { version = "0.1.0", features = ["json", "yaml"] }
```

## 基本使用

### 示例 1: 基本 Schema 定义

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub settings: Settings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub timeout: u32,
    pub retries: u32,
    pub enabled: bool,
}
```

### 示例 2: 使用 JSON 格式 (需要启用 json 特性)

```rust
#[cfg(feature = "json")]
fn json_example() {
    use serde_json;

    let config = Config {
        name: "my-app".to_string(),
        version: "1.0.0".to_string(),
        description: Some("My application".to_string()),
        settings: Settings {
            timeout: 30,
            retries: 3,
            enabled: true,
        },
    };

    // 序列化到 JSON
    let json = serde_json::to_string_pretty(&config).unwrap();
    println!("{}", json);

    // 从 JSON 反序列化
    let json_str = r#"
    {
        "name": "my-app",
        "version": "1.0.0",
        "description": "My application",
        "settings": {
            "timeout": 30,
            "retries": 3,
            "enabled": true
        }
    }
    "#;

    let config_from_json: Config = serde_json::from_str(json_str).unwrap();
    println!("{:?}", config_from_json);
}
```

### 示例 3: 使用 YAML 格式 (需要启用 yaml 特性)

```rust
#[cfg(feature = "yaml")]
fn yaml_example() {
    use serde_yaml;

    let config = Config {
        name: "my-app".to_string(),
        version: "1.0.0".to_string(),
        description: Some("My application".to_string()),
        settings: Settings {
            timeout: 30,
            retries: 3,
            enabled: true,
        },
    };

    // 序列化到 YAML
    let yaml = serde_yaml::to_string(&config).unwrap();
    println!("{}", yaml);

    // 从 YAML 反序列化
    let yaml_str = r#"
name: my-app
version: "1.0.0"
description: My application
settings:
  timeout: 30
  retries: 3
  enabled: true
    "#;

    let config_from_yaml: Config = serde_yaml::from_str(yaml_str).unwrap();
    println!("{:?}", config_from_yaml);
}
```

## 特性

- `default`: 仅包含核心功能，无序列化依赖
- `json`: 启用 JSON 格式支持
- `yaml`: 启用 YAML 格式支持

## 许可证

MIT 许可证，详见 [LICENSE](LICENSE) 文件。

## 项目链接

- 主页: https://itools.dev
- 仓库: https://github.com/itools-dev/itools
