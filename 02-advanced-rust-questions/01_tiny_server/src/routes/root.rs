use crate::utils::simple_response;

pub fn handle() -> String {
    simple_response(200, "OK", "Welcome to the tiny tokio server!\n")
}
