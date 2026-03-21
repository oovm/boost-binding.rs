use tokio::{sync::AcquireError, task::JoinError};

/// 遍历错误
#[derive(Debug)]
pub enum WalkError {
    /// IO 错误
    Io(std::io::Error),
    /// Glob 模式错误
    GlobPattern(String),
    /// Glob 解析错误
    GlobParse(String),
    /// 信号量获取错误
    Acquire(AcquireError),
    /// 任务连接错误
    Join(JoinError),
    /// 其他错误
    Other(String),
}

impl std::error::Error for WalkError {}

impl std::fmt::Display for WalkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalkError::Io(err) => write!(f, "IO error: {}", err),
            WalkError::GlobPattern(err) => write!(f, "Glob pattern error: {}", err),
            WalkError::GlobParse(err) => write!(f, "Glob parse error: {}", err),
            WalkError::Acquire(err) => write!(f, "Acquire error: {}", err),
            WalkError::Join(err) => write!(f, "Join error: {}", err),
            WalkError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl From<std::io::Error> for WalkError {
    fn from(err: std::io::Error) -> Self {
        WalkError::Io(err)
    }
}



impl From<AcquireError> for WalkError {
    fn from(err: AcquireError) -> Self {
        WalkError::Acquire(err)
    }
}

impl From<JoinError> for WalkError {
    fn from(err: JoinError) -> Self {
        WalkError::Join(err)
    }
}
