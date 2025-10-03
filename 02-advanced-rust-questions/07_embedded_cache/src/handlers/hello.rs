use crate::types::Response;
use serde::Serialize;
use serde_json::to_string;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

pub async fn handle(req: &str) -> Response {
    if let Some(path) = req.split_whitespace().nth(1) {
        if let Some(name) = path.strip_prefix("/hello/") {
            let body = to_string(&HelloResponse {
                message: format!("Hello, {}!", name),
            })
            .unwrap();
            return Response {
                status: 200,
                content_type: "application/json".into(),
                body,
            };
        }
    }
    Response {
        status: 404,
        content_type: "text/plain".into(),
        body: "Not Found".into(),
    }
}
