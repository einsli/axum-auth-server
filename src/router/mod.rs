use axum::{
    routing::{get, post},
    Router,
    middleware::from_fn,
};

use crate::{
    handlers::user_hander,
    interceptor::header_interceptor::verify_auth,
};


/**
 * @desc 总路由
 */
pub fn routers(
) -> Router {
    Router::new()
        .nest("/user", user_routers())
        .nest("/api", api_routers())
}

/**
 * @desc 注册登录路由
 */
pub fn user_routers() -> Router {
    Router::new()
        .route("/register", post(user_hander::user_register))
        .route("/login", post(user_hander::authorize))
}


/**
 * @desc API路由
 */
pub fn api_routers() -> Router {
    Router::new()
        .route("/get_me", get(user_hander::get_me))
        .route("/logout", get(user_hander::logout))
        .layer(from_fn(verify_auth))
}