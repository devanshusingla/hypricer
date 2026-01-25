use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub base_theme: String,
    #[serde(default)]
    pub overrides: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    pub meta: ThemeMeta,
    pub inputs: Option<Vec<String>>,
    #[serde(default)]
    pub static_components: HashMap<String, String>, // "apps" -> "apps_modern"
}

#[derive(Debug, Deserialize)]
pub struct ThemeMeta {
    pub name: String,
    pub template: String,
}
