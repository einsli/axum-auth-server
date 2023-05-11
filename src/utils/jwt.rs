use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode};
use serde::{Serialize, Deserialize};

use crate::base::errors::AppError;


use crate::config::{
    JWT_SECRET,
    EXPIRE_HOURS,
};
use crate::Result;

// token 声明，用于数据存储
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(EXPIRE_HOURS);

        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

impl std::fmt::Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id: {}\niat: {}", self.sub, self.iat)
    }
}

// token签名
pub fn sign(id: String) -> Result<String> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    ).unwrap())
}


// token验证
pub fn verify(token: &str) -> Result<Claims> {
    let verified_code_opt = decode(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default()
    ).map(|data| data.claims);

    match verified_code_opt {
        Ok(res) => {
            Ok(res)
        },
        Err(err) => {
            tracing::error!("jwt token verify error {}", err);
            Err(AppError::forbidden())
        }
    }
}