//! `binding-com` 集成测试：apartment 守卫、错误路径、实例创建与任务内存回收。
//!
//! 仅 Windows 目标运行（本 crate 在非 Windows 上编译为空）。

#![cfg(windows)]

use binding_com::{Apartment, ApartmentState, CoTaskMem, create_instance};
use windows::Win32::System::Com::CoTaskMemAlloc;
use windows::core::{BSTR, GUID, IUnknown, PWSTR};

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
