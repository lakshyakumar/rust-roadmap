use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub service: String,
    pub action: String,
    pub timestamp: String,
}

static EVENT_LOG: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub async fn append_event(service: &str, action: &str) {
    let _guard = EVENT_LOG.lock().await;
    let event = Event {
        service: service.to_string(),
        action: action.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    let json_event = serde_json::to_string(&event).unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("event.log")
        .unwrap();
    writeln!(file, "{}", json_event).unwrap();
}

pub fn rebuild_state() {
    let file = match File::open("events.log") {
        Ok(f) => f,
        Err(_) => {
            println!("No events.log found — no events have been recorded yet.");
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut user_requests = 0;
    let mut order_requests = 0;

    for line in reader.lines() {
        if let Ok(l) = line {
            if let Ok(event) = serde_json::from_str::<Event>(&l) {
                match event.action.as_str() {
                    "GET /users" => user_requests += 1,
                    "GET /orders" => order_requests += 1,
                    _ => {}
                }
            }
        }
    }

    println!("=== Rebuilt State ===");
    println!("Total user requests: {}", user_requests);
    println!("Total order requests: {}", order_requests);
}
