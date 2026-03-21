//! 完整的 CLI 应用示例
//! 
//! 展示如何构建一个包含多个命令的完整应用

use ctmd::init_cli;
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
    /// 初始化新项目
    Init {
        /// 项目名称
        name: String,
        /// 是否覆盖
        force: bool,
    },
    /// 构建项目
    Build {
        /// 构建目标
        target: String,
        /// 构建模式
        mode: String,
    },
    /// 开发服务器
    Dev {
        /// 端口
        port: u32,
    },
    /// 检查项目
    Check,
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
        Command::Init { name, force } => {
            println!("初始化项目: {} (force: {})\n", name, force);
            println!("正在创建项目结构...");
            // 模拟初始化过程
            for i in 1..=10 {
                println!("创建文件 {}...", i);
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
            println!("项目初始化完成！");
        }
        Command::Build { target, mode } => {
            println!("构建项目: target={}, mode={}\n", target, mode);
            println!("正在构建项目...");
            // 模拟构建过程
            for i in 1..=10 {
                println!("构建步骤 {}...", i);
                std::thread::sleep(std::time::Duration::from_millis(300));
            }
            println!("项目构建完成！");
        }
        Command::Dev { port } => {
            println!("启动开发服务器: port={}\n", port);
            println!("正在启动服务器...");
            std::thread::sleep(std::time::Duration::from_millis(1000));
            println!("服务器已启动，监听端口 {}！", port);
            println!("按 Ctrl+C 退出...");
            // 模拟服务器运行
            std::thread::sleep(std::time::Duration::from_millis(5000));
        }
        Command::Check => {
            println!("检查项目\n");
            println!("正在检查项目...");
            // 模拟检查过程
            for i in 1..=5 {
                println!("检查步骤 {}...", i);
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
            println!("项目检查完成，一切正常！");
        }
    }
}
