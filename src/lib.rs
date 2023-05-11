pub mod base;
pub mod config;
pub mod db;
pub mod dtos;
pub mod handlers;
pub mod interceptor;
pub mod models;
pub mod response;
pub mod router;
pub mod utils;

use crate::{
    base::errors::AppError,
};
use crate::response::ResVO;

use axum::Json;

pub type ResponseResult<T> = std::result::Result<Json<ResVO<T>>, AppError>;
pub type Result<T> = std::result::Result<T, AppError>;

