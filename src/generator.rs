use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::Path;
use crate::registry::Registry;
use crate::structs::{Profile, Theme};

pub fn generate(root: &Path, registry: &Registry, profile_name: &str) -> Result<()> {
    let profile_path = root.join("profiles").join(format!("{}.toml", profile_name));
    let profile_str = fs::read_to_string(&profile_path).with_context(|| format!("Profile not found: {:?}", profile_path))?;
    let profile: Profile = toml::from_str(&profile_str)?;

    let theme_dir = root.join("themes").join(&profile.base_theme);
    let theme_path = theme_dir.join("theme.toml");
    let theme_str = fs::read_to_string(&theme_path).with_context(|| format!("Theme not found: {:?}", theme_path))?;
    let theme: Theme = toml::from_str(&theme_str)?;

    println!("   🎨 Compiling Theme: '{}' ({})", theme.meta.name, profile.base_theme);

    // --- NEW: VALIDATION STEP ---
    validate_requirements(registry)?;
    // ----------------------------

    // Prepare Paths
    let gen_src = root.join("generated/source/src");
    let gen_logic = gen_src.join("logic");
    let live_conf = root.join("live/active_session.conf");

    // Inject Logic
    if gen_logic.exists() { fs::remove_dir_all(&gen_logic)?; }
    fs::create_dir_all(&gen_logic)?;
    let user_logic_dir = theme_dir.join("logic");
    if user_logic_dir.exists() {
        copy_dir_recursive(&user_logic_dir, &gen_logic)?;
        println!("   🧠 Injected Logic from {:?}", user_logic_dir);
    } else {
        fs::write(gen_logic.join("mod.rs"), "")?;
    }

    // Load Template
    let template_path = root.join(&theme.meta.template);
    let template_content = fs::read_to_string(&template_path).with_context(|| format!("Template not found: {:?}", template_path))?;

    // Generators
    let (watcher_code, watcher_inits) = generate_watchers(registry);
    let provider_logic = generate_providers(registry);
    let static_inits = generate_statics(root, &theme, registry);

    let main_code = format!(
        r####"
mod logic; 

use tokio::sync::mpsc;
use std::process::Command;
use std::time::{{Duration, Instant}};
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Event {{
    pub key: String,
    pub value: String,
}}

pub struct Context {{
    pub data: HashMap<String, String>,
}}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    println!("🍚 Daemon Started for '{}'");
    
    let (tx, mut rx) = mpsc::channel(32);
    let mut cache: HashMap<String, String> = HashMap::new();
    let mut last_update = Instant::now();

    // --- STATIC DEFAULTS ---
    {} 

    // --- WATCHERS ---
    {}

    // --- COORDINATOR LOOP ---
    println!("   👂 Waiting for events...");
    
    // Initial Render
    refresh_and_render(&mut cache).await;

    while let Some(event) = rx.recv().await {{
        println!("   ✨ Event: {{:?}}", event);
        
        cache.insert(event.key, event.value);

        if last_update.elapsed().as_millis() > 50 {{
            refresh_and_render(&mut cache).await; // Fetch providers on event
            last_update = Instant::now();
        }}
    }}

    Ok(())
}}

async fn refresh_and_render(cache: &mut HashMap<String, String>) {{
    let provider_data = fetch_providers().await;
    cache.extend(provider_data);
    update_config(cache);
}}

fn update_config(data: &HashMap<String, String>) {{
    let _ctx = Context {{ data: data.clone() }};
    
    // Template Replacement
    let template = r##"{}"##;
    let mut output = template.to_string();

    // Replace {{ key }} with value
    for (k, v) in data {{
        output = output.replace(&format!("{{{{{{{{ {{}} }}}}}}}}", k), v);
    }}

    let path = "{}";
    if let Err(e) = fs::write(path, output) {{
        eprintln!("❌ Failed to write config: {{}}", e);
    }} else {{
        println!("   💾 Config Updated");
    }}
}}

// --- GENERATED PROVIDER LOGIC ---
{}

// --- GENERATED WATCHER LOGIC ---
{}
"####,
        theme.meta.name,
        static_inits,
        watcher_inits,
        template_content, 
        live_conf.to_string_lossy(),
        provider_logic,
        watcher_code
    );

    fs::write(gen_src.join("main.rs"), main_code)?;
    Ok(())
}

fn validate_requirements(registry: &Registry) -> Result<()> {
    println!("   🔍 Validating dependencies...");
    
    // 1. Check Providers
    for (id, def) in &registry.providers {
        if let Some(checks) = &def.check {
            // FIX: Iterate over all checks in the list
            for (i, cmd) in checks.iter().enumerate() {
                run_check(id, &format!("Provider (check #{})", i+1), cmd)?;
            }
        }
    }

    // 2. Check Watchers
    for (id, def) in &registry.watchers {
        if let Some(checks) = &def.check {
            for (i, cmd) in checks.iter().enumerate() {
                run_check(id, &format!("Watcher (check #{})", i+1), cmd)?;
            }
        }
    }
    
    Ok(())
}

fn run_check(id: &str, kind: &str, cmd: &str) -> Result<()> {
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .with_context(|| format!("Failed to execute check for {} '{}'", kind, id))?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "\n❌ Missing dependency for {} '{}'.\n   Command '{}' failed.\n   Please install the required tool and try again.\n", 
            kind, id, cmd
        ));
    }
    Ok(())
}

fn generate_statics(root: &Path, theme: &Theme, registry: &Registry) -> String {
    let mut code = String::new();
    for (local_key, global_id) in &theme.static_components {
        if let Some(def) = registry.static_components.get(global_id) {
            let abs_path = root.join(&def.path).to_string_lossy().to_string();
            // We format it as a Hyprland source command
            let value = format!("source = {}", abs_path);
            code.push_str(&format!(
                "    cache.insert(\"{}\".to_string(), r#\"{}\"#.to_string());\n", 
                local_key, value
            ));
        } else {
            println!("   ⚠️  Warning: Static component '{}' not found in registry.", global_id);
        }
    }
    code
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_recursive(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn generate_watchers(registry: &Registry) -> (String, String) {
    let mut code_defs = String::new();
    let mut code_inits = String::new();

    for (id, def) in &registry.watchers {
        let func_name = format!("watch_{}", id.replace(".", "_"));
        if def.provider == "poll_cmd" {
            let cmd = &def.cmd;
            let interval = def.interval.unwrap_or(5000);
            
            code_defs.push_str(&format!(
                r#"
async fn {}(tx: mpsc::Sender<Event>) {{
    let mut interval = tokio::time::interval(Duration::from_millis({}));
    let mut last_val = String::new();
    loop {{
        interval.tick().await;
        let output = Command::new("sh").arg("-c").arg("{}").output();
        match output {{
            Ok(o) => {{
                let val = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if val != last_val {{
                    let _ = tx.send(Event {{ key: "{}".to_string(), value: val.clone() }}).await;
                    last_val = val;
                }}
            }}
            Err(e) => eprintln!("Watcher failed: {{}}", e),
        }}
    }}
}}
"#,
                func_name, interval, cmd, id
            ));

            code_inits.push_str(&format!(
                "    let tx_clone = tx.clone();\n    tokio::spawn({}(tx_clone));\n", 
                func_name
            ));
        }
    }
    (code_defs, code_inits)
}

fn generate_providers(registry: &Registry) -> String {
    let mut calls = String::new();
    for (id, def) in &registry.providers {
        calls.push_str(&format!(
            "    results.insert(\"{}\".to_string(), run_provider(r#\"{}\"#, r#\"{}\"#).await);\n",
            id, def.cmd, def.default
        ));
    }

    format!(
        r#"
async fn fetch_providers() -> HashMap<String, String> {{
    let mut results = HashMap::new();
{}
    return results;
}}

async fn run_provider(cmd: &str, default_val: &str) -> String {{
    let timeout = Duration::from_millis(200);
    let task = async {{
        let output = Command::new("sh").arg("-c").arg(cmd).output();
        match output {{
            Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
            Err(_) => default_val.to_string(),
        }}
    }};
    match tokio::time::timeout(timeout, task).await {{
        Ok(val) => val,
        Err(_) => default_val.to_string(),
    }}
}}
"#,
        calls
    )
}
