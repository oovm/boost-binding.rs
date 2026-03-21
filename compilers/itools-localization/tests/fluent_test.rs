use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use unic_langid::langid;

#[test]
fn test_fluent_bundle_parse() {
    // 测试基本 FTL 解析
    let ftl = "hello = Hello world!";

    match FluentResource::try_new(ftl.to_string()) {
        Ok(resource) => {
            println!("Parsed successfully: {:?}", resource);
        }
        Err((_, errors)) => {
            println!("Parse error: {:?}", errors);
            panic!("Failed to parse FTL content: {:?}", errors);
        }
    }
}

#[test]
fn test_fluent_bundle_version() {
    println!("Fluent bundle version: {}", env!("CARGO_PKG_VERSION"));
}
