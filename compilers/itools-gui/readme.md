# iTools GUI

## 概述

iTools GUI 是 iTools 项目的图形用户界面模块，提供了直观的用户交互界面。

## 功能特性

- 现代化的图形界面
- 响应式设计
- 国际化支持
- 与 iTools 核心功能集成

## 安装

```bash
# 从源码构建
cargo build --package itools-gui

# 运行
cargo run --package itools-gui
```

## 使用方法

### 基本使用

1. **启动 GUI**
   ```bash
   cargo run --package itools-gui
   ```

2. **创建新项目**
   - 在主界面点击 "New Project" 按钮
   - 输入项目名称和路径
   - 选择项目类型
   - 点击 "Create" 按钮

3. **打开现有项目**
   - 在主界面点击 "Open Project" 按钮
   - 浏览并选择项目目录
   - 点击 "Open" 按钮

### 项目管理

- **构建项目**：点击工具栏中的 "Build" 按钮
- **运行项目**：点击工具栏中的 "Run" 按钮
- **检查项目**：点击工具栏中的 "Check" 按钮

### 配置

GUI 配置存储在项目根目录的 `.itools/config.toml` 文件中。

## 开发指南

### 项目结构

```
itools-gui/
├── src/
│   ├── app.rs         # 应用主逻辑
│   ├── components.rs  # UI 组件
│   ├── event.rs       # 事件处理
│   ├── lib.rs         # 库入口
│   ├── render.rs      # 渲染逻辑
│   ├── style.rs       # 样式定义
│   ├── utils.rs       # 工具函数
│   └── window.rs      # 窗口管理
├── Cargo.toml         # 项目配置
└── readme.md          # 项目文档
```

### 扩展 GUI

1. **添加新组件**
   - 在 `src/components.rs` 中定义新组件
   - 在 `src/render.rs` 中实现渲染逻辑
   - 在 `src/event.rs` 中处理组件事件

2. **添加新功能**
   - 在 `src/app.rs` 中添加新的应用逻辑
   - 更新 UI 组件以支持新功能
   - 添加相应的事件处理

## 国际化

GUI 支持多语言，翻译资源与 iTools 核心共享。

### 支持的语言
- 英文 (`en`)
- 中文 (`zh-CN`)

### 语言切换

通过系统语言设置自动切换，或在设置菜单中手动选择。

## 故障排除

- **GUI 启动失败**：检查依赖是否正确安装
- **界面显示异常**：检查系统分辨率和显示设置
- **功能无响应**：检查后台服务是否正常运行

## 贡献

欢迎提交 Issue 和 Pull Request 来改进 iTools GUI。

## 许可证

AGPL-3.0
