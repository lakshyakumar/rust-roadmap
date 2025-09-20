// 86. How do you build a CLI with clap using derived Args and Subcommand?
// Add a --verbose flag to change the tracing level. Why is CLI ergonomics important?
use clap::{Parser, Subcommand};
use tracing::{debug, info};
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(name = "mycli", about = "A demo CLI with clap")]
struct Cli {
    /// Increase verbosity (-v, -vv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all items
    List,

    /// Add a new item
    Add {
        /// Name of the item
        #[arg(short, long)]
        name: String,
    },

    /// Remove an item by ID
    Remove {
        /// ID of the item
        #[arg(short, long)]
        id: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    // Setup tracing level based on --verbose count
    let level = match cli.verbose {
        0 => tracing::Level::INFO,
        1 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    tracing_subscriber::fmt().with_max_level(level).init();

    match cli.command {
        Commands::List => {
            info!("Listing items...");
        }
        Commands::Add { name } => {
            info!("Adding item: {}", name);
            debug!("Debug: Added with verbose flag set to {}", cli.verbose);
        }
        Commands::Remove { id } => {
            info!("Removing item {}", id);
        }
    }
}
