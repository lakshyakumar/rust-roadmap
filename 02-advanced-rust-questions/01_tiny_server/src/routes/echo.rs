use crate::utils::simple_response;

pub fn handle(body: &str) -> String {
    simple_response(200, "OK", &format!("you posted:\n{}\n", body))
}
