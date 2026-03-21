//! 配置解析测试

use ctmd_config::ConfigManager;
use std::{fs, path::Path};

#[test]
fn test_config_manager() {
    // 创建临时配置文件
    let temp_dir = tempfile::tempdir().unwrap();
    let config_dir = temp_dir.path().join("config");
    fs::create_dir_all(&config_dir).unwrap();

    // 创建测试配置文件
    let config1 = config_dir.join("config1.json");
    fs::write(&config1, r#"{"app": {"name": "ctmd", "version": "1.0.0"}}"#).unwrap();

    let config2 = config_dir.join("config2.toml");
    fs::write(
        &config2,
        r#"[app]
env = "development"
"#,
    )
    .unwrap();

    // 创建配置管理器
    let mut config_manager = ConfigManager::new();

    // 加载配置目录
    config_manager.load_directory(&config_dir).unwrap();

    // 验证配置
    let config = config_manager.get_config();
    assert_eq!(config["app"]["name"], "ctmd");
    assert_eq!(config["app"]["version"], "1.0.0");
    assert_eq!(config["app"]["env"], "development");
}

#[test]
fn test_config_override() {
    // 创建配置管理器
    let mut config_manager = ConfigManager::new();

    // 加载默认配置
    let temp_dir = tempfile::tempdir().unwrap();
    let config_dir = temp_dir.path().join("config");
    fs::create_dir_all(&config_dir).unwrap();

    let config_file = config_dir.join("default.json");
    fs::write(&config_file, r#"{"app": {"name": "ctmd", "port": 3000}}"#).unwrap();

    config_manager.load_directory(&config_dir).unwrap();

    // 测试命令行参数覆盖
    let args = vec!["ctmd", "--config.app.port=8080", "--config.app.name=ctmd-dev"];
    config_manager.override_from_args(&args);

    let config = config_manager.get_config();
    assert_eq!(config["app"]["name"], "ctmd-dev");
    assert_eq!(config["app"]["port"], 8080);
}

#[test]
fn test_config_validation() {
    // 创建配置管理器
    let mut config_manager = ConfigManager::new();

    // 加载有效配置
    let temp_dir = tempfile::tempdir().unwrap();
    let config_dir = temp_dir.path().join("config");
    fs::create_dir_all(&config_dir).unwrap();

    let config_file = config_dir.join("valid.json");
    fs::write(&config_file, r#"{"app": {"name": "ctmd"}}"#).unwrap();

    config_manager.load_directory(&config_dir).unwrap();
    assert!(config_manager.validate_merged_config().is_ok());
}
