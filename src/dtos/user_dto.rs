use crate::{
    base::errors::AppError,
    config::EXPIRE_HOURS,
    db::axredis::get_redis_client,
    models::{
        user::{
            RegisterUserRequest,
            User,
            UserInfoDisPlay,
        },
        auth::{
            AuthBody,
            AuthUser,
        },
    },
    utils::{
        rencrypt::{
            hash,
            verify,
        },
        jwt::sign,
    },
    Result,
};

use chrono::Utc;
use sqlx::mysql::MySqlPool;


pub async fn create(pool: &MySqlPool, payload: RegisterUserRequest) -> Result<u64> {
    // 判断用户名是否存在
    let query_user_count = user_count(pool, payload.user_name.as_ref().unwrap().as_str())
        .await?;

    if query_user_count != 0 {
        return Err(AppError::duplicate("user is already exist"))
    }

    let user_id = uuid::Uuid::new_v4().to_string();
    let time_now = Utc::now();
    let passwd_hashed = hash(&payload.password.unwrap()).unwrap();

    let sql = "
        INSERT INTO
            `users` (
                `user_id`,
                `user_name`,
                `password`,
                `created_at`,
                `updated_at`,
                `is_del`
            )
        VALUES
            (?,?,?,?,?,?)";

    let affected_row = sqlx::query(sql)
        .bind(&user_id)
        .bind(&payload.user_name)
        .bind(&passwd_hashed)
        .bind(time_now)
        .bind(time_now)
        .bind(0)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(affected_row)
}


// 查询用户是否存在
pub async fn user_count(pool: &MySqlPool, user_name: &str) -> Result<i32> {
    let sql: &str = "
         SELECT EXISTS(
            SELECT * FROM
            `users`
        WHERE
            `user_name` = ?)";
    let user_count = sqlx::query_as::<_, (i32,)>(sql)
        .bind(user_name)
        .fetch_one(pool)
        .await?;
    Ok(user_count.0)
}


// 用户登录
pub async fn user_auth(
    pool: &MySqlPool,
    payload:AuthUser) ->Result<AuthBody> {
    let user_info = find_user(pool, payload.user_name.as_ref().unwrap().as_str())
        .await?;

    let verify_res = verify(payload.password.as_ref().unwrap(), &user_info.password).unwrap();
    let token_key = format!("token:{}", &user_info.user_id);
    if verify_res {
        let rd_client = get_redis_client().unwrap();

        let mut con = rd_client.get_connection()?;

        // 查看是否已经登陆过
        let login_token_string: String = match redis::cmd("GET")
            .arg(token_key.clone())
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

        if !login_token_string.is_empty() {
            tracing::info!("you've already logged in");
            Ok(AuthBody::new(login_token_string))
        } else {
            tracing::info!("user logging");
            let token = sign(user_info.clone().user_id).unwrap();

            let _ : () = redis::cmd("SET").arg(token_key)
                .arg(token.clone())
                .arg("EX")
                .arg(EXPIRE_HOURS * 3600)
                .query(&mut con)?;

            Ok(AuthBody::new(token))
        }
    } else {
        Err(AppError::forbidden())
    }
}


// 查询用户
async fn find_user(pool: &MySqlPool, user_name: &str) -> Result<User> {
    let sql: &str = "
        SELECT * FROM
            `users`
        WHERE
            `user_name` = ?";
    let user_info_opt = sqlx::query_as::<_, User>(sql)
        .bind(user_name)
        .fetch_one(pool)
        .await;

    match user_info_opt {
        Ok(user) => {
            Ok(user)
        },
        Err(_err) => {
            Err(AppError::notfound())
        }
    }
}

// 查询用户
async fn find_user_by_id(pool: &MySqlPool, user_id: &str) -> Result<User> {
    let sql: &str = "
        SELECT * FROM
            `users`
        WHERE
            `user_id` = ?";
    let user_info_opt = sqlx::query_as::<_, User>(sql)
        .bind(user_id)
        .fetch_one(pool)
        .await;

    match user_info_opt {
        Ok(user) => {
            Ok(user)
        },
        Err(_err) => {
            Err(AppError::notfound())
        }
    }
}

// 用户信息
pub async fn user_info(pool: &MySqlPool, user_id: &str) -> Result<UserInfoDisPlay> {
    let user_info = find_user_by_id(pool, user_id).await?;
    let user_display: UserInfoDisPlay = UserInfoDisPlay {
        user_id: user_info.user_id,
        user_name: user_info.user_name,
    };
    Ok(user_display)
}

// 用户注销
pub async fn logout(user_id: &str) -> Result<bool> {
    let rd_client = get_redis_client().unwrap();

    let mut con = rd_client.get_connection()?;

    let token_key = format!("token:{}", user_id);

    let _ : () = redis::cmd("DEL")
        .arg(token_key)
        .query(&mut con)?;

    Ok(true)
}