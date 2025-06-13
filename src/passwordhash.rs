

use argon2::{
    Algorithm, Version, Params,
     password_hash::{PasswordHasher, SaltString},
    Argon2,
 };
use rand_core::OsRng;

// use rand_core::OsRng;


pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error>{

    let salt = SaltString::generate(&mut OsRng);


    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(
            15_000,
            2,
            1,
            None
        )?
    );

    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())


}

//verification
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier}
};

pub fn verify_password(
    password: &str,
    stored_hash: &str,
) -> Result<bool, argon2::password_hash::Error>{
    let parsed_hash = PasswordHash::new(stored_hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
    .is_ok())
}