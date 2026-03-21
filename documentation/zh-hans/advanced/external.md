# 外部命令文档

本文档详细介绍 iTools 命令行工具的外部命令功能，包括如何调用、管理和集成外部命令，帮助开发者扩展 iTools 的功能。

## 目录

1. [基本概念](#基本概念)
2. [调用外部命令](#调用外部命令)
3. [命令执行选项](#命令执行选项)
4. [错误处理](#错误处理)
5. [使用示例](#使用示例)

---

## 基本概念

外部命令是指由 iTools 调用的系统或第三方命令行工具。iTools 提供了统一的接口来执行这些外部命令，包括捕获输出、处理错误和管理执行环境。

## 调用外部命令

使用 `Command` 模块可以轻松调用外部命令：

```rust
use std::process::Command;

fn main() {
    let output = Command::new("echo")
        .arg("Hello, World!")
        .output()
        .expect("无法执行命令");

    println!("退出码: {}", output.status);
    println!("标准输出: {}", String::from_utf8_lossy(&output.stdout));
    println!("标准错误: {}", String::from_utf8_lossy(&output.stderr));
}
```

## 命令执行选项

### 基本执行方法

| 方法 | 说明 | 返回值 |
|------|------|--------|
| `output()` | 执行命令并捕获输出 | `Result<Output, Error>` |
| `spawn()` | 异步执行命令 | `Result<Child, Error>` |
| `status()` | 执行命令并返回状态 | `Result<ExitStatus, Error>` |
| `output()` | 执行命令并捕获输出 | `Result<Output, Error>` |

### 命令配置选项

| 方法 | 说明 | 参数 |
|------|------|------|
| `arg()` | 添加单个参数 | 参数值 (String) |
| `args()` | 添加多个参数 | 参数迭代器 |
| `env()` | 设置环境变量 | 变量名 (String), 变量值 (String) |
| `envs()` | 设置多个环境变量 | 环境变量迭代器 |
| `current_dir()` | 设置工作目录 | 目录路径 (PathBuf) |
| `stdin()` | 设置标准输入 | 输入源 |
| `stdout()` | 设置标准输出 | 输出目标 |
| `stderr()` | 设置标准错误 | 错误输出目标 |

## 错误处理

调用外部命令时，可能会遇到以下错误：

1. **命令未找到**：指定的命令在系统路径中不存在
2. **权限不足**：没有执行命令的权限
3. **命令执行失败**：命令执行过程中出现错误
4. **输入/输出错误**：与命令的输入/输出交互时出现错误

处理这些错误的最佳实践：

```rust
use std::process::{Command, Stdio};
use std::io::{self, Write};

fn execute_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("git")
        .arg("status")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        println!("命令执行成功:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("命令执行失败:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
```

## 使用示例

### 示例 1: 基本外部命令调用

```rust
use std::process::Command;

fn main() {
    // 执行简单命令
    let output = Command::new("ls")
        .arg("-la")
        .output()
        .expect("无法执行 ls 命令");

    println!("退出码: {}", output.status);
    println!("输出:");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
```

### 示例 2: 带环境变量的命令

```rust
use std::process::Command;
use std::env;

fn main() {
    let mut cmd = Command::new("echo");
    
    // 设置环境变量
    cmd.env("MY_VAR", "hello world")
       .arg("$MY_VAR");

    let output = cmd.output().expect("无法执行命令");
    println!("输出: {}", String::from_utf8_lossy(&output.stdout));
}
```

### 示例 3: 异步执行命令

```rust
use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader};

fn main() {
    // 异步执行命令
    let mut child = Command::new("ping")
        .arg("localhost")
        .stdout(Stdio::piped())
        .spawn()
        .expect("无法执行 ping 命令");

    // 读取输出
    let stdout = child.stdout.as_mut().expect("无法获取标准输出");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}`, line),
            Err(e) => eprintln!("读取错误: {}", e),
        }
    }

    // 等待命令完成
    let status = child.wait().expect("等待命令失败");
    println!("退出码: {}", status);
}
```

### 示例 4: 与 iTools 集成

```rust
use itools::{Parse, init_cli};
use itools_macros::Command;
use std::process::Command as StdCommand;

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
    /// 执行外部命令
    Exec {
        /// 命令名称
        command: String,
        /// 命令参数
        args: Vec<String>,
    },
    /// 查看系统信息
    SystemInfo,
}

#[tokio::main]
async fn main() {
    // 初始化 CLI 环境
    init_cli();

    // 获取命令行参数
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    // 解析命令行参数
    let cli = Cli::parse(&mut args);

    // 执行命令
    match cli.command {
        Command::Exec { command, args } => {
            println!("执行命令: {} {}", command, args.join(" "));
            
            let output = StdCommand::new(command)
                .args(args)
                .output()
                .expect("无法执行命令");

            println!("退出码: {}", output.status);
            println!("标准输出: {}", String::from_utf8_lossy(&output.stdout));
            println!("标准错误: {}", String::from_utf8_lossy(&output.stderr));
        }
        Command::SystemInfo => {
            println!("系统信息:");
            
            // 执行 uname 命令（Linux/macOS）或 ver 命令（Windows）
            #[cfg(target_os = "windows")]
            {
                let output = StdCommand::new("ver").output().expect("无法执行命令");
                println!("版本: {}", String::from_utf8_lossy(&output.stdout));
            }
            
            #[cfg(not(target_os = "windows"))]
            {
                let output = StdCommand::new("uname").arg("-a").output().expect("无法执行命令");
                println!("系统: {}", String::from_utf8_lossy(&output.stdout));
            }
        }
    }
}
```

### 示例 5: 高级外部命令集成

```rust
use itools::{Parse, init_cli};
use itools_macros::Command;
use std::process::{Command as StdCommand, Stdio};
use std::io::{self, BufRead, BufReader};

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
    /// 运行构建命令
    Build {
        /// 构建目标
        target: String,
    },
    /// 运行测试
    Test {
        /// 测试模式
        mode: String,
    },
}

#[tokio::main]
async fn main() {
    // 初始化 CLI 环境
    init_cli();

    // 获取命令行参数
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    // 解析命令行参数
    let cli = Cli::parse(&mut args);

    // 执行命令
    match cli.command {
        Command::Build { target } => {
            println!("构建目标: {}", target);
            
            // 执行 cargo build 命令
            let mut child = StdCommand::new("cargo")
                .arg("build")
                .arg("--target")
                .arg(target)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("无法执行 cargo build 命令");

            // 实时读取输出
            let stdout = child.stdout.as_mut().expect("无法获取标准输出");
            let stderr = child.stderr.as_mut().expect("无法获取标准错误");

            let stdout_reader = BufReader::new(stdout);
            let stderr_reader = BufReader::new(stderr);

            println!("构建输出:");
            for line in stdout_reader.lines() {
                if let Ok(line) = line {
                    println!("{}", line);
                }
            }

            for line in stderr_reader.lines() {
                if let Ok(line) = line {
                    eprintln!("{}", line);
                }
            }

            let status = child.wait().expect("等待构建完成失败");
            println!("构建完成，退出码: {}", status);
        }
        Command::Test { mode } => {
            println!("测试模式: {}", mode);
            
            // 执行 cargo test 命令
            let mut child = StdCommand::new("cargo")
                .arg("test")
                .arg(mode)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("无法执行 cargo test 命令");

            // 实时读取输出
            let stdout = child.stdout.as_mut().expect("无法获取标准输出");
            let stderr = child.stderr.as_mut().expect("无法获取标准错误");

            let stdout_reader = BufReader::new(stdout);
            let stderr_reader = BufReader::new(stderr);

            println!("测试输出:");
            for line in stdout_reader.lines() {
                if let Ok(line) = line {
                    println!("{}", line);
                }
            }

            for line in stderr_reader.lines() {
                if let Ok(line) = line {
                    eprintln!("{}", line);
                }
            }

            let status = child.wait().expect("等待测试完成失败");
            println!("测试完成，退出码: {}", status);
        }
    }
}
```