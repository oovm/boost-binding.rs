# iTools Notify

🔔 iTools Notify 是 iTools 项目的文件系统变更通知库，不依赖外部 C 库，为 iTools 生态系统提供跨平台的文件系统监控能力。

## 特性

- ✨ 跨平台支持：支持 Windows、Linux 和 macOS
- ✨ 无外部依赖：不依赖 notify 或 inotify-sys 等 C 库
- ✨ 简洁易用的 API：提供清晰的接口，支持文件和目录的监控
- ✨ 多种事件类型：支持创建、修改、删除、重命名等事件
- ✨ 事件过滤机制：支持基于路径的事件过滤
- ✨ 高效的事件处理：减少资源消耗，提高性能

## 使用方法

### 基本用法

```rust
use itools_notify::{new_watcher, FileSystemEvent};
use std::path::Path;

fn main() {
    // 创建监控器
    let mut watcher = new_watcher().unwrap();
    
    // 监控当前目录
    let current_dir = Path::new(".");
    watcher.watch(current_dir, true).unwrap();
    
    println!("开始监控目录: {}", current_dir.display());
    
    // 接收事件
    loop {
        match watcher.next_event() {
            Ok(Some(event)) => {
                match event {
                    FileSystemEvent::Create(path) => println!("创建: {}", path.display()),
                    FileSystemEvent::Modify(path) => println!("修改: {}", path.display()),
                    FileSystemEvent::Delete(path) => println!("删除: {}", path.display()),
                    FileSystemEvent::Rename(old_path, new_path) => {
                        println!("重命名: {} -> {}", old_path.display(), new_path.display());
                    }
                }
            }
            Ok(None) => break,
            Err(e) => println!("错误: {:?}", e),
        }
    }
}
```

### 使用事件过滤器

```rust
use itools_notify::{new_watcher, PathFilter};
use std::path::Path;

fn main() {
    // 创建监控器
    let mut watcher = new_watcher().unwrap();
    
    // 添加过滤器，只监控 .rs 文件
    let filter = PathFilter::new(vec!".rs".to_string(), vec!);
    watcher.add_filter(Box::new(filter));
    
    // 监控当前目录
    let current_dir = Path::new(".");
    watcher.watch(current_dir, true).unwrap();
    
    println!("开始监控目录: {} (只监控 .rs 文件)", current_dir.display());
    
    // 接收事件
    loop {
        match watcher.next_event() {
            Ok(Some(event)) => {
                println!("事件: {:?}", event);
            }
            Ok(None) => break,
            Err(e) => println!("错误: {:?}", e),
        }
    }
}
```

## 注意事项

- 不支持网络文件系统的监控
- 监控大型目录时可能会消耗较多资源
- 不同平台的事件处理可能存在差异
- 平台实现：Windows 使用 ReadDirectoryChangesW API，Linux 使用 inotify API，macOS 使用 FSEvents API