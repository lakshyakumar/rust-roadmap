use crate::types::Response;

pub async fn handle() -> Response {
    Response {
        status: 200,
        content_type: "text/plain".into(),
        body: "Ok".into(),
    }
}
