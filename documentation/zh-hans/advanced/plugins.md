# Dylib 构建与插件体系

本文档介绍如何在 iTools 中使用动态库（dylib）构建以及插件体系的设计和实现。

## 1. Dylib 构建基础

### 什么是动态库（Dylib）

动态库（Dynamic Library，简称 dylib）是一种可以被多个应用程序共享的库文件。与静态库不同，动态库在运行时被加载，而不是在编译时链接到可执行文件中。

### 动态库的优势

1. **代码复用**：多个应用程序可以共享同一个库文件，避免代码重复
2. **减小二进制文件大小**：每个应用程序只包含自己的代码，不包含库代码
3. **便于维护**：库的更新会自动应用到所有使用该库的应用程序
4. **一致的行为**：所有应用程序使用相同的库实现，确保行为一致

### 在 Rust 中创建动态库

在 Rust 中，创建动态库非常简单，只需在 `Cargo.toml` 文件中设置 `crate-type` 为 `dylib`：

```toml
[lib]
crate-type = ["dylib"]
```

## 2. iTools 中的 Dylib 示例

### 示例结构

iTools 提供了 `shared-dylib` 示例，展示了如何创建和使用动态库：

```
shared-dylib/
├── Cargo.toml          # 工作区配置
├── libasred/            # 共享动态库
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── asred-cli/           # 命令行应用
│   ├── readme.md
├── asred-run/           # 插件应用
│   ├── readme.md
├── bin/                 # 二进制文件
│   └── shared.rs
├── src/                 # 主库
│   └── lib.rs
└── README.md            # 文档
```

### 共享库实现

共享库 `libasred` 包含可被多个应用程序使用的公共功能：

```rust
// libasred/src/lib.rs

/// 生成问候消息
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to libasred shared library.", name)
}

/// 计算数字的平方
pub fn square(num: i32) -> i32 {
    num * num
}

/// 应用配置结构体
#[derive(Debug)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
}

impl AppConfig {
    /// 创建新的应用配置
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
        }
    }
}
```

## 3. 插件体系设计

### 插件体系的概念

插件体系允许第三方开发者为 iTools 添加新功能，而无需修改核心代码。插件通常以动态库的形式实现，可以在运行时被加载和卸载。

### 插件体系的组成部分

1. **插件接口**：定义插件必须实现的方法和结构
2. **插件加载器**：负责加载和管理插件
3. **插件实现**：第三方开发者实现的具体功能

### 插件接口设计

插件接口通常定义为 trait，所有插件必须实现这个 trait：

```rust
// 插件接口
trait Plugin {
    /// 获取插件名称
    fn name(&self) -> &str;
    
    /// 获取插件版本
    fn version(&self) -> &str;
    
    /// 初始化插件
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 执行插件功能
    fn run(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 清理插件资源
    fn cleanup(&mut self);
}
```

### 插件加载器实现

插件加载器负责从指定目录加载插件：

```rust
// 插件加载器
struct PluginLoader {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginLoader {
    /// 创建新的插件加载器
    fn new() -> Self {
        Self { plugins: Vec::new() }
    }
    
    /// 加载指定目录中的所有插件
    fn load_plugins(&mut self, plugin_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 实现插件加载逻辑
        Ok(())
    }
    
    /// 获取所有已加载的插件
    fn get_plugins(&self) -> &[Box<dyn Plugin>] {
        &self.plugins
    }
}
```

## 4. 如何使用 Dylib 和插件

### 步骤 1: 创建共享库

1. 创建一个新的 Rust 库项目：
   ```bash
   cargo new --lib my-shared-lib
   ```

2. 在 `Cargo.toml` 中设置为动态库：
   ```toml
   [lib]
   crate-type = ["dylib"]
   ```

3. 实现共享功能：
   ```rust
   // src/lib.rs
   pub fn add(a: i32, b: i32) -> i32 {
       a + b
   }
   ```

### 步骤 2: 创建使用共享库的应用

1. 创建一个新的 Rust 二进制项目：
   ```bash
   cargo new my-app
   ```

2. 在 `Cargo.toml` 中添加对共享库的依赖：
   ```toml
   [dependencies]
   my-shared-lib = { path = "../my-shared-lib" }
   ```

3. 在代码中使用共享库：
   ```rust
   // src/main.rs
   use my_shared_lib::add;
   
   fn main() {
       let result = add(2, 3);
       println!("2 + 3 = {}", result);
   }
   ```

### 步骤 3: 创建插件

1. 创建一个新的 Rust 库项目作为插件：
   ```bash
   cargo new --lib my-plugin
   ```

2. 在 `Cargo.toml` 中设置为动态库并添加对插件接口的依赖：
   ```toml
   [lib]
   crate-type = ["dylib"]
   
   [dependencies]
   plugin-interface = { path = "../plugin-interface" }
   ```

3. 实现插件接口：
   ```rust
   // src/lib.rs
   use plugin_interface::Plugin;
   
   struct MyPlugin {
       name: String,
       version: String,
   }
   
   impl Plugin for MyPlugin {
       fn name(&self) -> &str {
           &self.name
       }
       
       fn version(&self) -> &str {
           &self.version
       }
       
       fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
           println!("Initializing {} plugin...", self.name);
           Ok(())
       }
       
       fn run(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
           println!("Running {} plugin with args: {:?}", self.name, args);
           Ok(())
       }
       
       fn cleanup(&mut self) {
           println!("Cleaning up {} plugin...", self.name);
       }
   }
   
   // 插件入口点
   #[no_mangle]
   pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
       Box::new(MyPlugin {
           name: "My Plugin".to_string(),
           version: "1.0.0".to_string(),
       })
   }
   ```

### 步骤 4: 加载和使用插件

1. 在主应用中实现插件加载：
   ```rust
   // src/main.rs
   use plugin_interface::{Plugin, PluginLoader};
   
   fn main() {
       let mut loader = PluginLoader::new();
       
       // 加载插件
       if let Err(e) = loader.load_plugins("./plugins") {
           eprintln!("Error loading plugins: {}", e);
           return;
       }
       
       // 使用插件
       for plugin in loader.get_plugins() {
           println!("Found plugin: {} v{}", plugin.name(), plugin.version());
           
           if let Err(e) = plugin.run(&["arg1", "arg2"]) {
               eprintln!("Error running plugin {}: {}", plugin.name(), e);
           }
       }
   }
   ```

## 5. 最佳实践

### 1. 版本管理

- 使用语义化版本控制（Semantic Versioning）管理插件版本
- 确保插件接口的向后兼容性
- 提供插件版本检查机制

### 2. 错误处理

- 插件应该返回详细的错误信息
- 主应用应该优雅处理插件加载和执行错误
- 实现插件超时机制，防止插件卡住整个应用

### 3. 安全考虑

- 验证插件的签名，确保来源可信
- 限制插件的权限，防止恶意插件访问敏感资源
- 实现沙箱机制，隔离插件执行环境

### 4. 性能优化

- 延迟加载插件，只在需要时加载
- 缓存插件实例，避免重复加载
- 优化插件间的通信，减少开销

## 6. 示例应用

### 基本应用示例

```rust
// 使用共享库的基本应用
use my_shared_lib::greet;

fn main() {
    println!("=== Basic Application ===");
    let message = greet("World");
    println!("{}", message);
    println!("Application completed successfully!");
}
```

### 插件应用示例

```rust
// 使用插件的应用
use plugin_interface::{Plugin, PluginLoader};

fn main() {
    println!("=== Plugin Application ===");
    
    let mut loader = PluginLoader::new();
    
    // 加载插件
    if let Err(e) = loader.load_plugins("./plugins") {
        eprintln!("Error loading plugins: {}", e);
        return;
    }
    
    // 执行所有插件
    for plugin in loader.get_plugins() {
        println!("\nRunning plugin: {} v{}", plugin.name(), plugin.version());
        
        if let Err(e) = plugin.init() {
            eprintln!("Error initializing plugin: {}", e);
            continue;
        }
        
        if let Err(e) = plugin.run(&["--verbose"]) {
            eprintln!("Error running plugin: {}", e);
        }
        
        plugin.cleanup();
    }
    
    println!("\nPlugin application completed successfully!");
}
```

## 7. 总结

动态库和插件体系是 iTools 的重要特性，它们提供了：

1. **代码复用**：通过共享库减少代码重复
2. **可扩展性**：通过插件体系允许第三方扩展功能
3. **模块化**：将功能分解为独立的组件
4. **灵活性**：可以在运行时加载和卸载插件

通过合理使用动态库和插件体系，可以构建更加模块化、可扩展的应用程序，提高开发效率和代码质量。