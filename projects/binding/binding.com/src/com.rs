//! Windows COM 运行时底座实现：apartment 守卫、错误包装、实例创建与任务内存回收。

use std::error::Error;
use std::ffi::c_void;
use std::fmt::{self, Display};
use std::marker::PhantomData;

use windows::Win32::Foundation::{RPC_E_CHANGED_MODE, S_FALSE};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize,
};
use windows::core::{BSTR, GUID, HRESULT, Interface, PWSTR};

/// COM 调用失败错误，包装系统 `HRESULT`。
///
/// 展示时委托 `HRESULT` 自带的系统消息格式化：有描述则为「消息 (0x…)」，
/// 否则仅输出十六进制错误码。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComError(HRESULT);

impl ComError {
    /// 从 `HRESULT` 构造错误。
    #[inline]
    pub fn new(code: HRESULT) -> Self {
        Self(code)
    }

    /// 原始 `HRESULT`。
    #[inline]
    pub fn hresult(&self) -> HRESULT {
        self.0
    }

    /// 以无符号 32 位读取错误码（保留位模式，失败码高位为 1）。
    #[inline]
    pub fn code(&self) -> u32 {
        self.0.0 as u32
    }
}

impl Display for ComError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = self.0.message();
        if message.is_empty() { write!(f, "{}", self.0) } else { write!(f, "{} ({})", message, self.0) }
    }
}

impl Error for ComError {}

impl From<HRESULT> for ComError {
    fn from(code: HRESULT) -> Self {
        Self(code)
    }
}

/// `Apartment` 初始化失败的三种结果之一。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApartmentState {
    /// 该线程已按**不同**并发模型初始化（`RPC_E_CHANGED_MODE`）。
    ///
    /// 这是需要调用方显式处置的状态，绝不 panic，也不静默吞掉。
    ModeConflict,
    /// `CoInitializeEx` 返回其它失败 `HRESULT`。
    Failed(ComError),
}

impl Display for ApartmentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ModeConflict => write!(f, "线程已按不同并发模型初始化 (0x{:08X})", RPC_E_CHANGED_MODE.0 as u32),
            Self::Failed(error) => write!(f, "COM 初始化失败: {}", error),
        }
    }
}

impl Error for ApartmentState {}

/// RAII apartment 守卫，持有 `CoInitializeEx` 的一次成功初始化。
///
/// apartment 亲和性：COM 并发模型按线程生效，`CoUninitialize` 必须发生在
/// 初始化它的同一线程上，因此本类型刻意不是 `Send` / `Sync`，守卫随作用域
/// 在原地释放。`Drop` 时调用 `CoUninitialize`，与构造时的一次成功初始化
/// （含 `S_FALSE` 复用计数）严格配对。
#[derive(Debug)]
pub struct Apartment {
    already: bool,
    /// 线程亲和标记：裸指针语义使类型自动成为 `!Send` + `!Sync`。
    _affinity: PhantomData<*const ()>,
}

impl Apartment {
    /// 以单线程单元（STA）初始化当前线程。
    ///
    /// - 首次初始化成功：返回持有 `CoUninitialize` 守卫的 `Ok`；
    /// - 该线程已按 STA 初始化（`S_FALSE`）：返回可用守卫且 `is_already_initialized()` 为 `true`；
    /// - 该线程已按 MTA 初始化：返回 `Err(ApartmentState::ModeConflict)`。
    pub fn sta() -> Result<Self, ApartmentState> {
        Self::initialize(COINIT_APARTMENTTHREADED)
    }

    /// 以多线程单元（MTA）初始化当前线程，结果语义同 [`Apartment::sta`]。
    pub fn mta() -> Result<Self, ApartmentState> {
        Self::initialize(COINIT_MULTITHREADED)
    }

    /// 本守卫是否复用了线程上既有的同模型初始化（即 `CoInitializeEx` 返回 `S_FALSE`）。
    #[inline]
    pub fn is_already_initialized(&self) -> bool {
        self.already
    }

    fn initialize(coinit: windows::Win32::System::Com::COINIT) -> Result<Self, ApartmentState> {
        let code = unsafe { CoInitializeEx(None, coinit) };
        if code == RPC_E_CHANGED_MODE {
            return Err(ApartmentState::ModeConflict);
        }
        if code == S_FALSE {
            return Ok(Self { already: true, _affinity: PhantomData });
        }
        if code.is_ok() {
            return Ok(Self { already: false, _affinity: PhantomData });
        }
        Err(ApartmentState::Failed(code.into()))
    }
}

impl Drop for Apartment {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

/// 创建进程内 COM 对象（`CLSCTX_INPROC_SERVER`）的薄包装。
///
/// 调用线程必须已处于某个 apartment（见 [`Apartment`]），否则返回
/// `CO_E_NOTINITIALIZED` 对应的 [`ComError`]。
pub fn create_instance<T: Interface>(clsid: &GUID) -> Result<T, ComError> {
    unsafe { CoCreateInstance(clsid as *const GUID, None, CLSCTX_INPROC_SERVER) }.map_err(|error| ComError::new(error.code()))
}

/// `CoTaskMem` 家族的所有权收口：读取并释放由任务内存分配器管理的缓冲。
pub struct CoTaskMem;

impl CoTaskMem {
    /// 释放 `CoTaskMemAlloc` / `CoTaskMemRealloc` 分配的内存块；空指针安全。
    ///
    /// # Safety
    /// `ptr` 必须为空，或来自任务内存分配器且未被释放过。
    pub unsafe fn free(ptr: *mut c_void) {
        if !ptr.is_null() {
            unsafe { CoTaskMemFree(Some(ptr as *const c_void)) };
        }
    }

    /// 读取以 NUL 结尾的 UTF-16 缓冲、转 UTF-8（有损）并释放；空指针返回空串。
    ///
    /// # Safety
    /// `ptr` 必须为空，或来自 `CoTaskMemAlloc` 且指向 NUL 结尾的 UTF-16 序列。
    pub unsafe fn free_string(ptr: *mut u16) -> String {
        let text = unsafe { read_utf16_until_nul(ptr) };
        unsafe { Self::free(ptr.cast()) };
        text
    }

    /// [`CoTaskMem::free_string`] 的 [`PWSTR`] 形态。
    ///
    /// # Safety
    /// 同 [`CoTaskMem::free_string`]。
    pub unsafe fn free_pwstr(pwstr: PWSTR) -> String {
        unsafe { Self::free_string(pwstr.0) }
    }

    /// [`CoTaskMem::free_string`] 的 [`BSTR`] 形态。
    ///
    /// 注意：`BSTR` 自带长度头，COM 规定必须以 `SysFreeString` 释放；
    /// 本方法按此契约交由 `BSTR` 的 `Drop` 完成，不走 `CoTaskMemFree`。
    ///
    /// # Safety
    /// `bstr` 必须是有效的 `BSTR`（或空）。移交所有权后不得再使用原句柄。
    pub unsafe fn free_bstr(bstr: BSTR) -> String {
        let text = String::from_utf16_lossy(&bstr);
        drop(bstr);
        text
    }
}

/// 从 NUL 结尾的 UTF-16 指针读取内容并做有损转换；空指针返回空串。
///
/// # Safety
/// `ptr` 必须为空，或指向以 NUL 结尾的合法可读 UTF-16 区域。
unsafe fn read_utf16_until_nul(ptr: *const u16) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let mut length = 0usize;
    while unsafe { *ptr.add(length) } != 0 {
        length += 1;
    }
    let slice = unsafe { core::slice::from_raw_parts(ptr, length) };
    String::from_utf16_lossy(slice)
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows::Win32::System::Com::CoTaskMemAlloc;
    use windows::core::IUnknown;

    #[test]
    fn sta_repeated_on_same_thread_reports_already() {
        let first = Apartment::sta().expect("首次 STA 初始化应成功");
        assert!(!first.is_already_initialized());
        let second = Apartment::sta().expect("同线程重复 STA 不应报错");
        assert!(second.is_already_initialized());
        // 逆序释放，两次 CoUninitialize 与两次成功初始化配对
        drop(second);
        drop(first);
    }

    #[test]
    fn mta_then_sta_conflicts_on_dedicated_thread() {
        // 专用线程自洽：不受测试进程其它线程 apartment 状态影响
        let handle = std::thread::spawn(|| {
            let mta = Apartment::mta().expect("专用线程 MTA 初始化应成功");
            assert!(!mta.is_already_initialized());
            let error = Apartment::sta().expect_err("MTA 线程上 STA 必须返回模式冲突而非 panic");
            assert_eq!(error, ApartmentState::ModeConflict);
            drop(mta);
        });
        handle.join().expect("模式冲突线程不应 panic");
    }

    #[test]
    fn create_instance_unknown_clsid_returns_readable_error() {
        // 错误路径稳定可测：未注册 CLSID 必然失败，且不依赖桌面会话组件
        let handle = std::thread::spawn(|| {
            let _guard = Apartment::sta().expect("专用线程 STA 初始化应成功");
            let clsid = GUID::from_values(0xDEAD_BEEF, 0x0001, 0x0002, [3, 4, 5, 6, 7, 8, 9, 10]);
            let error = create_instance::<IUnknown>(&clsid).expect_err("未注册 CLSID 应返回 ComError");
            assert_ne!(error.code(), 0, "错误码必须可读");
            assert!(!error.to_string().is_empty(), "Display 必须非空");
        });
        handle.join().expect("未知 CLSID 线程不应 panic");
    }

    #[test]
    fn co_task_mem_string_roundtrip() {
        let source: Vec<u16> = "boost-com-测试".encode_utf16().chain(std::iter::once(0)).collect();
        let buffer = unsafe { CoTaskMemAlloc(source.len() * std::mem::size_of::<u16>()) }.cast::<u16>();
        assert!(!buffer.is_null());
        unsafe { core::ptr::copy_nonoverlapping(source.as_ptr(), buffer, source.len()) };
        let text = unsafe { CoTaskMem::free_string(buffer) };
        assert_eq!(text, "boost-com-测试");
    }

    #[test]
    fn null_pointers_are_safe() {
        assert_eq!(unsafe { CoTaskMem::free_string(core::ptr::null_mut()) }, "");
        assert_eq!(unsafe { CoTaskMem::free_pwstr(PWSTR::null()) }, "");
        unsafe { CoTaskMem::free(core::ptr::null_mut()) };
    }

    #[test]
    fn invalid_utf16_does_not_panic() {
        // 0xD800 是缺少后随编码单元的前导代理，必须走有损转换
        let source: Vec<u16> = vec![0xD800, 0x0041, 0];
        let buffer = unsafe { CoTaskMemAlloc(source.len() * std::mem::size_of::<u16>()) }.cast::<u16>();
        assert!(!buffer.is_null());
        unsafe { core::ptr::copy_nonoverlapping(source.as_ptr(), buffer, source.len()) };
        let text = unsafe { CoTaskMem::free_string(buffer) };
        assert!(text.contains('A'));
    }

    #[test]
    fn bstr_roundtrip() {
        let bstr = BSTR::from("hello");
        let text = unsafe { CoTaskMem::free_bstr(bstr) };
        assert_eq!(text, "hello");
    }
}
