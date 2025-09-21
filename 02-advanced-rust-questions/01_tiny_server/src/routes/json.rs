use std::collections::HashMap;

use crate::utils::simple_response;

pub fn handle(headers: &HashMap<String, String>, body: &str) -> String {
    let ct = headers.get("content-type").cloned().unwrap_or_default();
    simple_response(
        200,
        "OK",
        &format!("Content-Type: {}\nBody:\n{}\n", ct, body),
    )
}
