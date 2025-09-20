// 35. How do you define an enum with an internally tagged representation using Serde? Serialize and deserialize sample JSON.
// What are the advantages of tagged enums?

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
enum Message {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

fn main() {
    // Create a sample enum instance
    let msg = Message::Request {
        id: "42".to_owned(),
        method: "get".to_owned(),
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&msg).unwrap();
    println!("Serialized JSON:\n{}", json);

    // Deserialize back to the enum
    let deserialized: Message = serde_json::from_str(&json).unwrap();
    println!("\nDeserialized enum:\n{:?}", deserialized);
}
