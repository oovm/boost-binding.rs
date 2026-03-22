//! ITools Schema 示例
//! 
//! 展示如何使用 #[derive(itools::Schema)] 来自动生成 Schema 定义。

use itools::{Schema, ToSchema};

/// 用户结构体
#[derive(Schema, Debug)]
pub struct User {
    /// 用户名
    pub name: String,
    /// 年龄
    pub age: u32,
    /// 邮箱
    pub email: Option<String>,
    /// 角色
    pub role: Role,
    /// 标签
    pub tags: Vec<String>,
}

/// 角色枚举
#[derive(Schema, Debug)]
pub enum Role {
    /// 管理员
    Admin,
    /// 用户
    User,
    /// 访客
    Guest,
}

fn main() {
    // 生成 User 结构体的 Schema
    let user_schema = User::schema();
    println!("User Schema:");
    println!("{:#?}", user_schema);
    println!();
    
    // 生成 Role 枚举的 Schema
    let role_schema = Role::schema();
    println!("Role Schema:");
    println!("{:#?}", role_schema);
    println!();
    
    // 测试基本类型的 Schema
    println!("String Schema:");
    println!("{:#?}", String::schema());
    println!();
    
    println!("u32 Schema:");
    println!("{:#?}", u32::schema());
    println!();
    
    println!("Option<String> Schema:");
    println!("{:#?}", Option::<String>::schema());
    println!();
    
    println!("Vec<String> Schema:");
    println!("{:#?}", Vec::<String>::schema());
}
