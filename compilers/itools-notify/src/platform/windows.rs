//! Windows 平台的文件系统监控实现

use std::{
    ffi::{OsStr, OsString},
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::PathBuf,
    sync::mpsc,
    thread,
};

use winapi::{
    shared::minwindef::{BOOL, DWORD},
    um::{
        fileapi::{CreateFileW, OPEN_EXISTING},
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        winbase::{FILE_FLAG_BACKUP_SEMANTICS, ReadDirectoryChangesW},
        winnt::{
            FILE_ACTION_ADDED, FILE_ACTION_MODIFIED, FILE_ACTION_REMOVED, FILE_ACTION_RENAMED_NEW_NAME,
            FILE_ACTION_RENAMED_OLD_NAME, FILE_GENERIC_READ, FILE_NOTIFY_INFORMATION, FILE_SHARE_DELETE, FILE_SHARE_READ,
            FILE_SHARE_WRITE, HANDLE,
        },
    },
};

use crate::{FileSystemEvent, Watcher};

/// 包装 HANDLE 以实现 Send trait，因为 Windows HANDLE 可以安全在线程间传递
#[allow(dead_code)]
struct SendableHandle(HANDLE);

/// 为 SendableHandle 实现 Send trait
unsafe impl Send for SendableHandle {}

/// 为 SendableHandle 实现 Sync trait
unsafe impl Sync for SendableHandle {}

/// Windows 平台的文件系统监控器
pub struct WindowsWatcher {
    event_receiver: mpsc::Receiver<FileSystemEvent>,
    event_sender: mpsc::Sender<FileSystemEvent>,
    watch_paths: Vec<PathBuf>,
    filters: Vec<Box<dyn crate::EventFilter>>,
}

impl WindowsWatcher {
    /// 创建新的 Windows 监控器
    pub fn new() -> Result<Self, std::io::Error> {
        let (event_sender, event_receiver) = mpsc::channel();

        Ok(Self { event_receiver, event_sender, watch_paths: Vec::new(), filters: Vec::new() })
    }

    /// 将 PathBuf 转换为 Windows 宽字符串
    fn path_to_wstring<P: AsRef<OsStr>>(&self, path: P) -> Vec<u16> {
        path.as_ref().encode_wide().chain(std::iter::once(0)).collect()
    }

    /// 打开目录句柄
    fn open_directory(&self, path: &PathBuf) -> Result<HANDLE, std::io::Error> {
        let path_wstr = self.path_to_wstring(path);
        let handle = unsafe {
            CreateFileW(
                path_wstr.as_ptr(),
                FILE_GENERIC_READ,
                FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS,
                std::ptr::null_mut(),
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            return Err(std::io::Error::last_os_error());
        }

        Ok(handle)
    }
}

impl Watcher for WindowsWatcher {
    type Error = std::io::Error;

    fn watch(&mut self, path: PathBuf, recursive: bool) -> Result<(), Self::Error> {
        self.watch_paths.push(path.clone());

        // 打开目录句柄
        let handle = self.open_directory(&path)?;

        // 启动监控线程
        let event_sender = self.event_sender.clone();
        let path_clone = path.clone();

        // 将 HANDLE 转换为 isize 以在线程间安全传递
        let handle_ptr = handle as isize;

        thread::spawn(move || {
            // 将 isize 转换回 HANDLE
            let handle = handle_ptr as HANDLE;
            let mut buffer = [0u8; 4096];

            loop {
                let mut bytes_returned: DWORD = 0;
                let success = unsafe {
                    ReadDirectoryChangesW(
                        handle,
                        buffer.as_mut_ptr() as *mut _,
                        buffer.len() as DWORD,
                        recursive as BOOL,
                        0x00000001 | 0x00000002 | 0x00000004 | 0x00000008 | 0x00000010 | 0x00000020,
                        &mut bytes_returned,
                        std::ptr::null_mut(),
                        None,
                    )
                };

                if success == 0 {
                    break;
                }

                // 处理事件
                let mut offset = 0;
                while offset < bytes_returned as usize {
                    let info = unsafe { &*(buffer.as_ptr().add(offset) as *const FILE_NOTIFY_INFORMATION) };
                    let file_name = unsafe {
                        let file_name_ptr = info.FileName.as_ptr();
                        let file_name_slice = std::slice::from_raw_parts(file_name_ptr, info.FileNameLength as usize / 2);
                        OsString::from_wide(file_name_slice)
                    };
                    let full_path = path_clone.join(file_name);

                    match info.Action {
                        FILE_ACTION_ADDED => {
                            event_sender.send(FileSystemEvent::Create(full_path)).ok();
                        }
                        FILE_ACTION_REMOVED => {
                            event_sender.send(FileSystemEvent::Delete(full_path)).ok();
                        }
                        FILE_ACTION_MODIFIED => {
                            event_sender.send(FileSystemEvent::Modify(full_path)).ok();
                        }
                        FILE_ACTION_RENAMED_OLD_NAME => {
                            // 暂存旧名称，等待新名称
                        }
                        FILE_ACTION_RENAMED_NEW_NAME => {
                            // 处理重命名事件
                        }
                        _ => {}
                    }

                    offset += info.NextEntryOffset as usize;
                }
            }

            unsafe { CloseHandle(handle) };
        });

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
        match self.event_receiver.recv() {
            Ok(event) => {
                // 应用过滤器
                let mut filtered = false;
                for filter in &self.filters {
                    if filter.should_filter(&event) {
                        filtered = true;
                        break;
                    }
                }

                if !filtered { Ok(Some(event)) } else { self.next_event() }
            }
            Err(_) => Ok(None),
        }
    }
}
