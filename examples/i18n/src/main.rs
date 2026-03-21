//! 国际化示例
//!
//! 展示如何使用 ctmd 的国际化功能

use ctmd::{init_cli, t, t_with_args};
use ctmd_macros::Command;

/// 重导出 Parse trait，使宏能够正确引用
pub use ctmd::Parse;

/// 命令行工具
#[Command]
#[derive(Debug)]
pub struct Cli {
    /// 命令
    pub command: Command,
}

/// 命令枚举
#[Command]
#[derive(Debug)]
pub enum Command {
    /// 问候命令
    Greet {
        /// 名字
        name: String,
    },
    /// 帮助命令
    Help,
}

fn main() {
    // 初始化 CLI 环境
    init_cli();

    // 获取命令行参数
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    // 解析命令行参数
    let cli = Cli::parse(&mut args);

    // 执行命令
    match cli.command {
        Command::Greet { name } => {
            // 使用国际化函数
            println!("{}", t_with_args("greet", &[("name", &name)]));
            println!("{}", t("welcome"));
        }
        Command::Help => {
            println!("{}", t("help"));
            println!("{}", t("usage"));
        }
    }
}
