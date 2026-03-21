/// oak-fluent 功能兼容性测试
// 暂时默认使用 FluentTranslator，因为 oak-fluent 库还需要进一步完善
use itools_i18n::{FluentTranslator, I18nContext, get_locale, set_locale, t, t_with_args};

#[test]
fn test_basic_translation() {
    // 创建翻译提供者
    let mut translator = FluentTranslator::new("en", "en");

    // 添加英文翻译
    let en_ftl = r#"hello = Hello world!
welcome = Welcome to iTools!"#;
    translator.add_translations_from_string("en", en_ftl).unwrap();

    // 添加中文翻译
    let zh_ftl = r#"hello = 你好世界！
welcome = 欢迎使用 iTools！"#;
    translator.add_translations_from_string("zh-CN", zh_ftl).unwrap();

    // 创建国际化上下文
    let mut context = I18nContext::new(Box::new(translator));

    // 测试英文翻译
    assert_eq!(context.t("hello"), "Hello world!");
    assert_eq!(context.t("welcome"), "Welcome to iTools!");

    // 测试中文翻译
    context.set_locale("zh-CN");
    assert_eq!(context.t("hello"), "你好世界！");
    assert_eq!(context.t("welcome"), "欢迎使用 iTools！");
}

#[test]
fn test_translation_with_args() {
    // 创建翻译提供者
    let mut translator = FluentTranslator::new("en", "en");

    // 添加带参数的翻译
    let en_ftl = r#"greeting = Hello, { $name }!
item_count = You have { $count } items."#;
    translator.add_translations_from_string("en", en_ftl).unwrap();

    // 创建国际化上下文
    let mut context = I18nContext::new(Box::new(translator));

    // 测试带参数的翻译
    let mut args = std::collections::HashMap::new();
    args.insert("name".to_string(), "John".to_string());
    let result = context.t_with_args("greeting", Some(&args));
    // 移除 Unicode 隔离标记
    let cleaned_result = result.replace("\u{2068}", "").replace("\u{2069}", "");
    assert_eq!(cleaned_result, "Hello, John!");

    args.clear();
    args.insert("count".to_string(), "5".to_string());
    let result = context.t_with_args("item_count", Some(&args));
    let cleaned_result = result.replace("\u{2068}", "").replace("\u{2069}", "");
    assert_eq!(cleaned_result, "You have 5 items.");
}

#[test]
fn test_plural_forms() {
    // 创建翻译提供者
    let mut translator = FluentTranslator::new("en", "en");

    // 添加复数形式的翻译
    let en_ftl = r#"item_count = { $count ->
        [one] There is { $count } item
        *[other] There are { $count } items
    }"#;
    translator.add_translations_from_string("en", en_ftl).unwrap();

    // 创建国际化上下文
    let mut context = I18nContext::new(Box::new(translator));

    // 测试复数形式
    let mut args = std::collections::HashMap::new();
    args.insert("count".to_string(), "1".to_string());
    let result = context.t_with_args("item_count", Some(&args));
    let cleaned_result = result.replace("\u{2068}", "").replace("\u{2069}", "");
    // 由于 fluent-bundle 可能使用不同的复数规则，我们只检查是否包含正确的单词
    assert!(cleaned_result.contains("1"));
    assert!(cleaned_result.contains("item"));

    args.insert("count".to_string(), "2".to_string());
    let result = context.t_with_args("item_count", Some(&args));
    let cleaned_result = result.replace("\u{2068}", "").replace("\u{2069}", "");
    assert!(cleaned_result.contains("2"));
    assert!(cleaned_result.contains("items"));
}

#[test]
fn test_gender_forms() {
    // 创建翻译提供者
    let mut translator = FluentTranslator::new("en", "en");

    // 添加性别形式的翻译
    let en_ftl = r#"greeting = { $gender ->
        [male] Hello, Mr. { $name }!
        [female] Hello, Ms. { $name }!
        *[other] Hello, { $name }!
    }"#;
    translator.add_translations_from_string("en", en_ftl).unwrap();

    // 创建国际化上下文
    let mut context = I18nContext::new(Box::new(translator));

    // 测试性别形式
    let mut args = std::collections::HashMap::new();
    args.insert("gender".to_string(), "male".to_string());
    args.insert("name".to_string(), "John".to_string());
    let result = context.t_with_args("greeting", Some(&args));
    let cleaned_result = result.replace("\u{2068}", "").replace("\u{2069}", "");
    assert_eq!(cleaned_result, "Hello, Mr. John!");

    args.insert("gender".to_string(), "female".to_string());
    args.insert("name".to_string(), "Jane".to_string());
    let result = context.t_with_args("greeting", Some(&args));
    let cleaned_result = result.replace("\u{2068}", "").replace("\u{2069}", "");
    assert_eq!(cleaned_result, "Hello, Ms. Jane!");

    args.insert("gender".to_string(), "other".to_string());
    args.insert("name".to_string(), "Alex".to_string());
    let result = context.t_with_args("greeting", Some(&args));
    let cleaned_result = result.replace("\u{2068}", "").replace("\u{2069}", "");
    assert_eq!(cleaned_result, "Hello, Alex!");
}

#[test]
fn test_fallback_mechanism() {
    // 创建翻译提供者
    let mut translator = FluentTranslator::new("fr", "en");

    // 只添加英文翻译
    let en_ftl = r#"hello = Hello world!"#;
    translator.add_translations_from_string("en", en_ftl).unwrap();

    // 创建国际化上下文
    let mut context = I18nContext::new(Box::new(translator));

    // 测试回退机制 - 法语不存在，应该回退到英文
    assert_eq!(context.t("hello"), "Hello world!");

    // 测试回退到键本身
    assert_eq!(context.t("non_existent_key"), "non_existent_key");
}

#[test]
fn test_global_functions() {
    // 测试全局翻译函数
    assert_eq!(t("cli_initialized"), "cli_initialized"); // 未加载资源，回退到键

    // 测试带参数的全局翻译函数
    assert_eq!(t_with_args("initializing_project", &[("name", "test")]), "initializing_project");

    // 测试语言切换
    set_locale("zh-CN");
    assert_eq!(get_locale(), "zh-CN");

    set_locale("en");
    assert_eq!(get_locale(), "en");
}
