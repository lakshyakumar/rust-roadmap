// 34. How do you build a minimal Axum API with a GET /health endpoint and a POST /items endpoint that accepts JSON and returns the created item?
// Include extractors and state management. Why is Axum a good choice for web APIs?

use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::{Arc, Mutex};

// Define the item structure
#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: usize,
    name: String,
}

// In-memory app state
struct AppState {
    items: Mutex<Vec<Item>>,
}

// GET /health handler
async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "OK" }))
}

// POST /items handler
async fn create_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Item>,
) -> (StatusCode, Json<Item>) {
    let mut items = state.items.lock().unwrap();
    items.push(payload.clone());
    (StatusCode::CREATED, Json(payload))
}

// Set up routes and start server
#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        items: Mutex::new(Vec::new()),
    });
    let app = Router::new()
        .route("/health", get(health))
        .route("/items", post(create_item))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
