pub mod echo;
pub mod json;
pub mod root;
mod router;
pub mod time;
mod users;

use crate::utils::simple_response;
use std::collections::HashMap;

pub fn router(method: &str, path: &str, headers: &HashMap<String, String>, body: &str) -> String {
    match (method, path) {
        ("GET", "/") => root::handle(),
        ("GET", "/time") => time::handle(),
        ("POST", "/echo") => echo::handle(body),
        ("POST", "/json") => json::handle(headers, body),
        _ => {
            // dynamic route check: /users/:id
            if let Some(params) = router::match_route("/users/:id", path) {
                return users::handle_get(params);
            }

            simple_response(404, "Not Found", "Route not found\n")
        }
    }
}
