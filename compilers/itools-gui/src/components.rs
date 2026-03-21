//! 组件系统模块
//!
//! 提供 GUI 组件系统，包括基础组件和复合组件。

use crate::style::Style;
use serde::{Deserialize, Serialize};

/// 组件类型
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ComponentType {
    /// 按钮
    Button,
    /// 文本框
    TextBox,
    /// 标签
    Label,
    /// 容器
    Container,
    /// 面板
    Panel,
    /// 列表
    List,
    /// 下拉菜单
    Dropdown,
    /// 复选框
    Checkbox,
    /// 单选框
    Radio,
    /// 滑块
    Slider,
    /// 自定义组件
    Custom(String),
}

/// 组件结构体
#[derive(Debug)]
pub struct Component {
    /// 组件类型
    component_type: ComponentType,
    /// 组件ID
    id: String,
    /// 组件样式
    style: Style,
    /// 组件属性
    properties: ComponentProperties,
    /// 子组件
    children: Vec<Component>,
}

/// 组件属性
#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentProperties {
    /// 文本内容
    pub text: Option<String>,
    /// 是否启用
    pub enabled: bool,
    /// 是否可见
    pub visible: bool,
    /// 是否选中
    pub checked: bool,
    /// 最小值
    pub min: Option<f32>,
    /// 最大值
    pub max: Option<f32>,
    /// 当前值
    pub value: Option<serde_json::Value>,
    /// 选项列表
    pub options: Option<Vec<(String, String)>>,
    /// 自定义属性
    pub custom: serde_json::Value,
}

impl ComponentProperties {
    /// 创建默认组件属性
    ///
    /// # 返回值
    /// 默认组件属性
    pub fn default() -> Self {
        Self {
            text: None,
            enabled: true,
            visible: true,
            checked: false,
            min: None,
            max: None,
            value: None,
            options: None,
            custom: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
}

impl Component {
    /// 创建新的组件
    ///
    /// # 参数
    /// - `component_type`: 组件类型
    /// - `id`: 组件ID
    /// - `style`: 组件样式
    /// - `properties`: 组件属性
    ///
    /// # 返回值
    /// 组件实例
    pub fn new(component_type: ComponentType, id: &str, style: Style, properties: ComponentProperties) -> Self {
        Self { component_type, id: id.to_string(), style, properties, children: Vec::new() }
    }

    /// 添加子组件
    ///
    /// # 参数
    /// - `child`: 子组件
    pub fn add_child(&mut self, child: Component) {
        self.children.push(child);
    }

    /// 获取子组件
    ///
    /// # 参数
    /// - `id`: 子组件ID
    ///
    /// # 返回值
    /// 子组件引用
    pub fn get_child(&self, id: &str) -> Option<&Component> {
        self.children.iter().find(|child| child.id == id)
    }

    /// 获取可变子组件
    ///
    /// # 参数
    /// - `id`: 子组件ID
    ///
    /// # 返回值
    /// 可变子组件引用
    pub fn get_child_mut(&mut self, id: &str) -> Option<&mut Component> {
        self.children.iter_mut().find(|child| child.id == id)
    }

    /// 设置文本内容
    ///
    /// # 参数
    /// - `text`: 文本内容
    pub fn set_text(&mut self, text: &str) {
        self.properties.text = Some(text.to_string());
    }

    /// 设置是否启用
    ///
    /// # 参数
    /// - `enabled`: 是否启用
    pub fn set_enabled(&mut self, enabled: bool) {
        self.properties.enabled = enabled;
    }

    /// 设置是否可见
    ///
    /// # 参数
    /// - `visible`: 是否可见
    pub fn set_visible(&mut self, visible: bool) {
        self.properties.visible = visible;
    }

    /// 设置是否选中
    ///
    /// # 参数
    /// - `checked`: 是否选中
    pub fn set_checked(&mut self, checked: bool) {
        self.properties.checked = checked;
    }

    /// 设置值
    ///
    /// # 参数
    /// - `value`: 值
    pub fn set_value(&mut self, value: serde_json::Value) {
        self.properties.value = Some(value);
    }

    /// 获取组件类型
    ///
    /// # 返回值
    /// 组件类型
    pub fn component_type(&self) -> &ComponentType {
        &self.component_type
    }

    /// 获取组件ID
    ///
    /// # 返回值
    /// 组件ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 获取组件样式
    ///
    /// # 返回值
    /// 组件样式引用
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// 获取可变组件样式
    ///
    /// # 返回值
    /// 可变组件样式引用
    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    /// 获取组件属性
    ///
    /// # 返回值
    /// 组件属性引用
    pub fn properties(&self) -> &ComponentProperties {
        &self.properties
    }

    /// 获取可变组件属性
    ///
    /// # 返回值
    /// 可变组件属性引用
    pub fn properties_mut(&mut self) -> &mut ComponentProperties {
        &mut self.properties
    }

    /// 获取子组件列表
    ///
    /// # 返回值
    /// 子组件列表引用
    pub fn children(&self) -> &Vec<Component> {
        &self.children
    }

    /// 获取可变子组件列表
    ///
    /// # 返回值
    /// 可变子组件列表引用
    pub fn children_mut(&mut self) -> &mut Vec<Component> {
        &mut self.children
    }
}

/// 创建按钮组件
///
/// # 参数
/// - `id`: 组件ID
/// - `text`: 按钮文本
/// - `style`: 组件样式
///
/// # 返回值
/// 按钮组件实例
pub fn create_button(id: &str, text: &str, style: Style) -> Component {
    let mut properties = ComponentProperties::default();
    properties.text = Some(text.to_string());
    Component::new(ComponentType::Button, id, style, properties)
}

/// 创建标签组件
///
/// # 参数
/// - `id`: 组件ID
/// - `text`: 标签文本
/// - `style`: 组件样式
///
/// # 返回值
/// 标签组件实例
pub fn create_label(id: &str, text: &str, style: Style) -> Component {
    let mut properties = ComponentProperties::default();
    properties.text = Some(text.to_string());
    Component::new(ComponentType::Label, id, style, properties)
}

/// 创建文本框组件
///
/// # 参数
/// - `id`: 组件ID
/// - `value`: 文本框值
/// - `style`: 组件样式
///
/// # 返回值
/// 文本框组件实例
pub fn create_text_box(id: &str, value: &str, style: Style) -> Component {
    let mut properties = ComponentProperties::default();
    properties.value = Some(serde_json::Value::String(value.to_string()));
    Component::new(ComponentType::TextBox, id, style, properties)
}

/// 创建容器组件
///
/// # 参数
/// - `id`: 组件ID
/// - `style`: 组件样式
///
/// # 返回值
/// 容器组件实例
pub fn create_container(id: &str, style: Style) -> Component {
    let properties = ComponentProperties::default();
    Component::new(ComponentType::Container, id, style, properties)
}
