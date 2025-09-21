use crate::utils::simple_response;

pub fn handle() -> String {
    let t = chrono::Utc::now().to_rfc3339();
    simple_response(200, "OK", &format!("UTC time: {}\n", t))
}
