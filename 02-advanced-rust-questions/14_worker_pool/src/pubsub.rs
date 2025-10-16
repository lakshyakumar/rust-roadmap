use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

pub struct PubSubManager {
    channels: HashMap<String, broadcast::Sender<String>>,
}

impl PubSubManager {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, channel: &str) -> broadcast::Receiver<String> {
        if let Some(sender) = self.channels.get(channel) {
            sender.subscribe()
        } else {
            let (tx, rx) = broadcast::channel(100);
            self.channels.insert(channel.to_string(), tx.clone());
            rx
        }
    }

    pub async fn publish(&self, channel: &str, message: String) {
        if let Some(sender) = self.channels.get(channel) {
            let _ = sender.send(message);
        } else {
            // If channel doesn’t exist yet, create and send
            let (tx, _) = broadcast::channel(100);
            let _ = tx.send(message);
        }
    }
}

pub type SharedPubSub = Arc<Mutex<PubSubManager>>;
