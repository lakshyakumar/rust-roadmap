pub fn split_once<'a>(s: &'a str, delim: char) -> Option<(&'a str, &'a str)> {
    let mut iter = s.splitn(2, delim);
    let a = iter.next()?;
    let b = iter.next()?;
    Some((a, b))
}

pub fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}

pub fn simple_response(status: u16, reason: &str, body: &str) -> String {
    let body_bytes = body.as_bytes();
    format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: close\r\n\r\n{}",
        status,
        reason,
        body_bytes.len(),
        body
    )
}
