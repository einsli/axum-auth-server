use axum::{
    middleware::Next,
    http::{
        Request,
    },
    response::IntoResponse,
    Json
};

use crate::{
    config::CUSTOM_AUTHORIZATION_ERROR,
    utils::jwt,
    db::axredis::get_redis_client,
};


/**
 * @desc 自定义请求头验证
 */
pub async fn verify_auth<B>(
    request: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    // 在这里访问请求头并进行验证
    if request.headers().contains_key("Authorization") {
        // 验证成功，继续处理请求
        let token_bare = request.headers().get("Authorization")
            .unwrap().
            to_str().
            unwrap();

        let jwt_token = token_bare.trim_start_matches("Bearer ");
        let jwt_claims_id = match jwt::verify(jwt_token) {
            Ok(res) => {
                res.sub
            },
            Err(_err) => {
                "".to_string()
            }
        };
        if jwt_token.is_empty() {
            return Json(CUSTOM_AUTHORIZATION_ERROR()).into_response()
        }

        let client = get_redis_client().unwrap();
        let mut con = client.get_connection().unwrap();
        let user_token_key = format!("token:{}",jwt_claims_id);

        let login_token_string: String = match redis::cmd("GET")
            .arg(user_token_key)
            .query(&mut con) {
            Ok(login_token) => {
                tracing::debug!("user token is invalid!");
                login_token
            },
            Err(err) => {
                tracing::debug!("user token is expired! {:?}", err);
                "".to_string()
            }
        };
        if login_token_string.is_empty() {
            return Json(CUSTOM_AUTHORIZATION_ERROR()).into_response()
        }
        // 通过就正常请求
        next.run(request).await
    } else {
        // 验证失败，返回自定义错误响应
        Json(CUSTOM_AUTHORIZATION_ERROR()).into_response()
    }
}