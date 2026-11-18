//! `binding-dbus` 线协议编解码集成测试：round-trip、精确字节、签名解析与畸形输入。
//!
//! 跨平台运行（非 Linux 主机同样验证协议实现）。

use binding_dbus::marshal::{Endian, MarshalError, Signature, Type, Value, decode, encode, parse_types};

fn roundtrip(value: Value) {
    let (bytes, signature) = encode(&value);
    let decoded = decode(&signature, &bytes).unwrap_or_else(|error| panic!("sig {} bytes {bytes:?}: {error}", signature.as_str()));
    assert_eq!(decoded, value, "sig {}", signature.as_str());
}

#[test]
fn basic_types_roundtrip() {
    roundtrip(Value::Byte(0));
    roundtrip(Value::Byte(255));
    roundtrip(Value::Bool(true));
    roundtrip(Value::Bool(false));
    roundtrip(Value::U16(u16::MAX));
    roundtrip(Value::I16(i16::MIN));
    roundtrip(Value::U32(0x0102_0304));
    roundtrip(Value::I32(-1));
    roundtrip(Value::U64(u64::MAX));
    roundtrip(Value::I64(i64::MIN));
    roundtrip(Value::F64(1.5));
    roundtrip(Value::Str("hello".to_owned()));
    roundtrip(Value::Str(String::new()));
    roundtrip(Value::ObjPath("/org/freedesktop/DBus".to_owned()));
    roundtrip(Value::Signature("a{sv}".to_owned()));
}

#[test]
fn containers_roundtrip() {
    roundtrip(Value::Array(vec![Value::Str("a".to_owned()), Value::Str("b".to_owned())]));
    roundtrip(Value::Array(Vec::new()));
    roundtrip(Value::Struct(vec![Value::U32(1), Value::Str("x".to_owned()), Value::Bool(true)]));
    roundtrip(Value::Variant(Box::new(Value::Array(vec![Value::U64(1), Value::U64(2)]))));
    roundtrip(Value::Dict(vec![
        (Value::Str("title".to_owned()), Value::Variant(Box::new(Value::Str("hi".to_owned())))),
        (Value::Str("level".to_owned()), Value::Variant(Box::new(Value::U32(3)))),
        (Value::Str("paths".to_owned()), Value::Variant(Box::new(Value::Array(vec![Value::ObjPath("/a".to_owned())])))),
    ]));
    roundtrip(Value::Dict(Vec::new()));
    roundtrip(Value::Array(vec![Value::Struct(vec![Value::Str("k".to_owned()), Value::Variant(Box::new(Value::Bool(false)))])]));
}

#[test]
fn exact_bytes_string() {
    let (bytes, signature) = encode(&Value::Str("hi".to_owned()));
    assert_eq!(signature.as_str(), "s");
    assert_eq!(bytes, vec![2, 0, 0, 0, b'h', b'i', 0]);
}

#[test]
fn exact_bytes_struct_us() {
    let (bytes, signature) = encode(&Value::Struct(vec![Value::U32(0x0102_0304), Value::Str("ab".to_owned())]));
    assert_eq!(signature.as_str(), "(us)");
    assert_eq!(bytes, vec![0x04, 0x03, 0x02, 0x01, 2, 0, 0, 0, b'a', b'b', 0]);
}

#[test]
fn exact_bytes_variant_u32_padding() {
    let (bytes, signature) = encode(&Value::Variant(Box::new(Value::U32(7))));
    assert_eq!(signature.as_str(), "v");
    assert_eq!(bytes, vec![1, b'u', 0, 0, 7, 0, 0, 0]);
}

#[test]
fn exact_bytes_dict_sv() {
    let value = Value::Dict(vec![(Value::Str("a".to_owned()), Value::Variant(Box::new(Value::U32(1))))]);
    let (bytes, signature) = encode(&value);
    assert_eq!(signature.as_str(), "a{sv}");
    assert_eq!(
        bytes,
        vec![
            16, 0, 0, 0, // 字典体长度
            1, 0, 0, 0, b'a', 0, // 键 "a"
            1, b'u', 0, // 变体内层签名 u
            0, 0, 0, // 对齐到 8 的填充
            1, 0, 0, 0, // 值 1
        ]
    );
    assert_eq!(decode(&signature, &bytes).unwrap(), value);
}

#[test]
fn exact_bytes_alignment_padding() {
    let (bytes, signature) = encode(&Value::Struct(vec![Value::Byte(1), Value::Bool(true)]));
    assert_eq!(signature.as_str(), "(yb)");
    assert_eq!(bytes, vec![1, 0, 0, 0, 1, 0, 0, 0]);

    let (bytes, signature) = encode(&Value::Array(vec![Value::U64(1), Value::U64(2)]));
    assert_eq!(signature.as_str(), "at");
    assert_eq!(bytes, vec![16, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn signature_parser() {
    assert_eq!(parse_types("a{sv}").unwrap(), vec![Type::Dict(Box::new(Type::Str), Box::new(Type::Variant))]);
    assert_eq!(parse_types("aa{sv}").unwrap(), vec![Type::Array(Box::new(Type::Dict(Box::new(Type::Str), Box::new(Type::Variant))))]);
    assert_eq!(parse_types("(a(sv))").unwrap(), vec![Type::Struct(vec![Type::Array(Box::new(Type::Struct(vec![Type::Str, Type::Variant])))])]);
    assert_eq!(parse_types("v").unwrap(), vec![Type::Variant]);
    assert_eq!(parse_types("ss").unwrap(), vec![Type::Str, Type::Str]);
    assert_eq!(parse_types("()").unwrap(), vec![Type::Struct(Vec::new())]);
}

#[test]
fn invalid_signatures_rejected() {
    for bad in ["z", "a", "(", "(s", "a{sv", "{sv}", "a{vs}", "", "a{", "s)"] {
        assert!(Signature::parse(bad).is_err(), "非法签名被接受: {bad:?}");
    }
}

#[test]
fn truncated_messages_error() {
    let sig = |s: &str| Signature::parse(s).unwrap();
    assert_eq!(decode(&sig("s"), &[2, 0, 0, 0, b'a']), Err(MarshalError::UnexpectedEnd));
    assert_eq!(decode(&sig("u"), &[1, 2]), Err(MarshalError::UnexpectedEnd));
    assert_eq!(decode(&sig("v"), &[1, b'u', 0]), Err(MarshalError::UnexpectedEnd));
    assert_eq!(decode(&sig("a{sv}"), &[16, 0, 0, 0, 1, 0]), Err(MarshalError::UnexpectedEnd));
    assert_eq!(decode(&sig("s"), &[0, 0, 0, 0, 1]), Err(MarshalError::UnterminatedString));
}

#[test]
fn trailing_bytes_rejected() {
    let signature = Signature::parse("s").unwrap();
    let mut bytes = encode(&Value::Str("hi".to_owned())).0;
    bytes.push(0);
    assert!(matches!(decode(&signature, &bytes), Err(MarshalError::TrailingBytes { .. })));
    let two = Signature::parse("ss").unwrap();
    assert!(matches!(decode(&two, &bytes), Err(MarshalError::NotSingleType { count: 2 })));
}

#[test]
fn endian_markers() {
    assert_eq!(Endian::Little.marker(), b'l');
    assert_eq!(Endian::Big.marker(), b'B');
    assert_eq!(Endian::from_marker(b'B'), Ok(Endian::Big));
    assert_eq!(Endian::from_marker(b'L'), Err(MarshalError::InvalidByteOrder(b'L')));
}
