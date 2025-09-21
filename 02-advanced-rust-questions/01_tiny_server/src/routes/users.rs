// src/routes/users.rs
use crate::utils::simple_response;
use std::collections::HashMap;

pub fn handle_get(params: HashMap<String, String>) -> String {
    let id = params.get("id").cloned().unwrap_or("unknown".to_string());
    simple_response(
        200,
        "OK",
        &format!("Body:\n{}\n", format!("User ID requested: {}\n", id)),
    )
}
