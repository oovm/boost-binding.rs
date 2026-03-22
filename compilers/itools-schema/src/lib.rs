//! ITools Schema - Schema 定义与构造模块
//! 
//! 提供统一的 Schema 定义能力，支持：
//! - 数据结构 Schema 定义
//! - Schema 构造
//! - 兼容 serde metadata
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// Schema 类型定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SchemaType {
    /// 字符串类型
    String,
    /// 整数类型
    Integer,
    /// 浮点数类型
    Number,
    /// 布尔类型
    Boolean,
    /// 数组类型
    Array,
    /// 对象类型
    Object,
    /// 空值类型
    Null,
}

impl Default for SchemaType {
    /// 创建默认的 Schema 类型（Object）
    fn default() -> Self {
        Self::Object
    }
}

/// Schema 定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Schema {
    /// Schema 标题
    pub title: Option<String>,
    /// Schema 描述
    pub description: Option<String>,
    /// Schema 类型
    pub schema_type: SchemaType,
    /// 属性定义（仅 Object 类型）
    pub properties: Option<std::collections::BTreeMap<String, Schema>>,
    /// 必需属性列表
    pub required: Option<Vec<String>>,
    /// 数组元素类型（仅 Array 类型）
    pub items: Option<Box<Schema>>,
    /// 枚举值
    pub enum_values: Option<Vec<String>>,
    /// 默认值（字符串表示）
    pub default: Option<String>,
    /// 示例值（字符串表示）
    pub example: Option<String>,
    /// 是否可为空
    pub nullable: bool,
    /// 只读
    pub read_only: bool,
    /// 只写
    pub write_only: bool,
    /// 最小值（Number/Integer）
    pub minimum: Option<f64>,
    /// 最大值（Number/Integer）
    pub maximum: Option<f64>,
    /// 最小长度（String）
    pub min_length: Option<usize>,
    /// 最大长度（String）
    pub max_length: Option<usize>,
    /// 正则模式（String）
    pub pattern: Option<String>,
    /// 格式（String）
    pub format: Option<String>,
    /// 多个类型（用于 one_of）
    pub one_of: Option<Vec<Schema>>,
    /// 多个类型（用于 any_of）
    pub any_of: Option<Vec<Schema>>,
    /// 多个类型（用于 all_of）
    pub all_of: Option<Vec<Schema>>,
}

impl Default for Schema {
    /// 创建默认的 Schema（Object 类型）
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            schema_type: SchemaType::Object,
            properties: None,
            required: None,
            items: None,
            enum_values: None,
            default: None,
            example: None,
            nullable: false,
            read_only: false,
            write_only: false,
            minimum: None,
            maximum: None,
            min_length: None,
            max_length: None,
            pattern: None,
            format: None,
            one_of: None,
            any_of: None,
            all_of: None,
        }
    }
}

impl Schema {
    /// 创建字符串 Schema
    pub fn string() -> Self {
        Self { schema_type: SchemaType::String, ..Default::default() }
    }

    /// 创建整数 Schema
    pub fn integer() -> Self {
        Self { schema_type: SchemaType::Integer, ..Default::default() }
    }

    /// 创建数字 Schema
    pub fn number() -> Self {
        Self { schema_type: SchemaType::Number, ..Default::default() }
    }

    /// 创建布尔 Schema
    pub fn boolean() -> Self {
        Self { schema_type: SchemaType::Boolean, ..Default::default() }
    }

    /// 创建数组 Schema
    pub fn array(items: Schema) -> Self {
        Self { schema_type: SchemaType::Array, items: Some(Box::new(items)), ..Default::default() }
    }

    /// 创建对象 Schema
    pub fn object() -> Self {
        Self { schema_type: SchemaType::Object, properties: Some(std::collections::BTreeMap::new()), ..Default::default() }
    }

    /// 设置标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置描述
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// 添加属性
    pub fn property(mut self, name: impl Into<String>, schema: Schema) -> Self {
        let properties = self.properties.get_or_insert_with(std::collections::BTreeMap::new);
        properties.insert(name.into(), schema);
        self
    }

    /// 设置必需属性
    pub fn required(mut self, fields: Vec<&str>) -> Self {
        self.required = Some(fields.into_iter().map(String::from).collect());
        self
    }

    /// 设置枚举值
    pub fn enum_values(mut self, values: Vec<impl Into<String>>) -> Self {
        self.enum_values = Some(values.into_iter().map(Into::into).collect());
        self
    }

    /// 设置默认值
    pub fn with_default(mut self, value: impl Into<String>) -> Self {
        self.default = Some(value.into());
        self
    }

    /// 设置示例值
    pub fn example(mut self, value: impl Into<String>) -> Self {
        self.example = Some(value.into());
        self
    }

    /// 设置格式
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// 设置正则模式
    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    /// 设置最小长度
    pub fn min_length(mut self, len: usize) -> Self {
        self.min_length = Some(len);
        self
    }

    /// 设置最大长度
    pub fn max_length(mut self, len: usize) -> Self {
        self.max_length = Some(len);
        self
    }

    /// 设置最小值
    pub fn minimum(mut self, min: f64) -> Self {
        self.minimum = Some(min);
        self
    }

    /// 设置最大值
    pub fn maximum(mut self, max: f64) -> Self {
        self.maximum = Some(max);
        self
    }

    /// 设置可为空
    pub fn nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }

    /// 设置只读
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// 设置只写
    pub fn write_only(mut self, write_only: bool) -> Self {
        self.write_only = write_only;
        self
    }

    /// 设置 oneOf
    pub fn one_of(mut self, schemas: Vec<Schema>) -> Self {
        self.one_of = Some(schemas);
        self
    }

    /// 设置 anyOf
    pub fn any_of(mut self, schemas: Vec<Schema>) -> Self {
        self.any_of = Some(schemas);
        self
    }

    /// 设置 allOf
    pub fn all_of(mut self, schemas: Vec<Schema>) -> Self {
        self.all_of = Some(schemas);
        self
    }

}

/// Schema 生成 trait
///
/// 实现此 trait 的类型可以自动生成 Schema。
pub trait ToSchema {
    /// 生成 Schema 定义
    fn schema() -> Schema;
}

impl ToSchema for String {
    fn schema() -> Schema {
        Schema::string()
    }
}

impl ToSchema for i64 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for i32 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for i16 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for i8 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for u64 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for u32 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for u16 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for u8 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for f64 {
    fn schema() -> Schema {
        Schema::number()
    }
}

impl ToSchema for f32 {
    fn schema() -> Schema {
        Schema::number()
    }
}

impl ToSchema for bool {
    fn schema() -> Schema {
        Schema::boolean()
    }
}

impl<T: ToSchema> ToSchema for Vec<T> {
    fn schema() -> Schema {
        Schema::array(T::schema())
    }
}

impl<T: ToSchema> ToSchema for Option<T> {
    fn schema() -> Schema {
        T::schema().nullable(true)
    }
}

