use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Deserializer}; // Added Deserializer
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct Registry {
    pub static_components: HashMap<String, StaticDef>,
    pub watchers: HashMap<String, WatcherDef>,
    pub providers: HashMap<String, ProviderDef>,
}

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
    pub provider: String,
    pub cmd: String,
    pub interval: Option<u64>,
    pub output: Option<String>,
    
    // FIX: Use custom deserializer to handle String OR List
    #[serde(default, deserialize_with = "string_or_vec")]
    pub check: Option<Vec<String>>, 
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProviderDef {
    pub cmd: String,
    pub default: String,
    
    // FIX: Use custom deserializer here too
    #[serde(default, deserialize_with = "string_or_vec")]
    pub check: Option<Vec<String>>,
}

// --- HELPER FUNCTION: Allows check = "cmd" OR check = ["cmd1", "cmd2"] ---
fn string_or_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }

    Option::<StringOrVec>::deserialize(deserializer).map(|opt| match opt {
        Some(StringOrVec::String(s)) => Some(vec![s]),
        Some(StringOrVec::Vec(v)) => Some(v),
        None => None,
    })
}

impl Registry {
    pub fn load_from_dir(root: &Path) -> Result<Self> {
        let registry_dir = root.join("catalog/registry");
        let mut registry = Registry::default();

        if !registry_dir.exists() {
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

        for (id, def) in fragment.static_components {
            if self.static_components.contains_key(&id) {
                return Err(anyhow!("Duplicate Static ID found: '{}' in {:?}", id, path));
            }
            self.static_components.insert(id, def);
        }
        for (id, def) in fragment.watchers {
            if self.watchers.contains_key(&id) {
                return Err(anyhow!("Duplicate Watcher ID found: '{}' in {:?}", id, path));
            }
            self.watchers.insert(id, def);
        }
        for (id, def) in fragment.providers {
            if self.providers.contains_key(&id) {
                return Err(anyhow!("Duplicate Provider ID found: '{}' in {:?}", id, path));
            }
            self.providers.insert(id, def);
        }
        Ok(())
    }
}
