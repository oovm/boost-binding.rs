/// 国际化功能集成测试
use itools::{t, t_with_args};

#[test]
fn test_basic_translation() {
    // 测试基本翻译
    assert_eq!(t("messages.cli_initialized"), "iTools CLI initialized");
    assert_eq!(t("messages.building_project"), "Building project");
}

#[test]
fn test_translation_with_args() {
    // 测试带参数的翻译
    let result = t_with_args("messages.initializing_project", &[("name", "test-project")]);
    assert_eq!(result, "Initializing project: test-project");

    let result = t_with_args("errors.unknown_command", &[("command", "test")]);
    assert_eq!(result, "Unknown command: test");
}

#[test]
fn test_complex_forms() {
    // 测试复数形式
    let result1 = t_with_args("messages.item_count", &[("count", "1")]);
    assert_eq!(result1, "There is 1 item");

    let result2 = t_with_args("messages.item_count", &[("count", "2")]);
    assert_eq!(result2, "There are 2 items");

    // 测试性别形式
    let result_male = t_with_args("messages.greeting", &[("gender", "male"), ("name", "John")]);
    assert_eq!(result_male, "Hello, Mr. John!");

    let result_female = t_with_args("messages.greeting", &[("gender", "female"), ("name", "Jane")]);
    assert_eq!(result_female, "Hello, Ms. Jane!");

    let result_other = t_with_args("messages.greeting", &[("gender", "other"), ("name", "Alex")]);
    assert_eq!(result_other, "Hello, Alex!");
}

#[test]
fn test_fallback_mechanism() {
    // 测试回退机制 - 当键不存在时应返回键本身
    let result = t("non.existent.key");
    assert_eq!(result, "non.existent.key");
}
