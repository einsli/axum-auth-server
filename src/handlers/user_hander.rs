use axum::{
    Json,
    extract::{
        Extension,
    },
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use std::sync::Arc;

use crate::{
    config::AppState,
    dtos::{
        user_dto,
    },
    models::{
        auth::{
            AuthUser,
            AuthBody,
        },
        user::{
            UserInfoDisPlay,
            RegisterUserRequest,
        },
    },
    response::ResVO,
    ResponseResult,
    utils::jwt,
};

use axum_macros::debug_handler;


/**
 * @desc 用户创建
 */
#[debug_handler]
pub async fn user_register(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RegisterUserRequest>
) -> ResponseResult<String> {
    tracing::info!("register request");
    let affected_row = user_dto::create(&state.db_pool, payload).await;

    match affected_row {
        Ok(res) => {
            if res == 1 {
                Ok(
                    Json(ResVO::<String>::success_without_data())
                )
            } else {
                Ok(
                    Json(ResVO::<String>::from_error(
                        Some(1) ,
                        "no user create".to_string(),
                        None))
                )
            }

        },
        Err(err) => {
            Ok(
                Json(ResVO::<String>::from_error(
                    Some(1) ,
                    err.message.unwrap(),
                    None))
            )
        }
    }
}


/**
 * @desc 用户登录
 */
pub async fn authorize(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<AuthUser>
) -> ResponseResult<AuthBody> {
    tracing::info!("login request");
    let auth_res: Result<AuthBody, crate::base::errors::AppError> = user_dto::user_auth(
        &state.db_pool,
        payload).await;
    match auth_res {
        Ok(res) => {
            Ok(Json(
                ResVO::<AuthBody>::success_with_data(res)
            ))
        },
        Err(err) => {
            Ok(
                Json(
                    ResVO::<AuthBody>::from_error(
                        Some(1),
                        err.message.unwrap() ,
                        None
                ))
            )
        }
    }

}

/**
 * @desc get me
 */
pub async fn get_me(
    Extension(state): Extension<Arc<AppState>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> ResponseResult<UserInfoDisPlay> {
    // 这里有个报错需要处理下
    let token_data = jwt::verify( bearer.token()).unwrap();
    let user_id = token_data.sub;

    let user_info = user_dto::user_info(&state.db_pool, &user_id)
        .await?;

    Ok(Json(
        ResVO::<UserInfoDisPlay>::success_with_data(user_info)
    ))
}

/**
 * @desc get me
 */
pub async fn logout(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> ResponseResult<String> {
    // 这里有个报错需要处理下
    let token_data = jwt::verify( bearer.token()).unwrap();
    let user_id = token_data.sub;

    let logout_res = user_dto::logout(&user_id)
        .await?;
    if logout_res {
        Ok(Json(
            ResVO::<String>::success_without_data()
        ))
    } else {
        Ok(Json(
            ResVO::<String>::from_error(
                Some(1),
                "logout error".to_string(),
                None)
        ))
    }
}