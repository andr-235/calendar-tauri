use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};

const JWT_SECRET: &str = "your-secret-key-change-in-production";
const JWT_EXPIRATION_HOURS: u64 = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub role: String,
    pub exp: usize,
}

pub fn hash_password(password: &str) -> Result<String, String> {
    if password.len() < 6 {
        return Err("Password must be at least 6 characters long".to_string());
    }
    hash(password, DEFAULT_COST)
        .map_err(|e| format!("Failed to hash password: {}", e))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    verify(password, hash)
        .map_err(|e| format!("Failed to verify password: {}", e))
}

pub fn generate_token(user_id: i64, role: &str) -> Result<String, String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(JWT_EXPIRATION_HOURS as i64))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        role: role.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
    .map_err(|e| format!("Failed to generate token: {}", e))
}

pub fn verify_token(token: &str) -> Result<Claims, String> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| format!("Invalid token: {}", e))?;

    Ok(token_data.claims)
}

