// 87. How do you layer configuration loading in Rust (default, file, env, CLI args)?
// Merge with precedence using serde, envy, and clap. What are the challenges of config layering?
use clap::Parser;
use config;
use serde::Deserialize; // Add this import for the config crate

/// Application configuration
#[derive(Debug, Deserialize, Clone)]
struct AppConfig {
    #[serde(default = "default_host")]
    host: String,

    #[serde(default = "default_port")]
    port: u16,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}
fn default_port() -> u16 {
    8080
}

/// CLI args override everything else
#[derive(Parser, Debug)]
#[command(name = "config-demo")]
struct Cli {
    /// Override host
    #[arg(long)]
    host: Option<String>,

    /// Override port
    #[arg(long)]
    port: Option<u16>,

    /// Config file path
    #[arg(long, default_value = "Config.toml")]
    config: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // 1. Start with defaults via serde default functions
    let mut settings = config::Config::builder()
        .set_default("host", default_host())?
        .set_default("port", default_port())?;

    // 2. Load from config file (TOML/JSON/YAML)
    settings = settings.add_source(config::File::with_name(&cli.config).required(false));

    // 3. Load from environment variables (APP_HOST, APP_PORT)
    settings = settings.add_source(config::Environment::with_prefix("APP"));

    // Build the merged config
    let mut config: AppConfig = settings.build()?.try_deserialize()?;

    // 4. Apply CLI overrides
    if let Some(h) = cli.host {
        config.host = h;
    }
    if let Some(p) = cli.port {
        config.port = p;
    }

    println!("Final Config: {:?}", config);

    Ok(())
}
