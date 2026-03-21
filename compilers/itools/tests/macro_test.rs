/// 测试翻译宏
///
/// 验证 t! 宏的功能
use itools::t;

#[test]
fn test_translation_macros() {
    // 测试基本翻译
    let hello = t!("hello");
    assert_eq!(hello, "hello"); // 未加载资源，回退到键

    // 测试带参数的翻译
    let greeting = t!("greeting", &[("name", "World")]);
    assert_eq!(greeting, "greeting"); // 未加载资源，回退到键
}
