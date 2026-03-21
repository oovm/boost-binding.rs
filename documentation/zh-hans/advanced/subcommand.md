# 子命令文档

本文档详细介绍 iTools 命令行工具的子命令功能，包括如何定义、使用和组织子命令，帮助开发者构建复杂的命令行工具。

## 目录

1. [基本概念](#基本概念)
2. [定义子命令](#定义子命令)
3. [嵌套子命令](#嵌套子命令)
4. [子命令参数](#子命令参数)
5. [使用示例](#使用示例)

---

## 基本概念

子命令是命令行工具中用于组织和分类功能的一种方式，允许将相关的操作分组到一个主命令下。在 iTools 中，子命令通过枚举类型定义，每个枚举变体代表一个子命令。

## 定义子命令

使用 `#[Command]` 宏可以轻松定义子命令：

```rust
use itools::{Parse, init_cli};
use itools_macros::Command;

/// 主命令行工具
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
    },
    /// 构建项目
    Build,
    /// 开发服务器
    Dev,
    /// 检查项目
    Check,
}
```

## 嵌套子命令

iTools 支持嵌套子命令，通过在枚举中嵌套其他枚举类型实现：

```rust
use itools::{Parse, init_cli};
use itools_macros::Command;

/// 主命令行工具
#[Command]
#[derive(Debug)]
pub struct Cli {
    /// 命令
    pub command: MainCommand,
}

/// 主命令枚举
#[Command]
#[derive(Debug)]
pub enum MainCommand {
    /// 项目管理
    Project(ProjectCommand),
    /// 工具管理
    Tool(ToolCommand),
}

/// 项目命令枚举
#[Command]
#[derive(Debug)]
pub enum ProjectCommand {
    /// 创建项目
    Create {
        /// 项目名称
        name: String,
    },
    /// 构建项目
    Build,
    /// 运行项目
    Run,
}

/// 工具命令枚举
#[Command]
#[derive(Debug)]
pub enum ToolCommand {
    /// 安装工具
    Install {
        /// 工具名称
        name: String,
    },
    /// 卸载工具
    Uninstall {
        /// 工具名称
        name: String,
    },
    /// 列出工具
    List,
}
```

## 子命令参数

子命令可以接受参数，参数通过结构体字段定义：

```rust
use itools::{Parse, init_cli};
use itools_macros::Command;

/// 命令枚举
#[Command]
#[derive(Debug)]
pub enum Command {
    /// 初始化新项目
    Init {
        /// 项目名称
        name: String,
        /// 是否覆盖现有项目
        #[arg(short = 'f', long = "force")]
        force: bool,
    },
    /// 构建项目
    Build {
        /// 构建目标
        target: String,
        /// 是否启用发布模式
        #[arg(short = 'r', long = "release")]
        release: bool,
    },
}
```

## 使用示例

### 示例 1: 基本子命令

```rust
use itools::{Parse, init_cli};
use itools_macros::Command;

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
    },
    /// 构建项目
    Build,
    /// 开发服务器
    Dev,
    /// 检查项目
    Check,
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
        Command::Init { name } => {
            println!("初始化项目: {}", name);
        }
        Command::Build => {
            println!("构建项目");
        }
        Command::Dev => {
            println!("启动开发服务器");
        }
        Command::Check => {
            println!("检查项目");
        }
    }
}
```

### 示例 2: 嵌套子命令

```rust
use itools::{Parse, init_cli};
use itools_macros::Command;

/// 主命令行工具
#[Command]
#[derive(Debug)]
pub struct Cli {
    /// 命令
    pub command: MainCommand,
}

/// 主命令枚举
#[Command]
#[derive(Debug)]
pub enum MainCommand {
    /// 项目管理
    Project(ProjectCommand),
    /// 工具管理
    Tool(ToolCommand),
}

/// 项目命令枚举
#[Command]
#[derive(Debug)]
pub enum ProjectCommand {
    /// 创建项目
    Create {
        /// 项目名称
        name: String,
    },
    /// 构建项目
    Build {
        /// 构建目标
        target: String,
    },
    /// 运行项目
    Run,
}

/// 工具命令枚举
#[Command]
#[derive(Debug)]
pub enum ToolCommand {
    /// 安装工具
    Install {
        /// 工具名称
        name: String,
    },
    /// 卸载工具
    Uninstall {
        /// 工具名称
        name: String,
    },
    /// 列出工具
    List,
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
        MainCommand::Project(project_cmd) => {
            match project_cmd {
                ProjectCommand::Create { name } => {
                    println!("创建项目: {}", name);
                }
                ProjectCommand::Build { target } => {
                    println!("构建项目，目标: {}", target);
                }
                ProjectCommand::Run => {
                    println!("运行项目");
                }
            }
        }
        MainCommand::Tool(tool_cmd) => {
            match tool_cmd {
                ToolCommand::Install { name } => {
                    println!("安装工具: {}", name);
                }
                ToolCommand::Uninstall { name } => {
                    println!("卸载工具: {}", name);
                }
                ToolCommand::List => {
                    println!("列出工具");
                }
            }
        }
    }
}
```

### 示例 3: 子命令参数

```rust
use itools::{Parse, init_cli};
use itools_macros::Command;

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
        /// 是否覆盖现有项目
        force: bool,
    },
    /// 构建项目
    Build {
        /// 构建目标
        target: String,
        /// 是否启用发布模式
        release: bool,
    },
    /// 运行项目
    Run {
        /// 端口号
        port: u32,
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
        Command::Init { name, force } => {
            if force {
                println!("强制初始化项目: {}", name);
            } else {
                println!("初始化项目: {}", name);
            }
        }
        Command::Build { target, release } => {
            if release {
                println!("以发布模式构建项目，目标: {}", target);
            } else {
                println!("构建项目，目标: {}", target);
            }
        }
        Command::Run { port } => {
            println!("运行项目，端口: {}", port);
        }
    }
}
```