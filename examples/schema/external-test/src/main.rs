//! 外部项目测试
//! 
//! 展示外部项目如何使用 #[derive(itools::Schema)] 来自动生成 Schema 定义。

use itools::{Schema, ToSchema};

/// 产品结构体
#[derive(Schema, Debug)]
pub struct Product {
    /// 产品 ID
    pub id: u64,
    /// 产品名称
    pub name: String,
    /// 产品价格
    pub price: f64,
    /// 是否有库存
    pub in_stock: bool,
    /// 产品分类
    pub category: Category,
    /// 产品标签
    pub tags: Vec<String>,
    /// 描述
    pub description: Option<String>,
}

/// 产品分类枚举
#[derive(Schema, Debug)]
pub enum Category {
    /// 电子产品
    Electronics,
    /// 服装
    Clothing,
    /// 食品
    Food,
    /// 其他
    Other,
}

/// 订单结构体
#[derive(Schema, Debug)]
pub struct Order {
    /// 订单 ID
    pub id: String,
    /// 客户信息
    pub customer: Customer,
    /// 产品列表
    pub products: Vec<Product>,
    /// 订单状态
    pub status: OrderStatus,
    /// 下单时间
    pub created_at: String,
}

/// 客户结构体
#[derive(Schema, Debug)]
pub struct Customer {
    /// 客户 ID
    pub id: u32,
    /// 客户姓名
    pub name: String,
    /// 客户邮箱
    pub email: String,
    /// 客户地址
    pub address: Option<Address>,
}

/// 地址结构体
#[derive(Schema, Debug)]
pub struct Address {
    /// 街道
    pub street: String,
    /// 城市
    pub city: String,
    /// 州/省
    pub state: String,
    /// 邮编
    pub zip_code: String,
}

/// 订单状态枚举
#[derive(Schema, Debug)]
pub enum OrderStatus {
    /// 待处理
    Pending,
    /// 已处理
    Processing,
    /// 已发货
    Shipped,
    /// 已完成
    Completed,
    /// 已取消
    Cancelled,
}

fn main() {
    println!("=== 外部项目 Schema 测试 ===");
    println!();
    
    // 生成 Product 结构体的 Schema
    let product_schema = Product::schema();
    println!("Product Schema:");
    println!("{:#?}", product_schema);
    println!();
    
    // 生成 Category 枚举的 Schema
    let category_schema = Category::schema();
    println!("Category Schema:");
    println!("{:#?}", category_schema);
    println!();
    
    // 生成 Customer 结构体的 Schema
    let customer_schema = Customer::schema();
    println!("Customer Schema:");
    println!("{:#?}", customer_schema);
    println!();
    
    // 生成 Address 结构体的 Schema
    let address_schema = Address::schema();
    println!("Address Schema:");
    println!("{:#?}", address_schema);
    println!();
    
    // 生成 OrderStatus 枚举的 Schema
    let order_status_schema = OrderStatus::schema();
    println!("OrderStatus Schema:");
    println!("{:#?}", order_status_schema);
    println!();
    
    // 生成 Order 结构体的 Schema
    let order_schema = Order::schema();
    println!("Order Schema:");
    println!("{:#?}", order_schema);
    println!();
    
    println!("=== 测试完成 ===");
}