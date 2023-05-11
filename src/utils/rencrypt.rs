use bcrypt::DEFAULT_COST;

use crate::{base::errors::AppError, Result};


// 用于进行hash加密
pub fn hash(pwd: &str) -> Result<String> {
    bcrypt::hash(pwd, DEFAULT_COST).map_err(AppError::from)
}


// 用于验证密码
pub fn verify(pwd: &str, hashed_pwd: &str) -> Result<bool> {
    bcrypt::verify(pwd, hashed_pwd).map_err(AppError::from)
}
