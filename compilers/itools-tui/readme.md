# iTools TUI

## 概述

iTools TUI 是 iTools 项目的终端用户界面模块，提供了功能丰富的命令行界面。

## 功能特性

- 现代化的终端界面
- 响应式设计
- 丰富的交互组件
- 国际化支持
- 与 iTools 核心功能集成

## 安装

```bash
# 从源码构建
cargo build --package itools-tui

# 运行示例
cargo run --package itools-tui --example basic
```

## 使用方法

### 基本使用

1. **在项目中集成 TUI**
   ```rust
   use itools_tui::Terminal;
   
   fn main() -> Result<(), Box<dyn std::error::Error>> {
       let mut terminal = Terminal::new()?;
       terminal.clear()?;
       
       // 在这里添加 TUI 逻辑
       
       terminal.restore()?;
       Ok(())
   }
   ```

2. **使用组件**
   ```rust
   use itools_tui::components::{Button, Input};
   
   // 创建按钮
   let button = Button::new("Click Me");
   
   // 创建输入框
   let mut input = Input::new("Enter text:");
   ```

## 组件库

### 可用组件

- **Button** - 可点击按钮
- **Input** - 文本输入框
- **List** - 可滚动列表
- **Panel** - 容器面板
- **ProgressBar** - 进度条
- **SelectMenu** - 选择菜单
- **Table** - 表格组件
- **LoadingAnimation** - 加载动画

### 组件示例

#### 按钮

```rust
use itools_tui::components::Button;

let button = Button::new("Submit");
button.render(&mut terminal, 10, 5)?;
```

#### 输入框

```rust
use itools_tui::components::Input;

let mut input = Input::new("Username:");
input.render(&mut terminal, 10, 5)?;
```

## 开发指南

### 项目结构

```
itools-tui/
├── src/
│   ├── components/     # UI 组件
│   │   ├── button.rs
│   │   ├── input.rs
│   │   ├── list.rs
│   │   ├── loading_animation.rs
│   │   ├── mod.rs
│   │   ├── panel.rs
│   │   ├── progress_bar.rs
│   │   ├── select_menu.rs
│   │   └── table.rs
│   ├── event.rs        # 事件处理
│   ├── layout.rs       # 布局管理
│   ├── lib.rs          # 库入口
│   ├── render.rs       # 渲染逻辑
│   ├── style.rs        # 样式定义
│   ├── terminal.rs     # 终端管理
│   └── utils.rs        # 工具函数
├── examples/           # 示例代码
│   └── basic.rs
├── Cargo.toml          # 项目配置
└── readme.md           # 项目文档
```

### 扩展 TUI

1. **添加新组件**
   - 在 `src/components/` 目录中创建新的组件文件
   - 在 `src/components/mod.rs` 中导出新组件
   - 实现 `Component` trait

2. **自定义样式**
   - 在 `src/style.rs` 中定义新的样式
   - 在组件中使用自定义样式

## 国际化

TUI 支持多语言，翻译资源与 iTools 核心共享。

### 支持的语言
- 英文 (`en`)
- 中文 (`zh-CN`)

### 语言切换

通过设置 `LANG` 环境变量来切换语言：

```bash
# 使用英文
LANG=en cargo run --package itools-tui --example basic

# 使用中文
LANG=zh-CN cargo run --package itools-tui --example basic
```

## 故障排除

- **终端显示异常**：确保终端支持 ANSI 转义序列
- **组件渲染错误**：检查终端尺寸是否足够大
- **事件处理无响应**：确保正确处理终端事件

## 贡献

欢迎提交 Issue 和 Pull Request 来改进 iTools TUI。

## 许可证

AGPL-3.0
