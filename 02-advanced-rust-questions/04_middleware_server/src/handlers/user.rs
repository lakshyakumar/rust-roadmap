use crate::types::Response;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: String,
    name: String,
    age: u32,
    email: String,
}

pub fn handle(req: &str) -> Response {
    if let Some(idx) = req.find("\r\n\r\n") {
        let body_str = &req[idx + 4..];
        if let Ok(payload) = serde_json::from_str::<CreateUserRequest>(body_str) {
            let user = UserResponse {
                id: Uuid::new_v4().to_string(),
                name: payload.name,
                age: payload.age,
                email: payload.email,
            };
            if let Ok(body) = serde_json::to_string(&user) {
                return Response {
                    status: 201,
                    content_type: "application/json".into(),
                    body,
                };
            }
        }
    }
    return Response {
        status: 400,
        content_type: "text/plain".into(),
        body: "Bad Request".into(),
    };
}
