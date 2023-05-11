#![allow(non_snake_case)]
use sqlx::mysql::MySqlPool;
use crate::response::ResVO;


/**
 * @desc AppState  App 状态
 */
#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: MySqlPool,
}

pub const JWT_SECRET: &str = "axum-user-auth";

/**
 * @desc token过期时间
 */
pub const EXPIRE_HOURS: i64 = 2;



pub fn CUSTOM_AUTHORIZATION_ERROR() -> ResVO<()> {
    let AUTHORIZATION_ERROR: ResVO<()> = ResVO::from_error(
        Some(403),
        "Authorization Error".to_string(),
        None);
    AUTHORIZATION_ERROR
}