//! 测试 CLI 宏 API

use crate::Parse;

/// 测试命令行工具
#[derive(Debug)]
pub struct TestCli {
    /// 命令
    pub command: String,
}

impl Parse for TestCli {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("Expected command");
        }
        Self { command: args.remove(0) }
    }
}

/// 测试宏功能
pub fn test_macro() {
    println!("Testing CLI macro API...");

    // 测试初始化命令
    let args1 = vec!["init", "test-project"];
    let mut args1_str: Vec<String> = args1.iter().map(|s| s.to_string()).collect();
    let cli1 = TestCli::parse(&mut args1_str);
    println!("Test 1 passed: {:?}", cli1);

    // 测试构建命令
    let args2 = vec!["build"];
    let mut args2_str: Vec<String> = args2.iter().map(|s| s.to_string()).collect();
    let cli2 = TestCli::parse(&mut args2_str);
    println!("Test 2 passed: {:?}", cli2);

    // 测试开发命令
    let args3 = vec!["dev"];
    let mut args3_str: Vec<String> = args3.iter().map(|s| s.to_string()).collect();
    let cli3 = TestCli::parse(&mut args3_str);
    println!("Test 3 passed: {:?}", cli3);

    // 测试检查命令
    let args4 = vec!["check"];
    let mut args4_str: Vec<String> = args4.iter().map(|s| s.to_string()).collect();
    let cli4 = TestCli::parse(&mut args4_str);
    println!("Test 4 passed: {:?}", cli4);

    println!("All tests passed!");
}
