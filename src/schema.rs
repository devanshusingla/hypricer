use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// The Registry: Maps logical IDs to physical paths
#[derive(Debug, Deserialize, Serialize)]
pub struct Registry {
    pub static_modules: HashMap<String, ComponentEntry>,
    pub tunable_modules: HashMap<String, ComponentEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentEntry {
    pub path: String,
    pub description: Option<String>,
}

/// The Theme Manifest: Defines which slots are available
#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    pub meta: ThemeMeta,
    #[serde(default)] // Optional: Defaults to empty list
    pub static_reqs: Vec<String>, 
    pub slots: HashMap<String, SlotDef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeMeta {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SlotDef {
    pub default: String, // The default Registry ID
    pub description: Option<String>,
}

/// The User Profile: The actual "save file" for a user's setup
#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub base_theme: String, // e.g., "modern_dark"
    #[serde(default)]
    pub overrides: HashMap<String, String>, // Slot Name -> Registry ID
}
