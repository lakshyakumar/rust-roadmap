use crate::types::Response;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

fn get_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "fallback_secret".to_string())
        .into_bytes()
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject = username
    pub exp: usize,  // expiry timestamp
    pub iat: usize,  // usize
    pub email: Option<String>,
}

pub async fn create_token(req: &str) -> Response {
    if let Some(idx) = req.find("\r\n\r\n") {
        let body = &req[idx + 4..];
        if let Ok(payload) = serde_json::from_str::<AuthRequest>(&body) {
            let now = Utc::now();
            let claims = Claims {
                sub: payload.username.clone(),
                email: payload.email.clone(),
                iat: now.timestamp() as usize,
                exp: (now + Duration::hours(1)).timestamp() as usize,
            };
            let secret = get_secret();

            match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(&secret),
            ) {
                Ok(token) => {
                    let body = serde_json::json!({"token": token}).to_string();
                    return Response {
                        status: 200,
                        content_type: "application/json".into(),
                        body,
                    };
                }
                Err(err) => {
                    eprintln!("jwt encode error: {:?}", err);
                    return Response {
                        status: 500,
                        content_type: "text/plain".into(),
                        body: "Token creation failed".into(),
                    };
                }
            }
        }
    }

    Response {
        status: 400,
        content_type: "text/plain".into(),
        body: "Bad Request".into(),
    }
}

pub async fn verify_token(req: &str) -> Response {
    if let Some(idx) = req.find("\r\n\r\n") {
        let body = &req[idx + 4..];
        if let Ok(payload) = serde_json::from_str::<VerifyRequest>(body) {
            let secret = get_secret();
            match decode::<Claims>(
                &payload.token,
                &DecodingKey::from_secret(&secret),
                &Validation::default(),
            ) {
                Ok(token_data) => {
                    let body = serde_json::json!({
                        "valid": true,
                        "claims": token_data.claims
                    })
                    .to_string();
                    return Response {
                        status: 200,
                        content_type: "application/json".into(),
                        body,
                    };
                }
                Err(err) => {
                    let body = serde_json::json!({
                        "valid": false,
                        "error": err.to_string()
                    })
                    .to_string();
                    return Response {
                        status: 401,
                        content_type: "application/json".into(),
                        body,
                    };
                }
            }
        }
    }
    Response {
        status: 400,
        content_type: "text/plain".into(),
        body: "Bad Request".into(),
    }
}
