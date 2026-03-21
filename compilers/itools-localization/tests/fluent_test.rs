use oak_fluent::{
    Translator,
    ast::{FluentMessage, FluentPattern, FluentPatternElement, FluentRoot},
};

#[test]
fn test_oak_fluent_translator() {
    // 手动创建 FluentRoot 实例
    let root = FluentRoot {
        messages: vec![FluentMessage {
            id: "hello".to_string(),
            value: Some(FluentPattern { elements: vec![FluentPatternElement::Text("Hello world!".to_string())] }),
            attributes: vec![],
            variants: vec![],
        }],
    };

    // 创建翻译器
    let translator = Translator::new(root);

    // 测试翻译
    let args = std::collections::HashMap::new();
    match translator.translate("hello", &args) {
        Some(translation) => {
            println!("Translation: {}", translation);
            assert_eq!(translation, "Hello world!");
        }
        None => {
            panic!("Failed to translate 'hello'");
        }
    }
}

#[test]
fn test_oak_fluent_version() {
    println!("Oak Fluent version: {}", env!("CARGO_PKG_VERSION"));
}
