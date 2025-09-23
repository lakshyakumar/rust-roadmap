use crate::types::Response;

pub fn handle() -> Response {
    Response {
        status: 200,
        content_type: "text/plain".into(),
        body: "Ok".into(),
    }
}
