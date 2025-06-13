use futures_util::future::ok;
use jsonwebtoken::{EncodingKey, DecodingKey};
use redis::RedisConnectionInfo;
use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct SessionClaims {
    pub sub: String,
    pub session_id: Uuid,
    pub exp: usize,
    pub refresh_until: usize,
}


pub struct SessionConfig {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub access_exp: i64,
    pub refresh_exp: i64,
}


use chrono::Utc;

pub fn create_session(
    user_id: &str,
    config: &SessionConfig
) -> (String, String) {
    let session_id = Uuid::new_v4();
    let now = Utc::now();

    let access_claims = SessionClaims {
        sub: user_id.to_string(),
        session_id,
        exp: (now + chrono::Duration::minutes(15)).timestamp() as usize,
        refresh_until: (now + chrono::Duration::days(7)).timestamp() as usize,
    };

    let refresh_claims = SessionClaims {
        exp: (now + chrono::Duration::days(7)).timestamp() as usize,
        ..access_claims.clone()
    };

    let access_token = encode_acesss_token(&access_claims, config);
    let refresh_token = encode_refresh_token(&refresh_claims, config);

    (access_token, refresh_token)
}

use bb8_redis::{bb8, RedisConnectionManager};

pub type RedisPool = bb8::Pool<RedisConnectionManager>;

pub async fn init_redis_pool() -> RedisPool {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    bb8::Pool::builder()
        .max_size(15)
        .build(manager)
        .await
        .unwrap()
}

use redis::AsyncCommands;

pub async fn save_session(
    pool: &RedisPool,
    session_id: &Uuid,
    user_id: &str,
    ttl: usize,
) -> Result<(), SessionError> {
    let mut conn = pool.get().await?;
    conn.set_ex(
        format!("session:{}", session_id),
        user_id,
        ttl
    ).await?;
    Ok(())
}


pub async fn validate_session(
    pool: &RedisPool,
    session_id: &Uuid,
) -> Result<Option<String>, SessionError> {
    let mut conn = pool.get().await?;
    let user_id: Option<String> = conn.get(format!("session:{}", session_id)).await?;
    Ok(user_id)
}