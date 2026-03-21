//! 基本命令行工具示例
//!
//! 展示如何使用 itools 的宏来解析命令行参数

use itools::{Parse, init_cli};
use itools_macros::Command;

/// 测试命令行工具
#[Command]
#[derive(Debug)]
pub struct TestCli {
    /// 命令
    pub command: TestCommand,
}

/// 测试命令枚举
#[Command]
#[derive(Debug)]
pub enum TestCommand {
    /// 问候命令
    Greet {
        /// 名字
        name: String,
        /// 年龄
        age: u32,
    },
    /// 计算命令
    Calculate {
        /// 第一个数字
        a: i32,
        /// 第二个数字
        b: i32,
    },
    /// 退出命令
    Exit,
}

fn main() {
    // 初始化 CLI 环境
    init_cli();

    // 获取命令行参数
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    // 解析命令行参数
    let cli = TestCli::parse(&mut args);

    // 执行命令
    match cli.command {
        TestCommand::Greet { name, age } => {
            println!("Hello, {}! You are {} years old.", name, age);
        }
        TestCommand::Calculate { a, b } => {
            println!("{} + {} = {}", a, b, a + b);
        }
        TestCommand::Exit => {
            println!("Exiting...");
        }
    }
}
