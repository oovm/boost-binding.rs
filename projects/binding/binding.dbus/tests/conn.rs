//! `binding-dbus` 会话总线连接层集成测试：真实 `dbus-daemon` 往返、地址解析与 SASL base64。
//!
//! 连接层与 `dbus-daemon` 均为 Linux-only，非 Linux 目标整个文件不参与编译。

#![cfg(target_os = "linux")]

use binding_dbus::conn::{ConnError, SessionBus, base64, resolve_unix_address, self_uid};
use binding_dbus::marshal::Value;
use std::io::BufRead;
use std::io::BufReader;
use std::process::{Child, Command, Stdio};

struct Daemon {
    child: Child,
    address: String,
}

fn spawn_daemon() -> Daemon {
    let mut child = Command::new("dbus-daemon")
        .args(["--session", "--print-address", "--disable-syslog"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("dbus-daemon 必须可用：运行库缺失时测试硬失败，不伪绿");
    let address =
        BufReader::new(child.stdout.take().expect("stdout 已接管道")).lines().next().expect("dbus-daemon 应打印总线地址").expect("读取地址行");
    Daemon { child, address }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn hello_list_names_and_unix_user() {
    let daemon = spawn_daemon();
    let mut bus = SessionBus::connect_to_address(&daemon.address).expect("连接会话总线");
    let name = bus.hello().expect("Hello");
    assert!(name.starts_with(":1."), "唯一名形状异常: {name}");

    let names = bus
        .call("org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus", "ListNames", &Value::Struct(Vec::new()))
        .expect("ListNames");
    let Value::Array(items) = names else { panic!("ListNames 应答应为数组") };
    assert!(items.iter().any(|item| *item == Value::Str("org.freedesktop.DBus".to_owned())), "ListNames 缺少总线自身: {items:?}");

    let uid_value = bus
        .call("org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus", "GetConnectionUnixUser", &Value::Str(name.clone()))
        .expect("GetConnectionUnixUser");
    assert_eq!(uid_value, Value::U32(self_uid().expect("读取自身 uid")));

    let features = bus
        .call(
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus.Properties",
            "Get",
            &Value::Struct(vec![Value::Str("org.freedesktop.DBus".to_owned()), Value::Str("Features".to_owned())]),
        )
        .expect("Properties.Get");
    assert!(matches!(features, Value::Variant(_)), "Properties.Get 应答应为 Variant: {features:?}");
}

#[test]
fn address_parsing() {
    assert!(resolve_unix_address("unix:path=/run/user/1000/bus").is_ok());
    assert!(resolve_unix_address("unix:abstract=/tmp/dbus-x,guid=deadbeef").is_ok());
    assert!(matches!(resolve_unix_address("tcp:host=localhost"), Err(ConnError::Address(_))));
    assert!(matches!(resolve_unix_address("unix:guid=abc"), Err(ConnError::Address(_))));
}

#[test]
fn base64_matches_sasl_expectation() {
    assert_eq!(base64(b"31303030"), "MzEzMDMwMzA=");
    assert_eq!(base64(b"3130333132"), "MzEzMDMzMTMyMg==");
    assert_eq!(base64(b""), "");
}
