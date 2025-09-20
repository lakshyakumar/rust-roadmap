// 88. How do you implement HS256 JWT signing and verification for a minimal web API?
// Manage claims like exp, iat, and nbf. Why is JWT widely used for authentication?

// minimal HS256 JWT signer + verifier in Rust, show how to manage exp, iat, and nbf claims, and give a small example of using the verifier
//  in a web handler. use the commonly-used jsonwebtoken crate for correctness and safety.

//  keep this as a single, runnable example (library-style functions + a tiny Actix-web handler example).
//  Replace the web framework snippet with your preferred one if needed.

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Our JWT claims.
/// You can add application-specific fields here (roles, user_id, etc).
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // standard registered claims
    exp: usize,         // expiration time (as UTC timestamp seconds)
    iat: usize,         // issued at
    nbf: Option<usize>, // not before (optional)
    // app-specific
    sub: String, // subject (user id or username)
    role: Option<String>,
}

/// Create an HS256-signed JWT.
/// `secret` should be application secret bytes (keep it safe).
fn create_jwt(
    subject: &str,
    role: Option<&str>,
    secret: &[u8],
    ttl_seconds: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::seconds(ttl_seconds)).timestamp();

    let claims = Claims {
        exp: exp as usize,
        iat: iat as usize,
        nbf: None,
        sub: subject.to_owned(),
        role: role.map(|s| s.to_owned()),
    };

    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(secret))
}

/// Verify & decode a token string. Returns claims on success.
/// Uses a small leeway for clock skew.
fn verify_jwt(token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
    // Validation setup: require HS256 and set leeway for clock skew
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 60; // seconds of allowed clock skew

    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)?;
    Ok(token_data.claims)
}

/// Helper: extract "Bearer <token>" from Authorization header
fn extract_bearer(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| {
            if let Some(rest) = s.strip_prefix("Bearer ") {
                Some(rest.to_string())
            } else {
                None
            }
        })
}

/// Example protected route using Actix-web
async fn protected(req: HttpRequest) -> impl Responder {
    // In real code read secret from env/config, not hard-coded
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "my-secret-key".into());

    if let Some(token) = extract_bearer(&req) {
        match verify_jwt(&token, secret.as_bytes()) {
            Ok(claims) => {
                let subj = claims.sub;
                let role = claims.role.unwrap_or_else(|| "none".into());
                HttpResponse::Ok().body(format!("Hello {}, role={}", subj, role))
            }
            Err(e) => HttpResponse::Unauthorized().body(format!("Invalid token: {}", e)),
        }
    } else {
        HttpResponse::Unauthorized().body("Missing Bearer token")
    }
}

/// Example route that issues a token (login simulation)
async fn login() -> impl Responder {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "my-secret-key".into());
    // In real code, validate credentials first
    match create_jwt("user123", Some("admin"), secret.as_bytes(), 3600) {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to create token: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("Run: export JWT_SECRET=some-long-random-secret; cargo run");

    HttpServer::new(|| {
        App::new()
            .route("/login", web::get().to(login))
            .route("/protected", web::get().to(protected))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
