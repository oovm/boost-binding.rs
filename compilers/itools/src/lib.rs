#![warn(missing_docs)]

//! iTools CLI 核心模块
//!
//! 提供命令行接口功能，包括命令解析、执行和交互。

use clap::{Parser, Subcommand};
use tracing::info;

/// 导入宏
#[macro_use]
extern crate itools_macros;

/// 国际化功能 - 使用 itools-localization 模块的国际化实现
pub use itools_localization::{get_locale, set_locale, t, t_with_args};

/// 导出翻译宏
pub use itools_macros::t;

/// 命令行参数解析 trait
pub trait Parse {
    /// 从命令行参数中解析当前类型
    fn parse(args: &mut Vec<String>) -> Self;
}

/// 为 String 实现 Parse trait
impl Parse for String {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t!("errors.expected_string_argument"));
        }
        args.remove(0)
    }
}

/// 为 i32 实现 Parse trait
impl Parse for i32 {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t!("errors.expected_integer_argument"));
        }
        args.remove(0).parse().expect(&t!("errors.invalid_integer"))
    }
}

/// 为 u32 实现 Parse trait
impl Parse for u32 {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t!("errors.expected_unsigned_integer_argument"));
        }
        args.remove(0).parse().expect(&t!("errors.invalid_unsigned_integer"))
    }
}

/// 为 bool 实现 Parse trait
impl Parse for bool {
    fn parse(args: &mut Vec<String>) -> Self {
        if args.is_empty() {
            panic!("{}", t!("errors.expected_boolean_argument"));
        }
        args.remove(0).parse().expect(&t!("errors.invalid_boolean"))
    }
}

/// iTools 命令行工具 - 使用 clap 解析
#[derive(Parser, Debug)]
#[command(name = "itools", about = "iTools - A powerful toolkit for Rust developers")]
pub struct App {
    /// 子命令
    #[command(subcommand)]
    pub command: ClapCommand,
}

/// 命令枚举 - 使用 clap 解析
#[derive(Subcommand, Debug)]
pub enum ClapCommand {
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
            panic!("{}", t!("errors.expected_command"));
        }
        let cmd = args.remove(0);
        match cmd.as_str() {
            "init" => Self::Init { name: String::parse(args) },
            "build" => Self::Build,
            "dev" => Self::Dev,
            "check" => Self::Check,
            _ => panic!("{}", t!("errors.unknown_command")),
        }
    }
}

/// 从 ClapCommand 转换为 Command
impl From<ClapCommand> for Command {
    fn from(clap_command: ClapCommand) -> Self {
        match clap_command {
            ClapCommand::Init { name } => Self::Init { name },
            ClapCommand::Build => Self::Build,
            ClapCommand::Dev => Self::Dev,
            ClapCommand::Check => Self::Check,
        }
    }
}

/// 从 App 转换为 Cli
impl From<App> for Cli {
    fn from(app: App) -> Self {
        Self {
            command: app.command.into(),
        }
    }
}

/// 初始化 CLI 环境
pub fn init_cli() {
    // 初始化日志
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();

    info!("{}", t!("messages.cli_initialized"));
}

/// TUI 功能模块
pub mod tui {
    use std::{
        io::{self, Write},
        thread,
        time::Duration,
    };

    /// 进度条
    pub struct ProgressBar {
        total: u64,
        current: u64,
        width: u16,
    }

    impl ProgressBar {
        /// 创建新的进度条
        pub fn new(total: u64, width: u16) -> Self {
            Self { total, current: 0, width }
        }

        /// 更新进度
        pub fn update(&mut self, current: u64) {
            self.current = current;
            self.draw();
        }

        /// 增加进度
        pub fn inc(&mut self, delta: u64) {
            self.current += delta;
            self.draw();
        }

        /// 完成进度
        pub fn finish(&mut self) {
            self.current = self.total;
            self.draw();
            println!();
        }

        /// 绘制进度条
        fn draw(&self) {
            let percentage = if self.total > 0 { (self.current as f64 / self.total as f64) * 100.0 } else { 0.0 };

            let filled = (percentage / 100.0 * self.width as f64) as u16;
            let empty = self.width - filled;

            print!("\r[");
            print!("{}", "=".repeat(filled as usize));
            print!("{}", " ".repeat(empty as usize));
            print!(" ] {:.1}%", percentage);
            io::stdout().flush().unwrap();
        }
    }

    /// 交互式选择菜单
    pub fn select<T>(items: &[T], prompt: &str) -> Option<usize>
    where
        T: std::fmt::Display,
    {
        println!("{}", prompt);
        for (i, item) in items.iter().enumerate() {
            println!("{}: {}", i + 1, item);
        }

        loop {
            print!("请选择 (1-{}): ", items.len());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<usize>() {
                Ok(choice) if choice >= 1 && choice <= items.len() => {
                    return Some(choice - 1);
                }
                _ => {
                    println!("无效的选择，请重新输入");
                }
            }
        }
    }

    /// 显示加载动画
    pub fn loading_animation(duration: Duration, message: &str) {
        let chars = ["|", "/", "-", "\\"];
        let start = std::time::Instant::now();

        print!("{}", message);
        io::stdout().flush().unwrap();

        let mut i = 0;
        while start.elapsed() < duration {
            print!("\r{}{}", message, chars[i % chars.len()]);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
            i += 1;
        }

        print!("\r{}{}", message, " ");
        println!();
    }

    /// 交互式选择菜单（基于上下键）
    pub fn interactive_select<T>(items: &[T], prompt: &str) -> Option<usize>
    where
        T: std::fmt::Display,
    {
        use crossterm::{cursor, event, terminal};

        // 保存终端状态
        let mut stdout = io::stdout();
        terminal::enable_raw_mode().unwrap();

        let result = {
            // 清除屏幕
            print!("{}{}", cursor::SavePosition, terminal::Clear(terminal::ClearType::FromCursorDown));
            println!("{}", prompt);

            let mut selected = 0;
            let item_count = items.len();

            loop {
                // 显示选项
                for (i, item) in items.iter().enumerate() {
                    if i == selected {
                        // 高亮显示选中项
                        print!("[x] {}\n", item);
                    }
                    else {
                        print!("[ ] {}\n", item);
                    }
                }

                // 等待键盘输入
                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let event::Event::Key(key_event) = event::read().unwrap() {
                        match key_event.code {
                            event::KeyCode::Up => {
                                if selected > 0 {
                                    selected -= 1;
                                }
                            }
                            event::KeyCode::Down => {
                                if selected < item_count - 1 {
                                    selected += 1;
                                }
                            }
                            event::KeyCode::Enter => {
                                // 确认选择
                                break Some(selected);
                            }
                            event::KeyCode::Esc => {
                                // 取消选择
                                break None;
                            }
                            _ => {}
                        }
                    }
                }

                // 回到顶部
                print!("{}", cursor::RestorePosition);
                print!("{}", terminal::Clear(terminal::ClearType::FromCursorDown));
                println!("{}", prompt);
            }
        };

        // 恢复终端状态
        terminal::disable_raw_mode().unwrap();
        print!("{}{}", cursor::RestorePosition, terminal::Clear(terminal::ClearType::FromCursorDown));
        stdout.flush().unwrap();

        result
    }
}

/// 命令行参数构建器
///
/// 提供流式接口来构建命令行参数
pub struct Builder {
    args: Vec<String>,
}

impl Builder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }

    /// 设置命令
    pub fn command(mut self, command: &str) -> Self {
        self.args.push(command.to_string());
        self
    }

    /// 添加位置参数
    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    /// 添加选项参数
    pub fn option(mut self, option: &str, value: &str) -> Self {
        self.args.push(option.to_string());
        self.args.push(value.to_string());
        self
    }

    /// 构建命令行参数向量
    pub fn build_args(self) -> Vec<String> {
        self.args
    }

    /// 构建并解析为指定类型
    pub fn build<T: Parse>(self) -> T {
        let mut args = self.args;
        T::parse(&mut args)
    }
}

/// 执行命令
pub fn execute_command(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Command::Init { name } => {
            info!("{}", t!("messages.initializing_project"));
            // 测试进度条
            let mut pb = tui::ProgressBar::new(100, 50);
            for i in 0..=100 {
                pb.update(i);
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            pb.finish();
            Ok(())
        }
        Command::Build => {
            info!("{}", t!("messages.building_project"));
            // 测试加载动画
            tui::loading_animation(std::time::Duration::from_secs(3), "构建中...");
            Ok(())
        }
        Command::Dev => {
            info!("{}", t!("messages.starting_dev_server"));
            // 测试交互式选择菜单
            let items = ["选项 1", "选项 2", "选项 3"];
            if let Some(choice) = tui::select(&items, "请选择一个选项:") {
                info!("选择了: {}", items[choice]);
            }
            Ok(())
        }
        Command::Check => {
            info!("{}", t!("messages.checking_project"));
            // 测试进度条
            let mut pb = tui::ProgressBar::new(50, 30);
            for i in 0..=50 {
                pb.update(i);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            pb.finish();
            Ok(())
        }
    }
}

/// 构建带有资源的项目
///
/// 简化资源编译和图标添加的过程，自动处理 Windows 平台的资源文件
///
/// # 示例
/// ```rust
/// use itools::build_with_resource;
///
/// fn main() {
///     build_with_resource("assets/icon.ico");
/// }
/// ```
pub fn build_with_resource(icon_path: &str) {
    #[cfg(target_env = "msvc")]
    {
        use std::{fs, process::Command};

        // 生成 .rc 文件
        let rc_content = format!(
            r#"#include <windows.h>

// 图标资源
IDI_ICON1 ICON "{}"
"#,
            icon_path
        );

        // 写入 .rc 文件
        if let Ok(_) = fs::write("itools.rc", rc_content) {
            // 尝试使用 rc.exe 编译资源文件
            let status = Command::new("rc.exe").arg("/nologo").arg("itools.rc").status();

            // 清理临时文件
            let _ = fs::remove_file("itools.rc");

            match status {
                Ok(s) if s.success() => {
                    // 告诉 linker 链接资源文件
                    println!("cargo:rustc-link-lib=static=itools.res");
                    println!("cargo:rustc-link-search=.");
                }
                _ => {
                    // rc.exe 不可用或编译失败，跳过资源编译
                    println!("cargo:warning=Resource compilation skipped (rc.exe not found or failed)");
                }
            }
        }
    }
}

/// 国际化测试模块
pub mod i18n_test;
pub use i18n_test::*;
