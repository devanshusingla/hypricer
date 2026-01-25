use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// --- The Master Registry ---
// This holds the merged data from all files.
#[derive(Debug, Default)]
pub struct Registry {
    pub static_components: HashMap<String, StaticDef>,
    pub watchers: HashMap<String, WatcherDef>,
    pub providers: HashMap<String, ProviderDef>,
}

// --- TOML Schemas ---
// These match the structure inside your .toml files

#[derive(Debug, Deserialize)]
pub struct RegistryFragment {
    #[serde(rename = "static", default)]
    pub static_components: HashMap<String, StaticDef>,
    
    #[serde(rename = "watcher", default)]
    pub watchers: HashMap<String, WatcherDef>,
    
    #[serde(rename = "provider", default)]
    pub providers: HashMap<String, ProviderDef>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StaticDef {
    pub path: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WatcherDef {
    pub provider: String, // e.g., "poll_cmd"
    pub cmd: String,
    pub interval: Option<u64>,
    pub output: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProviderDef {
    pub cmd: String,
    pub default: String, // Critical: Providers must have a default
}

impl Registry {
    pub fn load_from_dir(root: &Path) -> Result<Self> {
        let registry_dir = root.join("catalog/registry");
        let mut registry = Registry::default();

        if !registry_dir.exists() {
             // It's okay if it doesn't exist yet, just warn
             println!("   ⚠️  Warning: Registry directory not found at {:?}", registry_dir);
             return Ok(registry);
        }

        println!("   📚 Loading Registry from {:?}...", registry_dir);
        let paths = fs::read_dir(&registry_dir)?;

        for entry in paths {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                registry.merge_file(&path)?;
            }
        }
        
        println!("      Summary: {} Static | {} Watchers | {} Providers", 
            registry.static_components.len(),
            registry.watchers.len(),
            registry.providers.len()
        );

        Ok(registry)
    }

    fn merge_file(&mut self, path: &PathBuf) -> Result<()> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read registry file: {:?}", path))?;

        let fragment: RegistryFragment = toml::from_str(&content)
            .with_context(|| format!("Failed to parse TOML: {:?}", path))?;

        // Merge Static
        for (id, def) in fragment.static_components {
            if self.static_components.contains_key(&id) {
                return Err(anyhow!("Duplicate Static ID found: '{}' in {:?}", id, path));
            }
            self.static_components.insert(id, def);
        }

        // Merge Watchers
        for (id, def) in fragment.watchers {
            if self.watchers.contains_key(&id) {
                return Err(anyhow!("Duplicate Watcher ID found: '{}' in {:?}", id, path));
            }
            self.watchers.insert(id, def);
        }

        // Merge Providers
        for (id, def) in fragment.providers {
            if self.providers.contains_key(&id) {
                return Err(anyhow!("Duplicate Provider ID found: '{}' in {:?}", id, path));
            }
            self.providers.insert(id, def);
        }

        Ok(())
    }
}
