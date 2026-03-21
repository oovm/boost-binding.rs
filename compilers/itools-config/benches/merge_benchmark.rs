use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itools_config::ConfigManager;
use figment::{Figment, providers::{Json, Toml, Yaml, Format}};
use serde::{Deserialize, Serialize};
use std::path::Path;

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

    // 写入临时文件
    std::fs::write("test_config.json", json_config).unwrap();
    std::fs::write("test_config.toml", toml_config).unwrap();
    std::fs::write("test_config.yaml", yaml_config).unwrap();

    let mut group = c.benchmark_group("merge");

    // 基准测试 itools-config
    group.bench_function("itools_config_merge", |b| {
        b.iter(|| {
            let mut config_manager = ConfigManager::new();
            config_manager.load_fragment(Path::new("test_config.json")).unwrap();
            config_manager.load_fragment(Path::new("test_config.toml")).unwrap();
            config_manager.load_fragment(Path::new("test_config.yaml")).unwrap();
            black_box(config_manager.get_config());
        });
    });

    // 基准测试 figment
    group.bench_function("figment_merge", |b| {
        b.iter(|| {
            let figment = Figment::new()
                .merge(Json::file("test_config.json"))
                .merge(Toml::file("test_config.toml"))
                .merge(Yaml::file("test_config.yaml"));
            let config: TestConfig = figment.extract().unwrap();
            black_box(config);
        });
    });

    group.finish();

    // 清理临时文件
    std::fs::remove_file("test_config.json").unwrap();
    std::fs::remove_file("test_config.toml").unwrap();
    std::fs::remove_file("test_config.yaml").unwrap();
}

criterion_group!(benches, benchmark_merge);
criterion_main!(benches);
