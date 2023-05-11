use serde::{Serialize, Deserialize};

// 用户登录请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser {
    pub user_name: Option<String>,
    pub password: Option<String>,
}

// 用户登录验证结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthRes {
    pub user_id: Option<String>,
    pub user_token: Option<String>,
}

// 请求时候验证请求body
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}