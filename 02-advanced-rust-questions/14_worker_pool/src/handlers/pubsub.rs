use crate::{pubsub::SharedPubSub, types::Response};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
struct PublishRequest {
    channel: String,
    message: String,
}

#[derive(Deserialize)]
struct SubscribeRequest {
    channel: String,
}

#[derive(Clone)]
pub struct PubSubHandler {
    pubsub: SharedPubSub,
}

impl PubSubHandler {
    pub fn new(pubsub: SharedPubSub) -> Self {
        Self { pubsub }
    }

    pub async fn publish(&self, req: &str) -> Response {
        if let Some(idx) = req.find("\r\n\r\n") {
            let body = &req[idx + 4..];
            if let Ok(payload) = serde_json::from_str::<PublishRequest>(body) {
                let pubsub = self.pubsub.clone();
                let pubsub = pubsub.lock().await;
                pubsub
                    .publish(&payload.channel, payload.message.clone())
                    .await;

                return Response {
                    status: 200,
                    content_type: "application/json".into(),
                    body: format!(
                        "{{\"status\": \"ok\", \"msg\": \"Published to {}\"}}",
                        payload.channel
                    ),
                };
            }
        }

        Response {
            status: 400,
            content_type: "application/json".into(),
            body: "{\"error\": \"Invalid request\"}".into(),
        }
    }

    pub async fn subscribe(&self, req: &str) -> Response {
        if let Some(idx) = req.find("\r\n\r\n") {
            let body = &req[idx + 4..];
            if let Ok(payload) = serde_json::from_str::<SubscribeRequest>(body) {
                let pubsub = self.pubsub.clone();
                let channel = payload.channel.clone();
                let mut rx = {
                    let mut guard = pubsub.lock().await;
                    guard.subscribe(&channel)
                };

                tokio::spawn({
                    let channel = channel.clone();
                    async move {
                        while let Ok(msg) = rx.recv().await {
                            println!("📩 Message on '{}': {}", channel, msg);
                        }
                    }
                });

                return Response {
                    status: 200,
                    content_type: "application/json".into(),
                    body: format!(
                        "{{\"status\": \"subscribed\", \"channel\": \"{}\"}}",
                        channel
                    ),
                };
            }
        }

        Response {
            status: 400,
            content_type: "application/json".into(),
            body: "{\"error\": \"Invalid request\"}".into(),
        }
    }
}
