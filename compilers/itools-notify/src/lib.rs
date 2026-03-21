//! itools-notify - 自研的文件系统变更通知库
//! 
//! 提供跨平台的文件系统监控能力，不依赖外部 C 库。

use std::path::PathBuf;

/// 文件系统事件类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileSystemEvent {
    /// 文件或目录创建
    Create(PathBuf),
    /// 文件或目录修改
    Modify(PathBuf),
    /// 文件或目录删除
    Delete(PathBuf),
    /// 文件或目录重命名
    Rename(PathBuf, PathBuf),
}

/// 文件系统事件过滤器
pub trait EventFilter {
    /// 检查事件是否应该被过滤
    /// 
    /// # 参数
    /// * `event` - 要检查的文件系统事件
    /// 
    /// # 返回值
    /// * `true` - 事件应该被过滤
    /// * `false` - 事件应该被保留
    fn should_filter(&self, event: &FileSystemEvent) -> bool;
}

/// 简单的路径过滤器
#[derive(Debug, Clone)]
pub struct PathFilter {
    include_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
}

impl PathFilter {
    /// 创建新的路径过滤器
    pub fn new(include_patterns: Vec<String>, exclude_patterns: Vec<String>) -> Self {
        Self {
            include_patterns,
            exclude_patterns,
        }
    }
}

impl EventFilter for PathFilter {
    fn should_filter(&self, event: &FileSystemEvent) -> bool {
        let path = match event {
            FileSystemEvent::Create(path) => path,
            FileSystemEvent::Modify(path) => path,
            FileSystemEvent::Delete(path) => path,
            FileSystemEvent::Rename(_, path) => path,
        };
        
        let path_str = path.to_string_lossy();
        
        // 检查是否在排除列表中
        for pattern in &self.exclude_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }
        
        // 检查是否在包含列表中
        if !self.include_patterns.is_empty() {
            for pattern in &self.include_patterns {
                if path_str.contains(pattern) {
                    return false;
                }
            }
            return true;
        }
        
        false
    }
}

/// 文件系统监控器 trait
pub trait Watcher {
    /// 错误类型
    type Error: std::error::Error;
    
    /// 开始监控指定路径
    /// 
    /// # 参数
    /// * `path` - 要监控的文件或目录路径
    /// * `recursive` - 是否递归监控子目录
    /// 
    /// # 返回值
    /// * `Ok(())` - 监控成功
    /// * `Err(Self::Error)` - 监控失败
    fn watch(&mut self, path: PathBuf, recursive: bool) -> Result<(), Self::Error>;
    
    /// 停止监控指定路径
    /// 
    /// # 参数
    /// * `path` - 要停止监控的文件或目录路径
    /// 
    /// # 返回值
    /// * `Ok(())` - 停止监控成功
    /// * `Err(Self::Error)` - 停止监控失败
    fn unwatch(&mut self, path: PathBuf) -> Result<(), Self::Error>;
    
    /// 添加事件过滤器
    /// 
    /// # 参数
    /// * `filter` - 要添加的事件过滤器
    fn add_filter(&mut self, filter: Box<dyn EventFilter>);
    
    /// 移除所有事件过滤器
    fn clear_filters(&mut self);
    
    /// 接收下一个文件系统事件
    /// 
    /// # 返回值
    /// * `Ok(Option<FileSystemEvent>)` - 成功接收到事件，`None` 表示监控已停止
    /// * `Err(Self::Error)` - 接收事件失败
    fn next_event(&mut self) -> Result<Option<FileSystemEvent>, Self::Error>;
}

/// 创建适合当前平台的文件系统监控器
/// 
/// # 返回值
/// * `Ok(Box<dyn Watcher>)` - 成功创建监控器
/// * `Err(String)` - 创建监控器失败
pub fn new_watcher() -> Result<Box<dyn Watcher<Error = Box<dyn std::error::Error>>>, String> {
    #[cfg(windows)]
    {
        match crate::platform::windows::WindowsWatcher::new() {
            Ok(watcher) => Ok(Box::new(watcher)),
            Err(e) => Err(e.to_string()),
        }
    }
    #[cfg(target_os = "linux")]
    {
        match crate::platform::linux::LinuxWatcher::new() {
            Ok(watcher) => Ok(Box::new(watcher)),
            Err(e) => Err(e.to_string()),
        }
    }
    #[cfg(target_os = "macos")]
    {
        match crate::platform::macos::MacOSWatcher::new() {
            Ok(watcher) => Ok(Box::new(watcher)),
            Err(e) => Err(e.to_string()),
        }
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    {
        Err("Unsupported platform".to_string())
    }
}

/// 平台特定的实现
mod platform {
    #[cfg(windows)]
    pub mod windows;
    
    #[cfg(target_os = "linux")]
    pub mod linux;
    
    #[cfg(target_os = "macos")]
    pub mod macos;
}
