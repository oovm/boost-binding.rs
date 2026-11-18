//! Linux 会话总线连接层：Unix socket、SASL、方法调用与信号读取。
//!
//! 仅 `std::os::unix`，无外部 crate。认证使用 `AUTH EXTERNAL`（uid 取自 `/proc/self/status`
//! 的 `Uid:` 行，按规范以十进制字符串逐字节的十六进制承载）。
//! 报文编解码复用 [`crate::marshal`]，本层只负责收发与头部组装。

use std::fmt;
use std::io::{Read, Write};
use std::os::linux::net::SocketAddrExt;
use std::os::unix::net::{SocketAddr, UnixStream};

use crate::marshal::{self, Endian, MarshalError, Signature, Type, Value};

const FIELD_PATH: u8 = 1;
const FIELD_INTERFACE: u8 = 2;
const FIELD_MEMBER: u8 = 3;
const FIELD_ERROR_NAME: u8 = 4;
const FIELD_REPLY_SERIAL: u8 = 5;
const FIELD_DESTINATION: u8 = 6;
const FIELD_SENDER: u8 = 7;
const FIELD_SIGNATURE: u8 = 8;
const FIELD_UNIX_FDS: u8 = 9;

const MAX_MESSAGE_BYTES: usize = 16 * 1024 * 1024;

/// 消息类型（固定头第二字节）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageKind {
    /// 方法调用（1）。
    MethodCall,
    /// 方法应答（2）。
    MethodReturn,
    /// 错误应答（3）。
    Error,
    /// 信号（4）。
    Signal,
}

impl MessageKind {
    /// 线上字节。
    pub fn code(self) -> u8 {
        match self {
            Self::MethodCall => 1,
            Self::MethodReturn => 2,
            Self::Error => 3,
            Self::Signal => 4,
        }
    }

    /// 解析线上字节。
    pub fn from_code(value: u8) -> Result<Self, ConnError> {
        match value {
            1 => Ok(Self::MethodCall),
            2 => Ok(Self::MethodReturn),
            3 => Ok(Self::Error),
            4 => Ok(Self::Signal),
            other => Err(ConnError::Protocol(format!("未知消息类型 {other}"))),
        }
    }
}

/// 一条已解析的报文。
#[derive(Debug)]
pub struct Message {
    /// 消息类型。
    pub kind: MessageKind,
    /// 固定头 flags 字节。
    pub flags: u8,
    /// 发送方 serial。
    pub serial: u32,
    /// `PATH` 头部字段。
    pub path: Option<String>,
    /// `INTERFACE` 头部字段。
    pub interface: Option<String>,
    /// `MEMBER` 头部字段。
    pub member: Option<String>,
    /// `ERROR_NAME` 头部字段。
    pub error_name: Option<String>,
    /// `REPLY_SERIAL` 头部字段。
    pub reply_serial: Option<u32>,
    /// `DESTINATION` 头部字段。
    pub destination: Option<String>,
    /// `SENDER` 头部字段。
    pub sender: Option<String>,
    /// `SIGNATURE` 头部字段。
    pub signature: Option<Signature>,
    /// `UNIX_FDS` 头部字段计数（本层不接收 fd 传递）。
    pub unix_fds: Option<u32>,
    /// 按签名解码出的 body 顶层值序列。
    pub body: Vec<Value>,
}

/// 连接层错误。
#[derive(Debug)]
pub enum ConnError {
    /// 未设置 `DBUS_SESSION_BUS_ADDRESS`。
    NoSessionAddress,
    /// 总线地址非法。
    Address(String),
    /// 底层 I/O 错误。
    Io(std::io::Error),
    /// SASL 握手失败。
    Auth(String),
    /// 协议层非法输入。
    Protocol(String),
    /// 编解码错误。
    Marshal(MarshalError),
}

impl fmt::Display for ConnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSessionAddress => f.write_str("DBUS_SESSION_BUS_ADDRESS 未设置；会话总线缺失时不回退系统总线"),
            Self::Address(msg) => write!(f, "总线地址非法: {msg}"),
            Self::Io(e) => write!(f, "连接层 I/O 错误: {e}"),
            Self::Auth(msg) => write!(f, "SASL 认证失败: {msg}"),
            Self::Protocol(msg) => write!(f, "协议错误: {msg}"),
            Self::Marshal(e) => write!(f, "编解码错误: {e}"),
        }
    }
}

impl std::error::Error for ConnError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Marshal(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConnError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<MarshalError> for ConnError {
    fn from(error: MarshalError) -> Self {
        Self::Marshal(error)
    }
}

/// 远端返回的 D-Bus 错误（error name 加可选描述文本）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbusError {
    /// 错误名，例如 `org.freedesktop.DBus.Error.ServiceUnknown`。
    pub name: String,
    /// 错误描述文本。
    pub message: String,
}

impl fmt::Display for DbusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.message.is_empty() { f.write_str(&self.name) } else { write!(f, "{}: {}", self.name, self.message) }
    }
}

impl std::error::Error for DbusError {}

impl From<ConnError> for DbusError {
    fn from(error: ConnError) -> Self {
        Self { name: "binding-dbus.Error.Transport".to_owned(), message: error.to_string() }
    }
}

/// 会话总线连接。
pub struct SessionBus {
    stream: UnixStream,
    serial: u32,
    unique_name: String,
}

impl SessionBus {
    /// 按 `DBUS_SESSION_BUS_ADDRESS` 连接会话总线；未设置时返回 [`ConnError::NoSessionAddress`]，
    /// 不回退系统总线。
    pub fn connect() -> Result<Self, ConnError> {
        let address = std::env::var("DBUS_SESSION_BUS_ADDRESS").map_err(|_| ConnError::NoSessionAddress)?;
        Self::connect_to_address(&address)
    }

    /// 连接显式给出的总线地址（支持 `unix:path=` 与 `unix:abstract=`）。
    pub fn connect_to_address(address: &str) -> Result<Self, ConnError> {
        let target = resolve_unix_address(address)?;
        let stream = UnixStream::connect_addr(&target)?;
        let mut bus = Self { stream, serial: 0, unique_name: String::new() };
        bus.authenticate()?;
        Ok(bus)
    }

    /// `Hello` 之后记录的总线唯一名（未 `Hello` 时为空串）。
    pub fn unique_name(&self) -> &str {
        &self.unique_name
    }

    /// 调用 `org.freedesktop.DBus.Hello`，返回并记录唯一名。
    pub fn hello(&mut self) -> Result<String, DbusError> {
        let value = self.call("org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus", "Hello", &Value::Struct(Vec::new()))?;
        match value {
            Value::Str(name) => {
                self.unique_name = name.clone();
                Ok(name)
            }
            other => {
                Err(DbusError { name: "binding-dbus.Error.Protocol".to_owned(), message: format!("Hello 应答不是字符串: {other:?}") })
            }
        }
    }

    /// 同步方法调用。
    ///
    /// `args` 为顶层参数值：`Value::Struct` 非空时展开为参数序列（签名如 `ss`），
    /// 空 `Struct` 表示无参数，其余值作为单个顶层参数。
    /// 应答解码：0 个值返回空 `Struct`，1 个值直接返回，多个值打包为 `Struct`。
    /// 传输与协议错误折叠进 [`DbusError`]（name 为 `binding-dbus.Error.Transport`）。
    pub fn call(&mut self, destination: &str, path: &str, interface: &str, member: &str, args: &Value) -> Result<Value, DbusError> {
        let (body, signature) = encode_args(args);
        let mut fields = vec![
            (FIELD_PATH, Value::ObjPath(path.to_owned())),
            (FIELD_INTERFACE, Value::Str(interface.to_owned())),
            (FIELD_MEMBER, Value::Str(member.to_owned())),
            (FIELD_DESTINATION, Value::Str(destination.to_owned())),
        ];
        if let Some(signature) = signature {
            fields.push((FIELD_SIGNATURE, Value::Signature(signature.as_str().to_owned())));
        }
        let serial = self.send(MessageKind::MethodCall, fields, &body)?;
        loop {
            let message = self.read_message()?;
            if message.reply_serial != Some(serial) {
                continue;
            }
            return match message.kind {
                MessageKind::MethodReturn => Ok(collapse_values(message.body)),
                MessageKind::Error => Err(decode_dbus_error(message.body)),
                other => Err(DbusError {
                    name: "binding-dbus.Error.Protocol".to_owned(),
                    message: format!("serial {serial} 的应答类型异常: {other:?}"),
                }),
            };
        }
    }

    /// 调用 `org.freedesktop.DBus.AddMatch` 订阅信号规则。
    pub fn add_match(&mut self, rule: &str) -> Result<(), DbusError> {
        self.call("org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus", "AddMatch", &Value::Str(rule.to_owned()))?;
        Ok(())
    }

    /// 读取下一条报文（信号循环入口）。与本次调用无关的报文由 [`SessionBus::call`] 内部跳过。
    pub fn read_message(&mut self) -> Result<Message, ConnError> {
        let head = self.read_bytes(16)?;
        let endian = Endian::from_marker(head[0])?;
        let kind = MessageKind::from_code(head[1])?;
        let flags = head[2];
        if head[3] != 1 {
            return Err(ConnError::Protocol(format!("不支持的协议版本 {}", head[3])));
        }
        let body_len = read_u32(&head[4..8], endian) as usize;
        let serial = read_u32(&head[8..12], endian);
        let fields_len = read_u32(&head[12..16], endian) as usize;
        let fields_bytes = self.read_bytes(fields_len)?;
        let mut pos = 0;
        let fields = marshal::decode_at(&header_fields_type(), &fields_bytes, &mut pos, endian)?;
        if pos != fields_bytes.len() {
            return Err(ConnError::Protocol("头部字段数组存在尾部字节".to_owned()));
        }
        let pad = (8 - (16 + fields_len) % 8) % 8;
        self.read_bytes(pad)?;
        let body = self.read_bytes(body_len)?;

        let mut message = Message {
            kind,
            flags,
            serial,
            path: None,
            interface: None,
            member: None,
            error_name: None,
            reply_serial: None,
            destination: None,
            sender: None,
            signature: None,
            unix_fds: None,
            body: Vec::new(),
        };
        if let Value::Array(items) = fields {
            for item in items {
                let Value::Struct(pair) = item else { continue };
                let [Value::Byte(code), Value::Variant(inner)] = pair.as_slice() else { continue };
                let inner = inner.as_ref();
                match *code {
                    FIELD_PATH => message.path = as_string(inner),
                    FIELD_INTERFACE => message.interface = as_string(inner),
                    FIELD_MEMBER => message.member = as_string(inner),
                    FIELD_ERROR_NAME => message.error_name = as_string(inner),
                    FIELD_REPLY_SERIAL => message.reply_serial = as_u32(inner),
                    FIELD_DESTINATION => message.destination = as_string(inner),
                    FIELD_SENDER => message.sender = as_string(inner),
                    FIELD_SIGNATURE => {
                        if let Value::Signature(text) = inner {
                            message.signature = Some(Signature::parse(text)?);
                        }
                    }
                    FIELD_UNIX_FDS => message.unix_fds = as_u32(inner),
                    _ => {}
                }
            }
        }
        let types = match &message.signature {
            Some(signature) => signature.types()?,
            None => Vec::new(),
        };
        let mut pos = 0;
        let mut values = Vec::new();
        for ty in &types {
            values.push(marshal::decode_at(ty, &body, &mut pos, endian)?);
        }
        if pos != body.len() {
            return Err(ConnError::Protocol("body 长度与签名不一致".to_owned()));
        }
        message.body = values;
        Ok(message)
    }

    fn authenticate(&mut self) -> Result<(), ConnError> {
        let uid = self_uid()?;
        let identity = external_identity(uid);
        // 认证规范要求客户端首字节为 NUL（协议版本标识）。它与 `AUTH` 必须合并进同一次写入：
        // 分两次写会让守护进程在读到一个孤立的 NUL 后立刻回一行 `REJECTED` 机制列表，
        // 也会在缺少 NUL 时按协议违规直接断开（表现为 `ECONNRESET`）。
        self.stream.write_all(format!("\0AUTH EXTERNAL {identity}\r\n").as_bytes())?;
        self.stream.flush()?;
        let response = self.read_line()?;
        if !response.starts_with("OK") {
            return Err(ConnError::Auth(format!("AUTH EXTERNAL 被拒绝: {response}")));
        }
        // 可选能力：`AGREE_UNIX_FD` 或 `ERROR` 都不影响后续 `BEGIN`。
        self.send_line("NEGOTIATE_UNIX_FD")?;
        let _ = self.read_line()?;
        self.send_line("BEGIN")?;
        Ok(())
    }

    fn send_line(&mut self, line: &str) -> Result<(), ConnError> {
        self.stream.write_all(line.as_bytes())?;
        self.stream.write_all(b"\r\n")?;
        self.stream.flush()?;
        Ok(())
    }

    fn read_line(&mut self) -> Result<String, ConnError> {
        let mut bytes = Vec::new();
        let mut byte = [0u8; 1];
        loop {
            let n = self.stream.read(&mut byte)?;
            if n == 0 {
                return Err(ConnError::Protocol("SASL 握手流提前结束".to_owned()));
            }
            if byte[0] == b'\n' {
                break;
            }
            if byte[0] != b'\r' {
                bytes.push(byte[0]);
            }
        }
        String::from_utf8(bytes).map_err(|e| ConnError::Protocol(format!("SASL 响应非法 UTF-8: {e}")))
    }

    fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, ConnError> {
        if n > MAX_MESSAGE_BYTES {
            return Err(ConnError::Protocol(format!("报文过大: {n} 字节")));
        }
        let mut buf = vec![0u8; n];
        self.stream.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn send(&mut self, kind: MessageKind, fields: Vec<(u8, Value)>, body: &[u8]) -> Result<u32, ConnError> {
        self.serial += 1;
        let serial = self.serial;
        let body_len = u32::try_from(body.len()).map_err(|_| ConnError::Protocol("body 超过 u32 长度".to_owned()))?;
        let fields_value = Value::Array(
            fields.into_iter().map(|(code, value)| Value::Struct(vec![Value::Byte(code), Value::Variant(Box::new(value))])).collect(),
        );
        let mut buf = vec![Endian::Little.marker(), kind.code(), 0, 1];
        buf.extend_from_slice(&body_len.to_le_bytes());
        buf.extend_from_slice(&serial.to_le_bytes());
        marshal::encode_into(&mut buf, &fields_value, Endian::Little);
        while !buf.len().is_multiple_of(8) {
            buf.push(0);
        }
        buf.extend_from_slice(body);
        self.stream.write_all(&buf)?;
        Ok(serial)
    }
}

fn header_fields_type() -> Type {
    Type::Array(Box::new(Type::Struct(vec![Type::Byte, Type::Variant])))
}

fn encode_args(args: &Value) -> (Vec<u8>, Option<Signature>) {
    let mut buf = Vec::new();
    match args {
        Value::Struct(members) if !members.is_empty() => {
            let mut signature = String::new();
            for member in members {
                signature.push_str(&marshal::encode_into(&mut buf, member, Endian::Little).to_string());
            }
            (buf, Some(Signature::from_valid(signature)))
        }
        Value::Struct(_) => (buf, None),
        other => {
            let ty = marshal::encode_into(&mut buf, other, Endian::Little);
            (buf, Some(Signature::from_valid(ty.to_string())))
        }
    }
}

fn collapse_values(mut values: Vec<Value>) -> Value {
    match values.len() {
        0 => Value::Struct(Vec::new()),
        1 => values.pop().expect("长度已检查"),
        _ => Value::Struct(values),
    }
}

fn decode_dbus_error(body: Vec<Value>) -> DbusError {
    let name = match body.first() {
        Some(Value::Str(name)) => name.clone(),
        _ => "org.freedesktop.DBus.Error.Failed".to_owned(),
    };
    let message = match body.get(1) {
        Some(Value::Str(text)) => text.clone(),
        _ => String::new(),
    };
    DbusError { name, message }
}

fn as_string(value: &Value) -> Option<String> {
    match value {
        Value::Str(text) | Value::ObjPath(text) => Some(text.clone()),
        _ => None,
    }
}

fn as_u32(value: &Value) -> Option<u32> {
    match value {
        Value::U32(number) => Some(*number),
        _ => None,
    }
}

fn read_u32(bytes: &[u8], endian: Endian) -> u32 {
    let mut raw = [0u8; 4];
    raw.copy_from_slice(bytes);
    match endian {
        Endian::Little => u32::from_le_bytes(raw),
        Endian::Big => u32::from_be_bytes(raw),
    }
}

/// 解析总线地址中的 unix 传输（`path=` 或 `abstract=` 键）；地址不含 unix 传输或缺少键时返回 [`ConnError::Address`]。
pub fn resolve_unix_address(address: &str) -> Result<SocketAddr, ConnError> {
    for transport in address.split(';') {
        let Some(rest) = transport.strip_prefix("unix:") else { continue };
        let mut path: Option<String> = None;
        let mut abstract_name: Option<String> = None;
        for key in rest.split(',') {
            if let Some(value) = key.strip_prefix("path=") {
                path = Some(value.to_owned());
            } else if let Some(value) = key.strip_prefix("abstract=") {
                abstract_name = Some(value.to_owned());
            }
        }
        if let Some(path) = path {
            return SocketAddr::from_pathname(path).map_err(ConnError::Io);
        }
        if let Some(name) = abstract_name {
            return SocketAddr::from_abstract_name(name).map_err(ConnError::Io);
        }
        return Err(ConnError::Address(format!("unix 传输缺少 path= 或 abstract= 键: {transport}")));
    }
    Err(ConnError::Address(format!("地址不含 unix 传输: {address}")))
}

/// 读取 `/proc/self/status` 的 `Uid:` 行，返回当前进程真实 uid（SASL `AUTH EXTERNAL` 凭据来源）。
pub fn self_uid() -> Result<u32, ConnError> {
    let status = std::fs::read_to_string("/proc/self/status")?;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("Uid:")
            && let Some(real) = rest.split_whitespace().next()
        {
            return real.parse::<u32>().map_err(|e| ConnError::Protocol(format!("无法解析 Uid 行: {e}")));
        }
    }
    Err(ConnError::Protocol("/proc/self/status 缺少 Uid 行".to_owned()))
}

/// `AUTH EXTERNAL` 的 payload：uid 十进制字符串逐字节的十六进制。
///
/// D-Bus 的 SASL 用十六进制承载该身份（例如 uid `1000` → `31303030`），
/// 不是 base64。多包一层编码会被守护进程判为非法 hex。
pub fn external_identity(uid: u32) -> String {
    uid.to_string().bytes().map(|byte| format!("{byte:02x}")).collect()
}
