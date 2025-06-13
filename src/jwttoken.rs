
use serde::{ Serialize, Deserialize};
use chrono::{Utc, Duration};



#[derive(Debug, Serialize, Deserialize)]
pub struct Clamis {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub nbf: usize,
    pub aud: String,
    pub iss: String,
    pub role: String,
}


impl Clamis {

    pub fn new(user_id: String, role: String, validity_hours: i64) -> Self {
        let now = Utc::now();

        Clamis { 
            sub: user_id, 
            exp: (now + Duration::hours(validity_hours)).timestamp() as usize , 
            iat: now.timestamp() as usize,
            nbf: now.timestamp() as usize,
            aud: "my-app".to_string(),
            iss: "auth-service".to_string(),
            role, 
        }
    }

}

use jsonwebtoken::{
    Algorithm,
    EncodingKey,
    DecodingKey,
    Validation
};

pub struct JwtConfig {
    pub algorithm: Algorithm,
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub validation: Validation,
}

impl JwtConfig {

    pub fn new(secret: &str) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["my-app"]);
        validation.set_issuer(&["auth-service"]);
        validation.leeway = 30;

        JwtConfig { 
            algorithm: Algorithm::HS256 ,
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            validation,
         }
    }
}


use jsonwebtoken::Header;
use tokio_postgres::config;

pub fn generate_token(
    clamis: &Clamis,
    config: &JwtConfig,
) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::new(config.algorithm),
        claims,
        &config.encoding_key,
    )
}

pub fn verify_token(
    token: &str,
    config: &JwtConfig,
) -> Result<Clamis, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &config.decoding_key,
        &config.validation,
    )?;

    Ok(token_data.claims)
}

match verify_token(&token, &config) {
    Ok(claims) => println!("Authenticated as {} with role {}", claims.sub, claims.role),
    Err(e) => eprintln!("Token verification failed: {:?}", e),
}