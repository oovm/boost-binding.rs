# iTools TUI

💻 iTools TUI 是 iTools 项目的终端用户界面模块，提供了功能丰富的命令行界面。

## 特性

- ✨ 现代化的终端界面
- ✨ 响应式设计
- ✨ 丰富的交互组件
- ✨ 国际化支持
- ✨ 与 iTools 核心功能集成

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

### 组件库

#### 可用组件

- **Button** - 可点击按钮
- **Input** - 文本输入框
- **List** - 可滚动列表
- **Panel** - 容器面板
- **ProgressBar** - 进度条
- **SelectMenu** - 选择菜单
- **Table** - 表格组件
- **LoadingAnimation** - 加载动画

#### 按钮示例

```rust
use itools_tui::components::Button;

let button = Button::new("Submit");
button.render(&mut terminal, 10, 5)?;
```

#### 输入框示例

```rust
use itools_tui::components::Input;

let mut input = Input::new("Username:");
input.render(&mut terminal, 10, 5)?;
```

## 注意事项

- 终端显示异常时，确保终端支持 ANSI 转义序列
- 组件渲染错误时，检查终端尺寸是否足够大
- 事件处理无响应时，确保正确处理终端事件
- 语言切换通过设置 `LANG` 环境变量来实现
- 支持的语言：英文 (`en`)、中文 (`zh-CN`)