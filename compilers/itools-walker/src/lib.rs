#![warn(missing_docs)]

//! iTools 异步目录遍历模块
//!
//! 提供异步的目录遍历功能，支持递归遍历目录结构、并发处理和 glob 模式匹配。

use futures::stream::{self, Stream};
use std::{num::NonZeroU32, path::Path};
use tokio::{
    fs,
    sync::{Semaphore, mpsc},
};

mod errors;
#[cfg(feature = "glob")]
mod glob;

pub use errors::WalkError;

/// 遍历选项
#[derive(Debug, Clone)]
pub struct WalkOptions {
    /// 是否递归遍历子目录
    pub recursive: bool,
    /// 是否跟随符号链接
    pub follow_symlinks: bool,
    /// 包含的 glob 模式
    #[cfg(feature = "glob")]
    pub include_patterns: Vec<String>,
    /// 排除的 glob 模式（如 gitignore）
    #[cfg(feature = "glob")]
    pub exclude_patterns: Vec<String>,
    /// 通道大小
    pub channel_size: NonZeroU32,
    /// 并发度限制
    pub concurrency_limit: NonZeroU32,
}

impl Default for WalkOptions {
    fn default() -> Self {
        Self {
            recursive: true,
            follow_symlinks: false,
            #[cfg(feature = "glob")]
            include_patterns: Vec::new(),
            #[cfg(feature = "glob")]
            exclude_patterns: Vec::new(),
            concurrency_limit: NonZeroU32::new(10).unwrap(),
            channel_size: NonZeroU32::new(64).unwrap(),
        }
    }
}

/// 遍历条目
#[derive(Debug, Clone)]
pub struct WalkEntry {
    /// 条目路径
    pub path: std::path::PathBuf,
    /// 文件元数据
    pub metadata: std::fs::Metadata,
}

impl WalkEntry {
    /// 是否为目录
    pub fn is_dir(&self) -> bool {
        self.metadata.is_dir()
    }

    /// 是否为文件
    pub fn is_file(&self) -> bool {
        self.metadata.is_file()
    }

    /// 是否为符号链接
    pub fn is_symlink(&self) -> bool {
        self.path.symlink_metadata().map(|m| m.file_type().is_symlink()).unwrap_or(false)
    }
}

/// 异步目录遍历器
#[derive(Debug, Clone)]
pub struct Walker {
    options: WalkOptions,
    #[cfg(feature = "glob")]
    glob_set: Option<glob::GlobSet>,
}

impl Walker {
    /// 创建新的遍历器
    pub fn new() -> Result<Self, WalkError> {
        Self::with_options(WalkOptions::default())
    }

    /// 使用指定选项创建新的遍历器
    pub fn with_options(options: WalkOptions) -> Result<Self, WalkError> {
        #[cfg(feature = "glob")]
        let mut walker = Self { options, glob_set: None };
        #[cfg(not(feature = "glob"))]
        let walker = Self { options };

        #[cfg(feature = "glob")]
        walker.build_glob_set()?;
        Ok(walker)
    }

    /// 构建 glob 模式集合
    #[cfg(feature = "glob")]
    fn build_glob_set(&mut self) -> Result<(), WalkError> {
        self.glob_set = Some(glob::build_glob_set(&self.options.include_patterns, &self.options.exclude_patterns)?);
        Ok(())
    }

    /// 检查路径是否应该被包含
    #[cfg(feature = "glob")]
    fn should_include(&self, path: &Path) -> bool {
        glob::should_include(path, &self.glob_set)
    }

    /// 检查路径是否应该被包含（无 glob 功能）
    #[cfg(not(feature = "glob"))]
    fn should_include(&self, _path: &Path) -> bool {
        true
    }

    /// 遍历目录
    ///
    /// 返回一个 Stream，用于流式获取遍历结果，避免一次性收集所有条目导致的内存使用过高问题。
    ///
    /// # Examples
    /// ```rust
    /// use itools_walker::Walker;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let walker = Walker::new().unwrap();
    ///     let mut stream = walker.walk(".").await.unwrap();
    ///
    ///     while let Some(entry) = stream.next().await {
    ///         match entry {
    ///             Ok(walk_entry) => println!("Found: {:?}", walk_entry.path),
    ///             Err(e) => println!("Error: {:?}", e),
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn walk<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<impl Stream<Item = Result<WalkEntry, WalkError>> + use<P>, WalkError> {
        let path = path.as_ref().to_path_buf();
        let (tx, rx) = mpsc::channel(self.options.channel_size.get() as usize);

        let semaphore = Semaphore::new(self.options.concurrency_limit.get() as usize);

        let self_clone = self.clone();

        tokio::spawn(async move {
            let _ = self_clone.walk_recursive(&path, tx, &semaphore).await;
        });

        let stream = stream::unfold(rx, |mut rx| async move {
            match rx.recv().await {
                Some(item) => Some((item, rx)),
                None => None,
            }
        });

        Ok(stream)
    }

    /// 递归遍历目录
    async fn walk_recursive(
        &self,
        path: &Path,
        tx: mpsc::Sender<Result<WalkEntry, WalkError>>,
        semaphore: &Semaphore,
    ) -> Result<(), WalkError> {
        if !self.should_include(path) {
            return Ok(());
        }

        let metadata =
            if self.options.follow_symlinks { fs::metadata(path).await? } else { fs::symlink_metadata(path).await? };

        let is_dir = metadata.is_dir();
        let entry = WalkEntry { path: path.to_path_buf(), metadata };

        if tx.send(Ok(entry)).await.is_err() {
            return Ok(());
        }

        if is_dir && self.options.recursive {
            let mut dir_entries = fs::read_dir(path).await?;

            while let Some(entry) = dir_entries.next_entry().await? {
                let entry_path = entry.path();

                if self.should_include(&entry_path) {
                    let permit = semaphore.acquire().await?;
                    let tx_clone = tx.clone();
                    let semaphore = semaphore;
                    let self_clone = self.clone();

                    Box::pin(self_clone.walk_recursive(&entry_path, tx_clone, &semaphore)).await?;
                    drop(permit);
                }
            }
        }

        Ok(())
    }

    /// 遍历目录并对每个条目执行回调
    pub async fn walk_with<P: AsRef<Path>, F: Fn(&WalkEntry) -> Result<(), WalkError>>(
        &self,
        path: P,
        mut callback: F,
    ) -> Result<(), WalkError> {
        let path = path.as_ref();
        let semaphore = Semaphore::new(self.options.concurrency_limit.get() as usize);

        self.walk_with_recursive(path, &mut callback, &semaphore).await
    }

    /// 递归遍历目录并执行回调
    async fn walk_with_recursive<F: Fn(&WalkEntry) -> Result<(), WalkError>>(
        &self,
        path: &Path,
        callback: &mut F,
        semaphore: &Semaphore,
    ) -> Result<(), WalkError> {
        if !self.should_include(path) {
            return Ok(());
        }

        let metadata =
            if self.options.follow_symlinks { fs::metadata(path).await? } else { fs::symlink_metadata(path).await? };

        let is_dir = metadata.is_dir();
        let entry = WalkEntry { path: path.to_path_buf(), metadata };

        callback(&entry)?;

        if is_dir && self.options.recursive {
            let mut dir_entries = fs::read_dir(path).await?;

            while let Some(entry) = dir_entries.next_entry().await? {
                let entry_path = entry.path();

                if self.should_include(&entry_path) {
                    let permit = semaphore.acquire().await?;
                    let semaphore = semaphore;
                    let self_clone = self.clone();

                    Box::pin(self_clone.walk_with_recursive(&entry_path, callback, &semaphore)).await?;
                    drop(permit);
                }
            }
        }

        Ok(())
    }
}

/// 便捷函数：遍历目录
pub async fn walk<P: AsRef<Path>>(path: P) -> Result<impl Stream<Item = Result<WalkEntry, WalkError>>, WalkError> {
    Walker::new()?.walk(path).await
}

/// 便捷函数：带选项遍历目录
pub async fn walk_with_options<P: AsRef<Path>>(
    path: P,
    options: WalkOptions,
) -> Result<impl Stream<Item = Result<WalkEntry, WalkError>>, WalkError> {
    Walker::with_options(options)?.walk(path).await
}
