// 46. What is the typestate pattern? Implement an HttpRequestBuilder that prevents send() until both URL and method are set,
// encoding state via generic type parameters. Why is typestate useful for API safety?

// The typestate pattern encodes the "state" of an object into its type, so the compiler enforces correct usage at compile time.
// Instead of runtime checks (like "did you set the URL before sending?"), Rust can enforce this at compile-time
// using phantom type parameters or marker types.

use std::marker::PhantomData;

// Marker types for state
struct NoUrl;
struct HasUrl;
struct NoMethod;
struct HasMethod;

// Builder struct with phantom markers for state
struct HttpRequestBuilder<UrlState, MethodState> {
    url: Option<String>,
    method: Option<String>,
    _url_state: PhantomData<UrlState>,
    _method_state: PhantomData<MethodState>,
}

// Default state: NoUrl + NoMethod
impl HttpRequestBuilder<NoUrl, NoMethod> {
    fn new() -> Self {
        HttpRequestBuilder {
            url: None,
            method: None,
            _url_state: PhantomData,
            _method_state: PhantomData,
        }
    }
}

// Transition: setting URL moves state from NoUrl -> HasUrl
impl<M> HttpRequestBuilder<NoUrl, M> {
    fn url(self, url: &str) -> HttpRequestBuilder<HasUrl, M> {
        HttpRequestBuilder {
            url: Some(url.to_string()),
            method: self.method,
            _url_state: PhantomData,
            _method_state: PhantomData,
        }
    }
}

// Transition: setting method moves state from NoMethod -> HasMethod
impl<U> HttpRequestBuilder<U, NoMethod> {
    fn method(self, method: &str) -> HttpRequestBuilder<U, HasMethod> {
        HttpRequestBuilder {
            url: self.url,
            method: Some(method.to_string()),
            _url_state: PhantomData,
            _method_state: PhantomData,
        }
    }
}

// Only valid when both URL + Method are set
impl HttpRequestBuilder<HasUrl, HasMethod> {
    fn send(self) {
        println!(
            "Sending {} request to {}",
            self.method.unwrap(),
            self.url.unwrap()
        );
    }
}

fn main() {
    // ❌ Won’t compile: missing URL and method
    // HttpRequestBuilder::new().send();

    // ❌ Won’t compile: method missing
    // HttpRequestBuilder::new().url("https://example.com").send();

    // ✅ Works: both URL + method set
    HttpRequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .send();
}
