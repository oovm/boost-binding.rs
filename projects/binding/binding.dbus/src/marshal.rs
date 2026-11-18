//! D-Bus 线协议编解码（跨平台）。
//!
//! 实现规范的对齐与类型规则：
//! - 每种类型按其大小对齐（1/2/4/8），不足处补零；
//! - 基本类型码 `y b n q i u x t d s o g v`，容器 `a`、`()`、`a{...}`；
//! - `Variant` 先写内层值签名的 `g`，再按内层类型对齐写值；
//! - 字符串与对象路径为 `u32` 长度 + 内容 + `NUL`，签名为 `u8` 长度 + 内容 + `NUL`。
//!
//! 本模块只使用 `std`，所有平台编译并测试。编解码按小端实现；
//! 大端报文由连接层显式拒绝（见 [`crate::marshal::Endian`]）。

use std::fmt;

/// 报文字节序。固定头首字节按规范为小端 `l`、大端 `B`。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian {
    /// 小端。
    Little,
    /// 大端。
    Big,
}

impl Endian {
    /// 固定头首字节标记。
    pub const fn marker(self) -> u8 {
        match self {
            Self::Little => b'l',
            Self::Big => b'B',
        }
    }

    /// 解析固定头首字节。
    pub fn from_marker(byte: u8) -> Result<Self, MarshalError> {
        match byte {
            b'l' => Ok(Self::Little),
            b'B' => Ok(Self::Big),
            other => Err(MarshalError::InvalidByteOrder(other)),
        }
    }
}

/// 编解码错误。所有畸形输入（非法签名、截断报文、缺失 `NUL`、非法 UTF-8）返回本类型，不 panic。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MarshalError {
    /// 报文长度不足。
    UnexpectedEnd,
    /// 解码结束后仍有剩余字节。
    TrailingBytes {
        /// 实际消耗的字节数。
        used: usize,
        /// 报文总字节数。
        total: usize,
    },
    /// 签名含多个顶层类型，而调用点期望恰好一个。
    NotSingleType {
        /// 实际顶层类型个数。
        count: usize,
    },
    /// 签名非法。
    InvalidSignature(String),
    /// 固定头字节序标记非法。
    InvalidByteOrder(u8),
    /// 字符串不是合法 UTF-8。
    InvalidUtf8,
    /// 字符串或签名缺少结尾 `NUL`。
    UnterminatedString,
    /// 容器长度与内容不一致等报文结构错误。
    InvalidMessage(String),
}

impl fmt::Display for MarshalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd => f.write_str("报文截断"),
            Self::TrailingBytes { used, total } => write!(f, "解码消耗 {used} 字节，但报文共 {total} 字节"),
            Self::NotSingleType { count } => write!(f, "签名含 {count} 个顶层类型，期望恰好 1 个"),
            Self::InvalidSignature(msg) => write!(f, "非法签名: {msg}"),
            Self::InvalidByteOrder(byte) => write!(f, "非法字节序标记: {byte:#04x}"),
            Self::InvalidUtf8 => f.write_str("字符串不是合法 UTF-8"),
            Self::UnterminatedString => f.write_str("字符串缺少结尾 NUL"),
            Self::InvalidMessage(msg) => write!(f, "报文结构错误: {msg}"),
        }
    }
}

impl std::error::Error for MarshalError {}

/// D-Bus 值模型，覆盖规范全部类型。
///
/// 容器约定：[`Value::Array`] 元素必须同构；[`Value::Dict`] 为字典条目序列；
/// [`Value::Struct`] 既表示线协议结构体，也在顶层参数处表示参数序列（见 `conn` 层文档）。
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// `y`
    Byte(u8),
    /// `b`（线上为 `u32`）
    Bool(bool),
    /// `q`
    U16(u16),
    /// `n`
    I16(i16),
    /// `u`
    U32(u32),
    /// `i`
    I32(i32),
    /// `t`
    U64(u64),
    /// `x`
    I64(i64),
    /// `d`
    F64(f64),
    /// `s`
    Str(String),
    /// `o`
    ObjPath(String),
    /// `g`
    Signature(String),
    /// `a...`
    Array(Vec<Value>),
    /// `(...)`
    Struct(Vec<Value>),
    /// `v`
    Variant(Box<Value>),
    /// `a{...}`
    Dict(Vec<(Value, Value)>),
}

/// 签名解析得到的类型树。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    /// `y`
    Byte,
    /// `b`
    Boolean,
    /// `n`
    Int16,
    /// `q`
    UInt16,
    /// `i`
    Int32,
    /// `u`
    UInt32,
    /// `x`
    Int64,
    /// `t`
    UInt64,
    /// `d`
    Double,
    /// `s`
    Str,
    /// `o`
    ObjectPath,
    /// `g`
    Signature,
    /// `v`
    Variant,
    /// `a...`
    Array(Box<Type>),
    /// `(...)`
    Struct(Vec<Type>),
    /// `a{...}`
    Dict(Box<Type>, Box<Type>),
}

impl Type {
    /// 该类型在线上要求的对齐字节数。
    pub fn alignment(&self) -> usize {
        match self {
            Self::Byte | Self::Signature | Self::Variant => 1,
            Self::Int16 | Self::UInt16 => 2,
            Self::Boolean | Self::Int32 | Self::UInt32 | Self::Str | Self::ObjectPath => 4,
            Self::Int64 | Self::UInt64 | Self::Double => 8,
            Self::Array(_) | Self::Dict(..) => 4,
            Self::Struct(members) => members.iter().map(Self::alignment).max().unwrap_or(1),
        }
    }

    /// 是否为基本类型（字典键的合法类型）。
    pub fn is_basic(&self) -> bool {
        !matches!(self, Self::Variant | Self::Array(_) | Self::Struct(_) | Self::Dict(..))
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Byte => f.write_str("y"),
            Self::Boolean => f.write_str("b"),
            Self::Int16 => f.write_str("n"),
            Self::UInt16 => f.write_str("q"),
            Self::Int32 => f.write_str("i"),
            Self::UInt32 => f.write_str("u"),
            Self::Int64 => f.write_str("x"),
            Self::UInt64 => f.write_str("t"),
            Self::Double => f.write_str("d"),
            Self::Str => f.write_str("s"),
            Self::ObjectPath => f.write_str("o"),
            Self::Signature => f.write_str("g"),
            Self::Variant => f.write_str("v"),
            Self::Array(elem) => write!(f, "a{elem}"),
            Self::Dict(key, value) => {
                f.write_str("a{")?;
                write!(f, "{key}")?;
                write!(f, "{value}")?;
                f.write_str("}")
            }
            Self::Struct(members) => {
                f.write_str("(")?;
                for member in members {
                    write!(f, "{member}")?;
                }
                f.write_str(")")
            }
        }
    }
}

/// 已校验的签名字符串。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature(String);

impl Signature {
    /// 校验并构造签名（非空、不超过 255 字节、语法合法）。
    pub fn parse(text: &str) -> Result<Self, MarshalError> {
        if text.is_empty() {
            return Err(MarshalError::InvalidSignature("签名为空".to_owned()));
        }
        if text.len() > 255 {
            return Err(MarshalError::InvalidSignature("签名超过 255 字节".to_owned()));
        }
        parse_types(text)?;
        Ok(Self(text.to_owned()))
    }

    /// 解析为顶层类型序列。
    pub fn types(&self) -> Result<Vec<Type>, MarshalError> {
        parse_types(&self.0)
    }

    /// 签名字符串。
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_type(ty: Type) -> Self {
        Self(ty.to_string())
    }

    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    pub(crate) fn from_valid(text: String) -> Self {
        Self(text)
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// 按小端编码单个值，返回字节与派生签名。
///
/// 空 `Array` 派生为 `av`、空 `Dict` 派生为 `a{sv}`（round-trip 保持空容器）。
/// 产生的字节假定被放置在按该类型对齐的偏移上。
pub fn encode(value: &Value) -> (Vec<u8>, Signature) {
    let mut buf = Vec::new();
    let ty = encode_into(&mut buf, value, Endian::Little);
    (buf, Signature::from_type(ty))
}

/// 按签名解码恰好一个值，并要求消耗全部字节。
pub fn decode(signature: &Signature, bytes: &[u8]) -> Result<Value, MarshalError> {
    let types = signature.types()?;
    if types.len() != 1 {
        return Err(MarshalError::NotSingleType { count: types.len() });
    }
    let mut pos = 0;
    let value = decode_at(&types[0], bytes, &mut pos, Endian::Little)?;
    if pos != bytes.len() {
        return Err(MarshalError::TrailingBytes { used: pos, total: bytes.len() });
    }
    Ok(value)
}

pub(crate) fn encode_into(buf: &mut Vec<u8>, value: &Value, endian: Endian) -> Type {
    let ty = type_of(value);
    write_value(buf, value, endian);
    ty
}

pub(crate) fn decode_at(ty: &Type, bytes: &[u8], pos: &mut usize, endian: Endian) -> Result<Value, MarshalError> {
    let mut reader = Reader { bytes, pos: *pos, endian };
    let value = reader.value(ty)?;
    *pos = reader.pos;
    Ok(value)
}

fn type_of(value: &Value) -> Type {
    match value {
        Value::Byte(_) => Type::Byte,
        Value::Bool(_) => Type::Boolean,
        Value::U16(_) => Type::UInt16,
        Value::I16(_) => Type::Int16,
        Value::U32(_) => Type::UInt32,
        Value::I32(_) => Type::Int32,
        Value::U64(_) => Type::UInt64,
        Value::I64(_) => Type::Int64,
        Value::F64(_) => Type::Double,
        Value::Str(_) => Type::Str,
        Value::ObjPath(_) => Type::ObjectPath,
        Value::Signature(_) => Type::Signature,
        Value::Array(items) => Type::Array(Box::new(items.first().map(type_of).unwrap_or(Type::Variant))),
        Value::Struct(members) => Type::Struct(members.iter().map(type_of).collect()),
        Value::Variant(_) => Type::Variant,
        Value::Dict(entries) => match entries.first() {
            Some((key, value)) => Type::Dict(Box::new(type_of(key)), Box::new(type_of(value))),
            None => Type::Dict(Box::new(Type::Str), Box::new(Type::Variant)),
        },
    }
}

fn pad(buf: &mut Vec<u8>, align: usize) {
    while !buf.len().is_multiple_of(align) {
        buf.push(0);
    }
}

fn put_u32(buf: &mut Vec<u8>, value: u32, endian: Endian) {
    let bytes = match endian {
        Endian::Little => value.to_le_bytes(),
        Endian::Big => value.to_be_bytes(),
    };
    buf.extend_from_slice(&bytes);
}

fn put_u64(buf: &mut Vec<u8>, value: u64, endian: Endian) {
    let bytes = match endian {
        Endian::Little => value.to_le_bytes(),
        Endian::Big => value.to_be_bytes(),
    };
    buf.extend_from_slice(&bytes);
}

fn put_string(buf: &mut Vec<u8>, text: &str, endian: Endian) {
    let len = u32::try_from(text.len()).expect("字符串超过 u32 长度");
    put_u32(buf, len, endian);
    buf.extend_from_slice(text.as_bytes());
    buf.push(0);
}

fn put_signature(buf: &mut Vec<u8>, text: &str) {
    let len = u8::try_from(text.len()).expect("签名超过 255 字节");
    buf.push(len);
    buf.extend_from_slice(text.as_bytes());
    buf.push(0);
}

fn write_value(buf: &mut Vec<u8>, value: &Value, endian: Endian) {
    match value {
        Value::Byte(byte) => buf.push(*byte),
        Value::Bool(flag) => {
            pad(buf, 4);
            put_u32(buf, if *flag { 1 } else { 0 }, endian);
        }
        Value::U16(v) => {
            pad(buf, 2);
            buf.extend_from_slice(&match endian {
                Endian::Little => v.to_le_bytes(),
                Endian::Big => v.to_be_bytes(),
            });
        }
        Value::I16(v) => {
            pad(buf, 2);
            buf.extend_from_slice(&match endian {
                Endian::Little => v.to_le_bytes(),
                Endian::Big => v.to_be_bytes(),
            });
        }
        Value::U32(v) => {
            pad(buf, 4);
            put_u32(buf, *v, endian);
        }
        Value::I32(v) => {
            pad(buf, 4);
            put_u32(buf, *v as u32, endian);
        }
        Value::U64(v) => {
            pad(buf, 8);
            put_u64(buf, *v, endian);
        }
        Value::I64(v) => {
            pad(buf, 8);
            put_u64(buf, *v as u64, endian);
        }
        Value::F64(v) => {
            pad(buf, 8);
            put_u64(buf, v.to_bits(), endian);
        }
        Value::Str(text) | Value::ObjPath(text) => {
            pad(buf, 4);
            put_string(buf, text, endian);
        }
        Value::Signature(text) => put_signature(buf, text),
        Value::Array(items) => write_array(buf, items, endian),
        Value::Struct(members) => {
            let align = members.iter().map(|m| type_of(m).alignment()).max().unwrap_or(1);
            pad(buf, align);
            for member in members {
                write_value(buf, member, endian);
            }
        }
        Value::Variant(inner) => {
            put_signature(buf, &type_of(inner).to_string());
            write_value(buf, inner, endian);
        }
        Value::Dict(entries) => write_dict(buf, entries, endian),
    }
}

fn write_array(buf: &mut Vec<u8>, items: &[Value], endian: Endian) {
    pad(buf, 4);
    let len_at = buf.len();
    buf.extend_from_slice(&[0u8; 4]);
    let align = items.first().map(|v| type_of(v).alignment()).unwrap_or(1);
    pad(buf, align);
    let start = buf.len();
    for item in items {
        pad(buf, align);
        write_value(buf, item, endian);
    }
    let len = u32::try_from(buf.len() - start).expect("数组超过 u32 长度");
    let bytes = match endian {
        Endian::Little => len.to_le_bytes(),
        Endian::Big => len.to_be_bytes(),
    };
    buf[len_at..len_at + 4].copy_from_slice(&bytes);
}

fn write_dict(buf: &mut Vec<u8>, entries: &[(Value, Value)], endian: Endian) {
    pad(buf, 4);
    let len_at = buf.len();
    buf.extend_from_slice(&[0u8; 4]);
    let align = entries.first().map(|(key, value)| type_of(key).alignment().max(type_of(value).alignment())).unwrap_or(4);
    pad(buf, align);
    let start = buf.len();
    for (key, value) in entries {
        pad(buf, align);
        write_value(buf, key, endian);
        write_value(buf, value, endian);
    }
    let len = u32::try_from(buf.len() - start).expect("字典超过 u32 长度");
    let bytes = match endian {
        Endian::Little => len.to_le_bytes(),
        Endian::Big => len.to_be_bytes(),
    };
    buf[len_at..len_at + 4].copy_from_slice(&bytes);
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
    endian: Endian,
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> Result<&[u8], MarshalError> {
        let end = self.pos.checked_add(n).ok_or(MarshalError::UnexpectedEnd)?;
        let slice = self.bytes.get(self.pos..end).ok_or(MarshalError::UnexpectedEnd)?;
        self.pos = end;
        Ok(slice)
    }

    fn skip_pad(&mut self, align: usize) -> Result<(), MarshalError> {
        let rem = self.pos % align;
        if rem != 0 {
            self.take(align - rem)?;
        }
        Ok(())
    }

    fn take_array<const N: usize>(&mut self) -> Result<[u8; N], MarshalError> {
        let slice = self.take(N)?;
        let mut out = [0u8; N];
        out.copy_from_slice(slice);
        Ok(out)
    }

    fn u8(&mut self) -> Result<u8, MarshalError> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, MarshalError> {
        self.skip_pad(2)?;
        let raw = self.take_array()?;
        Ok(match self.endian {
            Endian::Little => u16::from_le_bytes(raw),
            Endian::Big => u16::from_be_bytes(raw),
        })
    }

    fn i16(&mut self) -> Result<i16, MarshalError> {
        Ok(self.u16()? as i16)
    }

    fn u32(&mut self) -> Result<u32, MarshalError> {
        self.skip_pad(4)?;
        let raw = self.take_array()?;
        Ok(match self.endian {
            Endian::Little => u32::from_le_bytes(raw),
            Endian::Big => u32::from_be_bytes(raw),
        })
    }

    fn i32(&mut self) -> Result<i32, MarshalError> {
        Ok(self.u32()? as i32)
    }

    fn u64(&mut self) -> Result<u64, MarshalError> {
        self.skip_pad(8)?;
        let raw = self.take_array()?;
        Ok(match self.endian {
            Endian::Little => u64::from_le_bytes(raw),
            Endian::Big => u64::from_be_bytes(raw),
        })
    }

    fn i64(&mut self) -> Result<i64, MarshalError> {
        Ok(self.u64()? as i64)
    }

    fn f64(&mut self) -> Result<f64, MarshalError> {
        Ok(f64::from_bits(self.u64()?))
    }

    fn string(&mut self) -> Result<String, MarshalError> {
        let len = self.u32()? as usize;
        let raw = self.take(len + 1)?;
        if raw[len] != 0 {
            return Err(MarshalError::UnterminatedString);
        }
        String::from_utf8(raw[..len].to_vec()).map_err(|_| MarshalError::InvalidUtf8)
    }

    fn signature_text(&mut self) -> Result<String, MarshalError> {
        let len = self.u8()? as usize;
        let raw = self.take(len + 1)?;
        if raw[len] != 0 {
            return Err(MarshalError::UnterminatedString);
        }
        String::from_utf8(raw[..len].to_vec()).map_err(|_| MarshalError::InvalidUtf8)
    }

    fn value(&mut self, ty: &Type) -> Result<Value, MarshalError> {
        Ok(match ty {
            Type::Byte => Value::Byte(self.u8()?),
            Type::Boolean => Value::Bool(self.u32()? != 0),
            Type::Int16 => Value::I16(self.i16()?),
            Type::UInt16 => Value::U16(self.u16()?),
            Type::Int32 => Value::I32(self.i32()?),
            Type::UInt32 => Value::U32(self.u32()?),
            Type::Int64 => Value::I64(self.i64()?),
            Type::UInt64 => Value::U64(self.u64()?),
            Type::Double => Value::F64(self.f64()?),
            Type::Str => Value::Str(self.string()?),
            Type::ObjectPath => Value::ObjPath(self.string()?),
            Type::Signature => Value::Signature(self.signature_text()?),
            Type::Variant => {
                let text = self.signature_text()?;
                let types = parse_types(&text)?;
                let [inner] = types.as_slice() else {
                    return Err(MarshalError::NotSingleType { count: types.len() });
                };
                Value::Variant(Box::new(self.value(inner)?))
            }
            Type::Array(elem) => {
                let len = self.u32()? as usize;
                self.skip_pad(elem.alignment())?;
                let end = self.pos.checked_add(len).ok_or(MarshalError::UnexpectedEnd)?;
                let mut items = Vec::new();
                while self.pos < end {
                    self.skip_pad(elem.alignment())?;
                    items.push(self.value(elem)?);
                }
                if self.pos != end {
                    return Err(MarshalError::InvalidMessage("数组元素超出声明长度".to_owned()));
                }
                Value::Array(items)
            }
            Type::Struct(members) => {
                let align = members.iter().map(Type::alignment).max().unwrap_or(1);
                self.skip_pad(align)?;
                let mut fields = Vec::new();
                for member in members {
                    fields.push(self.value(member)?);
                }
                Value::Struct(fields)
            }
            Type::Dict(key, value) => {
                let len = self.u32()? as usize;
                let align = key.alignment().max(value.alignment());
                self.skip_pad(align)?;
                let end = self.pos.checked_add(len).ok_or(MarshalError::UnexpectedEnd)?;
                let mut entries = Vec::new();
                while self.pos < end {
                    self.skip_pad(align)?;
                    let k = self.value(key)?;
                    let v = self.value(value)?;
                    entries.push((k, v));
                }
                if self.pos != end {
                    return Err(MarshalError::InvalidMessage("字典条目超出声明长度".to_owned()));
                }
                Value::Dict(entries)
            }
        })
    }
}

/// 校验并解析签名文本，返回顶层类型序列；语法非法时返回 [`MarshalError::InvalidSignature`]。
pub fn parse_types(text: &str) -> Result<Vec<Type>, MarshalError> {
    let bytes = text.as_bytes();
    let mut pos = 0;
    let mut out = Vec::new();
    while pos < bytes.len() {
        let (ty, next) = parse_type(bytes, pos)?;
        out.push(ty);
        pos = next;
    }
    Ok(out)
}

fn parse_type(bytes: &[u8], pos: usize) -> Result<(Type, usize), MarshalError> {
    let code = *bytes.get(pos).ok_or_else(|| MarshalError::InvalidSignature("签名截断".to_owned()))?;
    let pos = pos + 1;
    match code {
        b'y' => Ok((Type::Byte, pos)),
        b'b' => Ok((Type::Boolean, pos)),
        b'n' => Ok((Type::Int16, pos)),
        b'q' => Ok((Type::UInt16, pos)),
        b'i' => Ok((Type::Int32, pos)),
        b'u' => Ok((Type::UInt32, pos)),
        b'x' => Ok((Type::Int64, pos)),
        b't' => Ok((Type::UInt64, pos)),
        b'd' => Ok((Type::Double, pos)),
        b's' => Ok((Type::Str, pos)),
        b'o' => Ok((Type::ObjectPath, pos)),
        b'g' => Ok((Type::Signature, pos)),
        b'v' => Ok((Type::Variant, pos)),
        b'a' => {
            if bytes.get(pos) == Some(&b'{') {
                let p = pos + 1;
                let (key, p) = parse_type(bytes, p)?;
                if !key.is_basic() {
                    return Err(MarshalError::InvalidSignature("字典键必须是基本类型".to_owned()));
                }
                let (value, p) = parse_type(bytes, p)?;
                if bytes.get(p) != Some(&b'}') {
                    return Err(MarshalError::InvalidSignature("字典缺少 '}'".to_owned()));
                }
                Ok((Type::Dict(Box::new(key), Box::new(value)), p + 1))
            } else {
                let (elem, p) = parse_type(bytes, pos)?;
                Ok((Type::Array(Box::new(elem)), p))
            }
        }
        b'(' => {
            let mut members = Vec::new();
            let mut p = pos;
            loop {
                match bytes.get(p) {
                    None => return Err(MarshalError::InvalidSignature("结构体缺少 ')'".to_owned())),
                    Some(b')') => {
                        p += 1;
                        break;
                    }
                    Some(_) => {
                        let (ty, next) = parse_type(bytes, p)?;
                        p = next;
                        members.push(ty);
                    }
                }
            }
            Ok((Type::Struct(members), p))
        }
        other => Err(MarshalError::InvalidSignature(format!("未知类型码 '{}'", other as char))),
    }
}
