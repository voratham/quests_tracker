use anyhow::{Ok, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub fn hash(password: String) -> Result<String> {
    let salt = SaltString::generate(OsRng);

    let bytes_password = password.as_bytes();

    let argon2 = Argon2::default();

    let result = argon2
        .hash_password(bytes_password, &salt)
        // convert mapping error to anyhow package
        .map_err(|e| anyhow::anyhow!(e.to_string()))?; // propagation error when error will throw

    Ok(result.to_string())
}

pub fn verify(password: String, hashed_password: String) -> Result<bool> {
    let parsed_hash =
        PasswordHash::new(&hashed_password).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let bytes_password = password.as_bytes();

    Ok(Argon2::default()
        .verify_password(bytes_password, &parsed_hash)
        .is_ok())
}
