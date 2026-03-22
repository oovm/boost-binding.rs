use itools::Schema;
use itools::ToSchema;

#[Schema]
pub struct User {
    /// 用户名
    pub name: String,
    /// 年龄
    pub age: u32,
    /// 邮箱
    pub email: Option<String>,
}

#[Schema]
pub enum Role {
    /// 管理员
    Admin,
    /// 用户
    User,
    /// 访客
    Guest,
}

#[test]
fn test_schema_derive() {
    let user_schema = User::schema();
    println!("User schema: {:?}", user_schema);
    
    let role_schema = Role::schema();
    println!("Role schema: {:?}", role_schema);
}
