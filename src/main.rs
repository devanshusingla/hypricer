mod registry; // <--- ADD THIS

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use registry::Registry; // <--- ADD THIS

// ... (Keep Cli and Commands structs exactly the same) ...
#[derive(Parser)]
#[command(name = "hyprricer", version = "2.0", about = "Reactive Theme Compiler for Hyprland")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long)]
        profile: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { profile } => {
            let root = std::env::current_dir()?;
            handle_build(&root, &profile)?;
        }
    }
    Ok(())
}

fn handle_build(root: &Path, profile_name: &str) -> Result<()> {
    println!("🍚 hyprricer v2.0");
    println!("   📂 Root: {:?}", root);
    println!("   ⚙️  Profile: {}", profile_name);

    // --- STEP 1: LOAD REGISTRY (LAYER 2) ---
    let registry = Registry::load_from_dir(root)?;
    
    // --- STEP 2: SCAFFOLDING (LAYER 1) ---
    let generated_dir = root.join("generated/source");
    let live_dir = root.join("live");
    let daemon_src_dir = generated_dir.join("src");

    println!("   🏗️  Scaffolding build environment...");
    if generated_dir.exists() {
        fs::remove_dir_all(&generated_dir)?;
    }
    fs::create_dir_all(&daemon_src_dir)?;
    fs::create_dir_all(&live_dir)?;

    // Generate Cargo.toml (Same as before)
    let daemon_cargo_toml = r#"
[package]
name = "hrm_daemon"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
shellexpand = "3.0"
once_cell = "1.18"
chrono = "0.4"
"#;
    fs::write(generated_dir.join("Cargo.toml"), daemon_cargo_toml)?;

    // Generate main.rs (Same as before)
    let daemon_main = r#"
#[tokio::main]
async fn main() {
    println!("Hello from the Generated Daemon! (Layer 1 Complete)");
}
"#;
    fs::write(daemon_src_dir.join("main.rs"), daemon_main)?;

    println!("   ✅ Scaffolding complete.");
    
    // In Layer 3, we will use 'registry' to generate code here
    
    Ok(())
}
