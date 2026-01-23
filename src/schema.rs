use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Registry {
    // Tell Serde: "When you see [static] in TOML, put it in this variable"
    #[serde(rename = "static")] 
    pub static_modules: HashMap<String, ComponentEntry>,
    
    #[serde(rename = "tunable")]
    pub tunable_modules: HashMap<String, ComponentEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentEntry {
    pub path: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    pub meta: ThemeMeta,
    #[serde(default)] 
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
    pub default: String, 
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub base_theme: String, 
    #[serde(default)]
    pub overrides: HashMap<String, String>, 
}
