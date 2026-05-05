use jsonwebtoken::{
    encode, decode, get_current_timestamp,
    Algorithm, DecodingKey, EncodingKey, Header, Validation, errors::Error
};
use serde::{Deserialize, Serialize};

// ── Config ────────────────────────────────────────────────────────────────────

const TOKEN_EXPIRY_SECS: u64 = 24 * 60 * 60; // 24 hours

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn create_token(user_id: &str, secret: &str) -> Result<String, Error> {
    let header = Header::new(Algorithm::HS512);
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    let iat = get_current_timestamp();
    let claims = Claims {
        sub: user_id.to_owned(),
        iat,
        exp: iat + TOKEN_EXPIRY_SECS,
    };

    encode(
        &header,
        &claims,
        &encoding_key,
    )
}

pub fn validate_token(token: &str, secret: &str) -> Result<Claims, Error> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS512);
    validation.validate_exp = true;

    decode::<Claims>(
        token,
        &decoding_key,
        &validation
    ).map(|data| data.claims)
}
