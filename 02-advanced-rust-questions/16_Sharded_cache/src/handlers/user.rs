use std::sync::Arc;

use crate::cache::sharded::ShardedCache;
use crate::types::Response;
use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub age: u32,
    pub email: String,
}

#[derive(Deserialize)]
struct GetUserRequest {
    email: String,
}

/// Get user(s) by email. Cache is now a sharded cache (async).
pub async fn get(
    req: &str,
    db: &Database,
    cache: &Arc<ShardedCache<String, UserResponse>>,
) -> Response {
    // parse body after header
    if let Some(idx) = req.find("\r\n\r\n") {
        let body_string = &req[idx + 4..];
        if let Ok(payload) = serde_json::from_str::<GetUserRequest>(body_string) {
            // Try cache first (async)
            if let Some(user) = cache.get(&payload.email).await {
                if let Ok(body) = serde_json::to_string(&vec![user]) {
                    println!("cache hit!");
                    return Response {
                        status: 200,
                        content_type: "application/json".into(),
                        body,
                    };
                }
            }

            // Not in cache -> query DB
            let collection = db.collection::<mongodb::bson::Document>("users");
            let filter = doc! { "email": &payload.email };

            match collection.find(filter).await {
                Ok(mut cursor) => {
                    use futures::stream::TryStreamExt;
                    let mut users = Vec::new();
                    while let Some(doc) = cursor.try_next().await.unwrap_or(None) {
                        let id = doc
                            .get_object_id("_id")
                            .map(|oid| oid.to_hex())
                            .unwrap_or_default();
                        let name = doc.get_str("name").unwrap_or("").to_string();
                        let age = doc.get_i32("age").unwrap_or(0) as u32;
                        let email = doc.get_str("email").unwrap_or("").to_string();

                        let user = UserResponse {
                            id,
                            name,
                            age,
                            email: email.clone(),
                        };

                        // populate cache asynchronously
                        cache.put(email.clone(), user.clone()).await;
                        users.push(user);
                    }

                    if !users.is_empty() {
                        if let Ok(body) = serde_json::to_string(&users) {
                            return Response {
                                status: 200,
                                content_type: "application/json".into(),
                                body,
                            };
                        }
                    }

                    return Response {
                        status: 404,
                        content_type: "text/plain".into(),
                        body: "No users found".into(),
                    };
                }
                Err(err) => {
                    eprintln!("mongo find error: {:?}", err);
                    return Response {
                        status: 500,
                        content_type: "text/plain".into(),
                        body: "Database error".into(),
                    };
                }
            }
        }
    }

    Response {
        status: 400,
        content_type: "text/plain".into(),
        body: "Bad Request".into(),
    }
}

/// Create a user and insert into DB + cache.
pub async fn handle(
    req: &str,
    db: &Database,
    cache: &Arc<ShardedCache<String, UserResponse>>,
) -> Response {
    if let Some(idx) = req.find("\r\n\r\n") {
        let body_str = &req[idx + 4..];
        if let Ok(payload) = serde_json::from_str::<CreateUserRequest>(body_str) {
            let user_doc = doc! {
                "name": &payload.name,
                "age": payload.age as i32,
                "email": &payload.email,
            };
            let collection = db.collection("users");
            match collection.insert_one(user_doc).await {
                Ok(result) => {
                    if let Some(oid) = result.inserted_id.as_object_id() {
                        let user = UserResponse {
                            id: oid.to_hex(),
                            name: payload.name,
                            age: payload.age,
                            email: payload.email.clone(),
                        };
                        if let Ok(body) = serde_json::to_string(&user) {
                            // put into sharded cache (async)
                            cache.put(payload.email.clone(), user.clone()).await;

                            return Response {
                                status: 201,
                                content_type: "application/json".into(),
                                body,
                            };
                        }
                    }
                }
                Err(err) => {
                    eprintln!("mongo insert error: {:?}", err);
                    return Response {
                        status: 500,
                        content_type: "text/plain".into(),
                        body: "Database error".into(),
                    };
                }
            }
        }
    }

    Response {
        status: 400,
        content_type: "text/plain".into(),
        body: "Bad Request".into(),
    }
}
