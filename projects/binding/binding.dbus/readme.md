# `binding-dbus`

D-Bus 协议客户端底座：Linux 上 portal 文件对话框、Secret Service、通知、logind、journal
等下游共用的总线基座。零外部 crate 依赖——线协议编解码与连接层全部基于 `std` 自实现，
连接层只使用 `std::os::unix`。

## 分层

- `marshal`：D-Bus 线协议编解码（固定头、`a(yv)` 头部字段、`Value` 值模型、签名解析、
  按类型对齐补零）。跨平台编译并测试，非 Linux 主机同样可验证协议实现。
- `conn`：Unix socket 连接、SASL `AUTH EXTERNAL`、`Hello`、方法调用、信号读取、`AddMatch`。
  仅 Linux 编译（`#[cfg(target_os = "linux")]` 门控），集成测试需要 `dbus-daemon` 运行库；
  缺失时测试硬失败，不伪绿。

## 作者面

```rust
use binding_dbus::marshal::{Signature, Value, decode, encode};

let value = Value::Dict(vec![(Value::Str("title".into()), Value::Variant(Box::new(Value::Str("hi".into()))))]);
let (bytes, signature) = encode(&value);
assert_eq!(signature.as_str(), "a{sv}");
assert_eq!(decode(&signature, &bytes).unwrap(), value);
```

Linux 会话总线：

```rust,no_run
# #[cfg(target_os = "linux")] {
use binding_dbus::conn::SessionBus;
use binding_dbus::marshal::Value;

let mut bus = SessionBus::connect()?;
let unique = bus.hello()?;
let names = bus.call("org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus", "ListNames", &Value::Struct(vec![]))?;
# let _ = (unique, names); Ok::<(), Box<dyn std::error::Error>>(())
# }
```

## 约定

- `encode` 派生签名时，空 `Array` 取元素类型 `v`（即 `av`），空 `Dict` 取 `{sv}`；
  需要精确空容器签名时由调用方显式构造非空元素或另行标注。
- 顶层 `Value::Struct` 作为 `call` 参数时展开为参数序列（签名如 `ss`）；
  需要单个结构体参数时使用嵌套 `Struct`（签名如 `(ss)`）。
- 编解码按小端实现；收到大端报文时 `conn` 层显式报错，不静默猜测。
- 会话总线地址缺失时返回明确错误，不回退系统总线。

测试：`cargo test -p binding-dbus`（`marshal` 全平台；`conn` 集成测试仅 Linux，需 `dbus-daemon`）。
