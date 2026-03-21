//! macOS 平台的文件系统监控实现

use std::{path::PathBuf, sync::mpsc, thread};

use crate::{FileSystemEvent, Watcher};

/// macOS 平台的文件系统监控器
pub struct MacOSWatcher {
    event_receiver: mpsc::Receiver<FileSystemEvent>,
    watch_paths: Vec<PathBuf>,
    filters: Vec<Box<dyn crate::EventFilter>>,
}

impl MacOSWatcher {
    /// 创建新的 macOS 监控器
    pub fn new() -> Result<Self, std::io::Error> {
        let (event_sender, event_receiver) = mpsc::channel();

        Ok(Self { event_receiver, watch_paths: Vec::new(), filters: Vec::new() })
    }
}

impl Watcher for MacOSWatcher {
    type Error = std::io::Error;

    fn watch(&mut self, path: PathBuf, _recursive: bool) -> Result<(), Self::Error> {
        self.watch_paths.push(path.clone());

        // 这里将实现具体的监控逻辑
        // 暂时简化实现
        Ok(())
    }

    fn unwatch(&mut self, path: PathBuf) -> Result<(), Self::Error> {
        self.watch_paths.retain(|p| p != &path);

        // 这里将实现具体的停止监控逻辑
        Ok(())
    }

    fn add_filter(&mut self, filter: Box<dyn crate::EventFilter>) {
        self.filters.push(filter);
    }

    fn clear_filters(&mut self) {
        self.filters.clear();
    }

    fn next_event(&mut self) -> Result<Option<FileSystemEvent>, Self::Error> {
        // 从通道接收事件
        while let Ok(event) = self.event_receiver.recv() {
            // 应用过滤器
            let mut filtered = false;
            for filter in &self.filters {
                if filter.should_filter(&event) {
                    filtered = true;
                    break;
                }
            }

            if !filtered {
                return Ok(Some(event));
            }
        }

        Ok(None)
    }
}
