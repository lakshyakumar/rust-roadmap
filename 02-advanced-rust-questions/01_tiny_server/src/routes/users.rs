// src/routes/users.rs
use std::collections::HashMap;

pub fn handle_get(params: HashMap<String, String>) -> String {
    let id = params.get("id").cloned().unwrap_or("unknown".to_string());
    format!("User ID requested: {}\n", id)
}
