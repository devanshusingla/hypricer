use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "hyprricer", version = "2.0", about = "Reactive Theme Compiler for Hyprland")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compiles the active profile into a background daemon
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

    // 1. Define Paths
    let generated_dir = root.join("generated/source");
    let live_dir = root.join("live");
    let daemon_src_dir = generated_dir.join("src");

    // 2. Clean & Scaffold Directories
    println!("   🏗️  Scaffolding build environment...");
    if generated_dir.exists() {
        fs::remove_dir_all(&generated_dir)?;
    }
    fs::create_dir_all(&daemon_src_dir)?;
    fs::create_dir_all(&live_dir)?;

    // 3. Generate Daemon Cargo.toml
    // This defines the dependencies for the generated binary
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
# We will add more dependencies here as we add Watchers (e.g. playerctl)
"#;

    fs::write(generated_dir.join("Cargo.toml"), daemon_cargo_toml)
        .context("Failed to write daemon Cargo.toml")?;

    // 4. Generate a Skeleton main.rs (Placeholder)
    // We will populate this in Layer 4
    let daemon_main = r#"
#[tokio::main]
async fn main() {
    println!("Hello from the Generated Daemon! (Layer 1 Complete)");
}
"#;
    
    fs::write(daemon_src_dir.join("main.rs"), daemon_main)
        .context("Failed to write daemon main.rs")?;

    println!("   ✅ Scaffolding complete at ./generated/source/");
    
    // In future layers, we will:
    // 5. Parse Registry
    // 6. Inject Logic
    // 7. Run 'cargo build' in generated_dir

    Ok(())
}
