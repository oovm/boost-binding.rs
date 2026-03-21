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
    fn watch<P: Into<PathBuf>>(&mut self, path: P, recursive: bool) -> Result<(), Self::Error>;
    
    /// 停止监控指定路径
    /// 
    /// # 参数
    /// * `path` - 要停止监控的文件或目录路径
    /// 
    /// # 返回值
    /// * `Ok(())` - 停止监控成功
    /// * `Err(Self::Error)` - 停止监控失败
    fn unwatch<P: Into<PathBuf>>(&mut self, path: P) -> Result<(), Self::Error>;
    
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
        Ok(Box::new(crate::platform::windows::WindowsWatcher::new()?))
    }
    #[cfg(target_os = "linux")]
    {
        Ok(Box::new(crate::platform::linux::LinuxWatcher::new()?))
    }
    #[cfg(target_os = "macos")]
    {
        Ok(Box::new(crate::platform::macos::MacOSWatcher::new()?))
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
