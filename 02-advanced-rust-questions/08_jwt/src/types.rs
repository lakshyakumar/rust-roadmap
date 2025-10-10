#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub content_type: String,
    pub body: String,
}

fn status_text(code: u16) -> &'static str {
    match code {
        200 => "Ok",
        400 => "BAD REQUEST",
        403 => "FORBIDDEN",
        404 => "NOT FOUND",
        500 => "INTERNAL SERVER ERROR",
        429 => "TOO MANY REQUESTS",
        _ => "UNKNOWN",
    }
}

impl Response {
    pub fn into_http(self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status,
            status_text(self.status),
            self.content_type,
            self.body.len(),
            self.body
        )
    }
}
