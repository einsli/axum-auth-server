use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Debug, Serialize, Deserialize,FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub user_id: String,
    pub user_name: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_del: i16,
}


impl User {
    pub const TABLE: &'static str = "users";
}

// 用户注册请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterUserRequest {
    pub user_name: Option<String>,
    pub password: Option<String>,
}

// 用户信息展示
#[derive(Debug, Serialize, Deserialize,FromRow, Clone)]
pub struct UserInfoDisPlay {
    pub user_id: String,
    pub user_name: String,
}
