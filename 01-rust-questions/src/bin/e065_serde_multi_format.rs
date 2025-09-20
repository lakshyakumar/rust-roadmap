// 65. How do you serialize a struct to multiple formats (JSON, YAML, CBOR) in Rust?
// Compare sizes and use serde(default) for missing fields. Why is multi-format serialization useful?
// Cargo.toml
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// serde_yaml = "0.9"
// serde_cbor = "0.11"

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,

    #[serde(default)] // if missing during deserialization, fallback to default()
    enabled: bool,

    #[serde(default = "default_timeout")]
    timeout: u32,
}

fn default_timeout() -> u32 {
    30
}

fn main() {
    let cfg = Config {
        name: "example".to_string(),
        enabled: true,
        timeout: 45,
    };

    // --- JSON ---
    let json = serde_json::to_vec(&cfg).unwrap();
    println!("JSON: {}", String::from_utf8_lossy(&json));
    println!("JSON size: {} bytes", json.len());

    // --- YAML ---
    let yaml = serde_yaml::to_string(&cfg).unwrap();
    println!("YAML:\n{}", yaml);
    println!("YAML size: {} bytes", yaml.len());

    // --- CBOR ---
    let cbor = serde_cbor::to_vec(&cfg).unwrap();
    println!("CBOR (binary): {:?}", cbor);
    println!("CBOR size: {} bytes", cbor.len());

    // --- Deserialization with missing fields ---
    let partial_json = r#"{ "name": "partial" }"#;
    let cfg2: Config = serde_json::from_str(partial_json).unwrap();
    println!("Deserialized with defaults: {:?}", cfg2);
}
