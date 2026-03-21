# iTools Walker

🚶 iTools Walker 是 iTools 项目的异步目录遍历库，提供并发处理和 glob 模式支持。

## 特性

- ✨ 异步：基于 Tokio 构建，支持非阻塞 I/O 操作
- ✨ 并发：支持并行目录遍历，可配置并发限制
- ✨ Glob 模式支持：使用 glob 模式包含和排除文件（如 .gitignore）
- ✨ 灵活的 API：提供回调式和流式两种使用方法
- ✨ 符号链接支持：可选的符号链接跟随
- ✨ 递归遍历：可配置的递归目录遍历

## 使用方法

### 基本异步遍历（流式）

```rust
use itools_walker::Walker;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建默认选项的遍历器
    let walker = Walker::new()?;
    
    // 异步遍历当前目录（返回流）
    let mut stream = walker.walk(".").await?;
    
    // 处理结果流
    while let Some(entry) = stream.next().await {
        match entry {
            Ok(walk_entry) => println!("{:?} - dir: {}, file: {}, symlink: {}", 
                walk_entry.path, walk_entry.is_dir(), walk_entry.is_file(), walk_entry.is_symlink()),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    
    Ok(())
}
```

### 带并发限制的遍历

```rust
use itools_walker::{Walker, WalkOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建带并发限制的选项
    let options = WalkOptions {
        concurrency_limit: Some(5), // 限制为 5 个并发操作
        ..Default::default()
    };
    
    let walker = Walker::with_options(options)?;
    
    // 使用回调遍历（对大型目录更内存高效）
    walker.walk_with(".", |entry| {
        println!("Processing: {:?}", entry.path);
        Ok(())
    }).await?;
    
    Ok(())
}
```

### Glob 模式支持

```rust
use itools_walker::{Walker, WalkOptions};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建带 glob 模式的选项
    let options = WalkOptions {
        include_patterns: vec!["**/*.rs", "**/*.toml"], // 包含 Rust 和 TOML 文件
        exclude_patterns: vec!["target/**", ".git/**"], // 排除 target 和 .git 目录
        ..Default::default()
    };
    
    let walker = Walker::with_options(options)?;
    
    let mut stream = walker.walk(".").await?;
    
    while let Some(entry) = stream.next().await {
        match entry {
            Ok(walk_entry) if walk_entry.is_file() => {
                println!("Found file: {:?}", walk_entry.path);
            }
            Err(e) => println!("Error: {:?}", e),
            _ => {}
        }
    }
    
    Ok(())
}
```

### 使用便捷函数

```rust
use itools_walker::{walk, walk_with_options, WalkOptions};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用默认选项的便捷函数
    let mut stream = walk(".").await?;
    let mut count = 0;
    
    while let Some(entry) = stream.next().await {
        if entry.is_ok() {
            count += 1;
        }
    }
    println!("Found {} entries", count);
    
    // 创建带 glob 模式的选项
    let options = WalkOptions {
        include_patterns: vec!["**/*.rs"],
        exclude_patterns: vec!["target/**"],
        concurrency_limit: Some(10),
        ..Default::default()
    };
    
    // 使用自定义选项的便捷函数
    let mut rust_stream = walk_with_options(".", options).await?;
    let mut rust_count = 0;
    
    while let Some(entry) = rust_stream.next().await {
        if let Ok(walk_entry) = entry {
            if walk_entry.is_file() {
                rust_count += 1;
            }
        }
    }
    
    println!("Found {} Rust files", rust_count);
    
    Ok(())
}
```

## 注意事项

- 流式 API 适合处理大型目录结构，内存使用更高效
- 并发限制应根据系统资源进行调整，避免过度消耗系统资源
- glob 模式支持标准的 glob 语法，如 `**` 表示递归匹配
- 符号链接跟随可能导致循环，使用时需谨慎
- 错误处理应妥善处理，避免因单个文件错误导致整个遍历过程失败