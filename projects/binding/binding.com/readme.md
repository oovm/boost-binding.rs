# binding-com

Boost 外部绑定层的 **Windows COM 运行时底座**。

## 定位

所有 Windows 绑定下游（WebView、Shell、媒体组件等）都需要 apartment 初始化、`HRESULT` 错误转换与 COM 内存释放。本 crate 是这套基础设施的唯一 owner，下游不再各自手搓 `CoInitializeEx` / `CoTaskMemFree` / 错误处理。

## 公开 API

| 入口 | 职责 |
|------|------|
| `Apartment::sta()` / `Apartment::mta()` | RAII apartment 守卫；区分首次初始化、`S_FALSE` 同模型复用与 `RPC_E_CHANGED_MODE` 模式冲突（显式 `Err`，不 panic、不吞错）；`Drop` 时 `CoUninitialize` |
| `ComError` | 包装 `HRESULT`，实现 `Display` / `Error`，`code()` 返回无符号错误码 |
| `create_instance::<T>(clsid)` | `CoCreateInstance` 薄包装（默认 `CLSCTX_INPROC_SERVER`） |
| `CoTaskMem::free` / `free_string` / `free_pwstr` / `free_bstr` | 任务内存与 UTF-16 → UTF-8 字符串回收，空指针安全 |

## 使用示例

```rust
use binding_com::{Apartment, create_instance};

let _apartment = Apartment::sta()?;
let object = create_instance::<IUnknown>(&clsid)?;
```

## 平台

仅 Windows 提供能力；其它目标编译为空 crate，不导出任何符号。

## 依赖

`windows`（系统 ole32 绑定），无其它外部依赖。
