#![warn(missing_docs)]

//! iTools 命令行接口模块
//! 
//! 提供命令行解析、命令执行和相关功能。

use std::error::Error;
use tracing::info;

/// 命令行参数解析 trait
pub trait Parse {
    /// 从命令行参数中解析当前类型
    fn parse(args: &mut Vec<String>) -> Self;
}

/// 为 String 实现 Parse trait
impl Parse for String {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("Expected string argument");
        }
        args.remove(0)
    }
}

/// 为 i32 实现 Parse trait
impl Parse for i32 {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("Expected integer argument");
        }
        args.remove(0).parse().expect("Invalid integer")
    }
}

/// 为 u32 实现 Parse trait
impl Parse for u32 {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("Expected unsigned integer argument");
        }
        args.remove(0).parse().expect("Invalid unsigned integer")
    }
}

/// 为 bool 实现 Parse trait
impl Parse for bool {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("Expected boolean argument");
        }
        args.remove(0).parse().expect("Invalid boolean")
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
            panic!("Expected command");
        }
        let cmd = args.remove(0);
        match cmd.as_str() {
            "init" => Self::Init { name: String::parse(args) },
            "build" => Self::Build,
            "dev" => Self::Dev,
            "check" => Self::Check,
            _ => panic!("Unknown command: {}", cmd),
        }
    }
}

/// 初始化 CLI 环境
pub fn init_cli() {
    // 初始化日志
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();

    info!("CLI initialized");
}

/// 执行命令
pub fn execute_command(cli: Cli) -> Result<(), Box<dyn Error>> {
    match cli.command {
        Command::Init { name } => {
            info!("Initializing project: {}", name);
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
            info!("Building project");
            // 测试加载动画
            itools_tui::utils::loading_animation(std::time::Duration::from_secs(3), "Building...");
            Ok(())
        }
        Command::Dev => {
            info!("Starting dev server");
            // 测试交互式选择菜单
            let items = ["Option 1", "Option 2", "Option 3"];
            if let Some(choice) = itools_tui::components::select(&items, "Please select an option:") {
                info!("Selected: {}", items[choice]);
            }
            Ok(())
        }
        Command::Check => {
            info!("Checking project");
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
