mod registry;
mod generator;
mod structs;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::Path;
use registry::Registry;

#[derive(Parser)]
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

    // 1. Load Registry
    let registry = Registry::load_from_dir(root)?;
    
    // 2. Scaffold (Keep your scaffolding logic here or move to generator, 
    //    but for now we just rely on generator to overwrite main.rs)
    //    (Make sure you kept the directory creation logic from Layer 1!)
    let generated_dir = root.join("generated/source");
    let src_dir = generated_dir.join("src");
    let live_dir = root.join("live");
    
    if !generated_dir.exists() {
        std::fs::create_dir_all(&src_dir)?;
        std::fs::create_dir_all(&live_dir)?;
        
        // Write Cargo.toml (Layer 1 logic - re-add if you deleted it)
        let cargo_toml = r#"
[package]
name = "hrm_daemon"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
"#;
        std::fs::write(generated_dir.join("Cargo.toml"), cargo_toml)?;
    }

    // 3. Generate Code (Layer 3)
    println!("   ⚙️  Generating Daemon Code...");
    generator::generate(root, &registry, profile_name)?;

    println!("   ✅ Build Complete. (Ready for compilation)");
    Ok(())
}
