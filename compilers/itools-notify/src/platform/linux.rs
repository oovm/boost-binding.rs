//! Linux 平台的文件系统监控实现

use std::{ffi::CString, os::unix::ffi::OsStrExt, path::PathBuf, sync::mpsc, thread};

use libc::{
    IN_ACCESS, IN_ALL_EVENTS, IN_CREATE, IN_DELETE, IN_MODIFY, IN_MOVED_FROM, IN_MOVED_TO, close, inotify_add_watch,
    inotify_init, inotify_rm_watch, read,
};

use crate::{FileSystemEvent, Watcher};

/// Linux 平台的文件系统监控器
pub struct LinuxWatcher {
    event_receiver: mpsc::Receiver<FileSystemEvent>,
    watch_paths: Vec<PathBuf>,
    inotify_fd: Option<i32>,
    filters: Vec<Box<dyn crate::EventFilter>>,
}

impl LinuxWatcher {
    /// 创建新的 Linux 监控器
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // 初始化 inotify
        let inotify_fd = unsafe { inotify_init() };
        if inotify_fd < 0 {
            return Err(std::io::Error::last_os_error().into());
        }

        let (event_sender, event_receiver) = mpsc::channel();
        let inotify_fd_clone = inotify_fd;

        // 启动监控线程
        thread::spawn(move || {
            let mut buffer = [0; 4096];

            loop {
                let bytes_read = unsafe { read(inotify_fd_clone, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len()) };
                if bytes_read < 0 {
                    break;
                }

                // 处理事件
                let mut offset = 0;
                while offset < bytes_read as usize {
                    let event = unsafe { &*(buffer.as_ptr().add(offset) as *const libc::inotify_event) };
                    let name_length = event.len as usize;

                    if name_length > 0 {
                        let name = unsafe {
                            std::ffi::OsString::from_vec(
                                buffer[offset + std::mem::size_of::<libc::inotify_event>()
                                    ..offset + std::mem::size_of::<libc::inotify_event>() + name_length]
                                    .to_vec(),
                            )
                        };
                        // 这里需要根据 watch descriptor 找到对应的路径
                        // 暂时简化处理
                        let path = PathBuf::from(name);

                        if event.mask & IN_CREATE != 0 {
                            event_sender.send(FileSystemEvent::Create(path)).ok();
                        }
                        else if event.mask & IN_DELETE != 0 {
                            event_sender.send(FileSystemEvent::Delete(path)).ok();
                        }
                        else if event.mask & IN_MODIFY != 0 {
                            event_sender.send(FileSystemEvent::Modify(path)).ok();
                        }
                        else if event.mask & IN_MOVED_FROM != 0 {
                            // 暂存旧名称，等待新名称
                        }
                        else if event.mask & IN_MOVED_TO != 0 {
                            // 处理重命名事件
                        }
                    }

                    offset += std::mem::size_of::<libc::inotify_event>() + name_length;
                }
            }
        });

        Ok(Self { event_receiver, watch_paths: Vec::new(), inotify_fd: Some(inotify_fd), filters: Vec::new() })
    }
}

impl Watcher for LinuxWatcher {
    type Error = Box<dyn std::error::Error>;

    fn watch<P: Into<PathBuf>>(&mut self, path: P, recursive: bool) -> Result<(), Self::Error> {
        let path = path.into();
        self.watch_paths.push(path.clone());

        if let Some(fd) = self.inotify_fd {
            let path_str = CString::new(path.as_os_str().as_bytes())?;
            let mask = IN_ALL_EVENTS;

            let wd = unsafe { inotify_add_watch(fd, path_str.as_ptr(), mask) };
            if wd < 0 {
                return Err(std::io::Error::last_os_error().into());
            }
        }

        // 这里将实现递归监控逻辑
        Ok(())
    }

    fn unwatch<P: Into<PathBuf>>(&mut self, path: P) -> Result<(), Self::Error> {
        let path = path.into();
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

impl Drop for LinuxWatcher {
    fn drop(&mut self) {
        if let Some(fd) = self.inotify_fd {
            unsafe { close(fd) };
        }
    }
}
