use crate::types::Response;
use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};
// use uuid::Uuid;

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct UserResponse {
    id: String,
    name: String,
    age: u32,
    email: String,
}

#[derive(Deserialize)]
struct GetUserRequest {
    email: String,
}

pub async fn get(req: &str, db: &Database) -> Response {
    if let Some(idx) = req.find("\r\n\r\n") {
        let body_string = &req[idx + 4..];
        if let Ok(payload) = serde_json::from_str::<GetUserRequest>(body_string) {
            let collection = db.collection::<mongodb::bson::Document>("users");
            println!("email: {}", &payload.email);
            // let filter = doc! { "email": &payload.email };
            match collection.find(filter).await {
                Ok(mut cursor) => {
                    use futures::stream::TryStreamExt;
                    let mut users = Vec::new();
                    while let Some(doc) = cursor.try_next().await.unwrap_or(None) {
                        // Extract fields manually, mapping _id to id
                        let id = doc
                            .get_object_id("_id")
                            .map(|oid| oid.to_hex())
                            .unwrap_or_default();
                        let name = doc.get_str("name").unwrap_or("").to_string();
                        let age = doc.get_i32("age").unwrap_or(0) as u32;
                        let email = doc.get_str("email").unwrap_or("").to_string();
                        users.push(UserResponse {
                            id,
                            name,
                            age,
                            email,
                        });
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
                Err(_) => {
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

pub async fn handle(req: &str, db: &Database) -> Response {
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
                            email: payload.email,
                        };
                        if let Ok(body) = serde_json::to_string(&user) {
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
                }
            }
            // let user = UserResponse {
            //     id: Uuid::new_v4().to_string(),
            //     name: payload.name,
            //     age: payload.age,
            //     email: payload.email,
            // };
            // if let Ok(body) = serde_json::to_string(&user) {
            //     return Response {
            //         status: 201,
            //         content_type: "application/json".into(),
            //         body,
            //     };
            // }
        }
    }
    return Response {
        status: 400,
        content_type: "text/plain".into(),
        body: "Bad Request".into(),
    };
}
