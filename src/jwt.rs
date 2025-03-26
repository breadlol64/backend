use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn gen_jwt(subject: &str) -> String{
    let key = env::var("JWT_KEY").expect("JWT_KEY must be set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(30*24*60*60)) // days*hours*minutes*seconds
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: subject.to_owned(),
        exp: expiration
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(key.as_ref()))
        .expect("Failed to generate JWT");

    token
}

pub fn decode_jwt(token: &str) -> String {
    let key = env::var("JWT_KEY").expect("JWT_KEY must be set");

    let token = decode::<Claims>(&token, &DecodingKey::from_secret(key.as_ref()), &Validation::default())
        .expect("Failed to decode JWT");

    token.claims.sub
}