use criterion::{Criterion, black_box, criterion_group, criterion_main};
use figment::{
    Figment,
    providers::{Format, Json, Toml, Yaml},
};
use itools_config::ConfigManager;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tempfile::NamedTempFile;

#[derive(Debug, Deserialize, Serialize)]
struct TestConfig {
    app: AppConfig,
    database: DatabaseConfig,
    server: ServerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct AppConfig {
    name: String,
    version: String,
    debug: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
    timeout: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: u32,
}

fn create_test_configs() -> (String, String, String) {
    let json_config = r#"
{
  "app": {
    "name": "test-app",
    "version": "1.0.0",
    "debug": true
  },
  "database": {
    "url": "postgres://localhost:5432/test",
    "max_connections": 10,
    "timeout": 30
  },
  "server": {
    "host": "0.0.0.0",
    "port": 8080,
    "workers": 4
  }
}
"#;

    let toml_config = r#"
[app]
name = "test-app"
version = "1.0.0"
debug = true

[database]
url = "postgres://localhost:5432/test"
max_connections = 10
timeout = 30

[server]
host = "0.0.0.0"
port = 8080
workers = 4
"#;

    let yaml_config = r#"
app:
  name: test-app
  version: "1.0.0"
  debug: true
database:
  url: postgres://localhost:5432/test
  max_connections: 10
  timeout: 30
server:
  host: 0.0.0.0
  port: 8080
  workers: 4
"#;

    (json_config.to_string(), toml_config.to_string(), yaml_config.to_string())
}

fn benchmark_merge(c: &mut Criterion) {
    let (json_config, toml_config, yaml_config) = create_test_configs();

    // 创建带有扩展名的临时文件
    let json_file = NamedTempFile::with_suffix(".json").unwrap();
    let toml_file = NamedTempFile::with_suffix(".toml").unwrap();
    let yaml_file = NamedTempFile::with_suffix(".yaml").unwrap();

    // 写入临时文件
    std::fs::write(json_file.path(), json_config).unwrap();
    std::fs::write(toml_file.path(), toml_config).unwrap();
    std::fs::write(yaml_file.path(), yaml_config).unwrap();

    let json_path = json_file.path();
    let toml_path = toml_file.path();
    let yaml_path = yaml_file.path();

    let mut group = c.benchmark_group("merge");

    // 基准测试 itools-config
    group.bench_function("itools_merge", |b| {
        b.iter(|| {
            let mut config_manager = ConfigManager::new();
            config_manager.load_fragment(json_path).unwrap();
            config_manager.load_fragment(toml_path).unwrap();
            config_manager.load_fragment(yaml_path).unwrap();
            black_box(config_manager.get_config());
        });
    });

    // 基准测试 figment
    group.bench_function("figment_merge", |b| {
        b.iter(|| {
            let figment = Figment::new().merge(Json::file(json_path)).merge(Toml::file(toml_path)).merge(Yaml::file(yaml_path));
            let config: TestConfig = figment.extract().unwrap();
            black_box(config);
        });
    });

    group.finish();

    // 临时文件会在作用域结束时自动删除
}

criterion_group!(benches, benchmark_merge);
criterion_main!(benches);
