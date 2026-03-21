/// 国际化功能测试
use super::{t, t_with_args};

/// 测试复数形式翻译
pub fn test_plural_forms() {
    println!("=== 测试复数形式翻译 ===");

    // 英文测试
    let en_count_1 = t_with_args("messages.item_count", &[("count", "1")]);
    let en_count_2 = t_with_args("messages.item_count", &[("count", "2")]);
    println!("英文 - 1 个项目: {}", en_count_1);
    println!("英文 - 2 个项目: {}", en_count_2);

    // 中文测试
    let zh_count_1 = t_with_args("messages.item_count", &[("count", "1")]);
    let zh_count_2 = t_with_args("messages.item_count", &[("count", "2")]);
    println!("中文 - 1 个项目: {}", zh_count_1);
    println!("中文 - 2 个项目: {}", zh_count_2);
}

/// 测试性别示例翻译
pub fn test_gender_forms() {
    println!("=== 测试性别示例翻译 ===");

    // 英文测试
    let en_male = t_with_args("messages.greeting", &[("gender", "male"), ("name", "John")]);
    let en_female = t_with_args("messages.greeting", &[("gender", "female"), ("name", "Jane")]);
    let en_other = t_with_args("messages.greeting", &[("gender", "other"), ("name", "Alex")]);
    println!("英文 - 男性: {}", en_male);
    println!("英文 - 女性: {}", en_female);
    println!("英文 - 其他: {}", en_other);

    // 中文测试
    let zh_male = t_with_args("messages.greeting", &[("gender", "male"), ("name", "张三")]);
    let zh_female = t_with_args("messages.greeting", &[("gender", "female"), ("name", "李四")]);
    let zh_other = t_with_args("messages.greeting", &[("gender", "other"), ("name", "王五")]);
    println!("中文 - 男性: {}", zh_male);
    println!("中文 - 女性: {}", zh_female);
    println!("中文 - 其他: {}", zh_other);
}

/// 测试基本翻译
pub fn test_basic_translation() {
    println!("=== 测试基本翻译 ===");

    // 测试英文翻译
    println!("英文 - CLI 初始化: {}", t("messages.cli_initialized"));
    println!("英文 - 构建项目: {}", t("messages.building_project"));

    // 测试带参数的翻译
    println!("英文 - 初始化项目: {}", t_with_args("messages.initializing_project", &[("name", "test-project")]));
    println!("英文 - 未知命令: {}", t_with_args("errors.unknown_command", &[("command", "test")]));
}
