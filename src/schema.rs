use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Registry {
    #[serde(rename = "static", default)] 
    pub static_modules: HashMap<String, ComponentEntry>,
    
    #[serde(rename = "tunable", default)]
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
    
    // 2. The Skeleton File (e.g., "themes/modern.conf")
    pub template: String, 
    
    // 3. The Mapper: "template_key" -> "registry_id"
    #[serde(default, rename = "static")] 
    pub static_map: HashMap<String, String>, 
    
    // 4. The Dynamic Slots
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
