#![warn(missing_docs)]

//! iTools 命令行接口模块
//! 
//! 提供命令行解析、命令执行和相关功能。

use std::error::Error;
use tracing::info;

/// 国际化功能 - 使用 itools-localization 模块的国际化实现
pub use itools_localization::{t, t_with_args};

/// 命令行参数解析 trait
pub trait Parse {
    /// 从命令行参数中解析当前类型
    fn parse(args: &mut Vec<String>) -> Self;
}

/// 为 String 实现 Parse trait
impl Parse for String {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t("errors.expected_string_argument"));
        }
        args.remove(0)
    }
}

/// 为 i32 实现 Parse trait
impl Parse for i32 {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t("errors.expected_integer_argument"));
        }
        args.remove(0).parse().expect(&t("errors.invalid_integer"))
    }
}

/// 为 u32 实现 Parse trait
impl Parse for u32 {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t("errors.expected_unsigned_integer_argument"));
        }
        args.remove(0).parse().expect(&t("errors.invalid_unsigned_integer"))
    }
}

/// 为 bool 实现 Parse trait
impl Parse for bool {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t("errors.expected_boolean_argument"));
        }
        args.remove(0).parse().expect(&t("errors.invalid_boolean"))
    }
}

/// iTools 命令行工具
#[derive(Debug)]
pub struct Cli {
    /// 命令
    pub command: Command,
}

/// 命令枚举
#[derive(Debug)]
pub enum Command {
    /// 初始化新项目
    Init {
        /// 项目名称
        name: String,
    },
    /// 构建项目
    Build,
    /// 开发服务器
    Dev,
    /// 检查项目
    Check,
}

/// 为 Cli 实现 Parse trait
impl Parse for Cli {
    fn parse(args: &mut Vec<String>) -> Self {
        Self { command: Command::parse(args) }
    }
}

/// 为 Command 实现 Parse trait
impl Parse for Command {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t("errors.expected_command"));
        }
        let cmd = args.remove(0);
        match cmd.as_str() {
            "init" => Self::Init { name: String::parse(args) },
            "build" => Self::Build,
            "dev" => Self::Dev,
            "check" => Self::Check,
            _ => panic!("{}", t_with_args("errors.unknown_command", &[("command", &cmd)])),
        }
    }
}

/// 初始化 CLI 环境
pub fn init_cli() {
    // 初始化日志
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();

    info!("{}", t("messages.cli_initialized"));
}

/// 执行命令
pub fn execute_command(cli: Cli) -> Result<(), Box<dyn Error>> {
    match cli.command {
        Command::Init { name } => {
            info!("{}", t_with_args("messages.initializing_project", &[("name", &name)]));
            // 测试进度条
            let mut pb = itools_tui::components::ProgressBar::new(100, 50);
            for i in 0..=100 {
                pb.update(i);
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            pb.finish();
            Ok(())
        }
        Command::Build => {
            info!("{}", t("messages.building_project"));
            // 测试加载动画
            itools_tui::utils::loading_animation(std::time::Duration::from_secs(3), "构建中...");
            Ok(())
        }
        Command::Dev => {
            info!("{}", t("messages.starting_dev_server"));
            // 测试交互式选择菜单
            let items = ["选项 1", "选项 2", "选项 3"];
            if let Some(choice) = itools_tui::components::select(&items, "请选择一个选项:") {
                info!("选择了: {}", items[choice]);
            }
            Ok(())
        }
        Command::Check => {
            info!("{}", t("messages.checking_project"));
            // 测试进度条
            let mut pb = itools_tui::components::ProgressBar::new(50, 30);
            for i in 0..=50 {
                pb.update(i);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            pb.finish();
            Ok(())
        }
    }
}
