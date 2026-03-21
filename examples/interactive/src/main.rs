//! 交互式选择示例
//!
//! 展示如何使用 iTools 的 TUI 功能，包括交互式菜单、进度条和加载动画

use itools::{Parse, init_cli, tui};
use itools_macros::command;
use std::time::Duration;

/// 测试命令行工具
#[command]
#[derive(Debug)]
pub struct InteractiveCli {
    /// 命令
    pub command: InteractiveCommand,
}

/// 测试命令枚举
#[command]
#[derive(Debug)]
pub enum InteractiveCommand {
    /// 基于数字输入的交互式菜单
    NumberSelect,
    /// 基于上下键的交互式菜单
    InteractiveSelect,
    /// 进度条示例
    ProgressBar,
    /// 加载动画示例
    LoadingAnimation,
}

fn main() {
    // 初始化 CLI 环境
    init_cli();

    // 获取命令行参数
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    // 解析命令行参数
    let cli = InteractiveCli::parse(&mut args);

    // 执行命令
    match cli.command {
        InteractiveCommand::NumberSelect => {
            println!("=== 基于数字输入的交互式菜单 ===");
            println!();
            let items = ["选项 1: 开始新任务", "选项 2: 查看历史记录", "选项 3: 退出程序"];
            if let Some(choice) = tui::select(&items, "请选择一个选项:") {
                println!("你选择了: {}", items[choice]);
            }
            println!();
        }
        InteractiveCommand::InteractiveSelect => {
            println!("=== 基于上下键的交互式菜单 ===");
            println!();
            let items = ["选项 1: 开始新任务", "选项 2: 查看历史记录", "选项 3: 退出程序"];
            if let Some(choice) = tui::interactive_select(&items, "请选择一个选项 (使用上下键选择，Enter 确认，Esc 取消):")
            {
                println!("你选择了: {}", items[choice]);
            }
            else {
                println!("你取消了选择");
            }
            println!();
        }
        InteractiveCommand::ProgressBar => {
            println!("=== 进度条示例 ===");
            println!();
            let mut pb = tui::ProgressBar::new(100, 50);
            for i in 0..=100 {
                pb.update(i);
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            pb.finish();
            println!();
        }
        InteractiveCommand::LoadingAnimation => {
            println!("=== 加载动画示例 ===");
            println!();
            tui::loading_animation(Duration::from_secs(3), "加载中...");
            println!();
        }
    }

    println!("=== 示例完成 ===");
}
