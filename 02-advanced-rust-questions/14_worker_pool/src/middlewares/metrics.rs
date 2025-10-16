use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use serde::Serialize;

use crate::middleware::Middleware;
use crate::types::Response;

pub struct MetricsMiddleware {
    total_requests: Arc<Mutex<usize>>,
    total_response_time: Arc<Mutex<f64>>,
    average_response_time: Arc<Mutex<Option<f64>>>,
}

#[derive(Serialize)]
pub struct MetricsResponse {
    total_requests: usize,
    average_response_time: f64,
}
impl MetricsMiddleware {
    pub fn new() -> Self {
        MetricsMiddleware {
            total_requests: Arc::new(Mutex::new(0)),
            total_response_time: Arc::new(Mutex::new(0.0)),
            average_response_time: Arc::new(Mutex::new(None)),
        }
    }

    pub fn handle_metrics(&self) -> Response {
        let total_requests = *self.total_requests.lock().unwrap();
        let average_response_time = self.average_response_time.lock().unwrap().unwrap_or(0.0);
        Response {
            status: 200,
            content_type: "application/json".into(),
            body: serde_json::to_string(&MetricsResponse {
                total_requests,
                average_response_time,
            })
            .unwrap_or_else(|_| "{}".to_string()),
        }
    }
}
impl Middleware for MetricsMiddleware {
    fn handle(
        &self,
        req: &str,
        client_ip: &str,
        next: &dyn for<'a> Fn(&'a str) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>,
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        let req = req.to_string();
        let client_ip = client_ip.to_string();
        let total_requests = Arc::clone(&self.total_requests);
        let total_response_time = Arc::clone(&self.total_response_time);
        let average_response_time = Arc::clone(&self.average_response_time);

        let fut = next(&req);
        Box::pin(async move {
            let start_time = Instant::now();
            let mut parts = req.split_whitespace();
            let response = fut.await;
            let duration = start_time.elapsed();
            let response_time_ms = duration.as_secs_f64() * 1000.0; // Convert to milliseconds

            // if let (Some(method), Some(path)) = (parts.next(), parts.next()) {
            //     println!(
            //         "[Metric gatherer] {} {}, status: {}, response_time: {:.2}ms, from {} ",
            //         method, path, response.status, response_time_ms, client_ip
            //     )
            // }

            // Update metrics
            {
                let mut total = total_requests.lock().unwrap();
                let mut total_time = total_response_time.lock().unwrap();
                let mut avg_time = average_response_time.lock().unwrap();

                *total += 1;
                *total_time += response_time_ms;

                // Calculate new average response time
                let new_average = *total_time / (*total as f64);
                *avg_time = Some(new_average);
            }

            response
        })
    }
}
