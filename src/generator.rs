use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::Path;
use crate::registry::Registry;
use crate::structs::{Profile, Theme};

pub fn generate(root: &Path, registry: &Registry, profile_name: &str) -> Result<()> {
    // 1. Load Profile & Theme
    let profile_path = root.join("profiles").join(format!("{}.toml", profile_name));
    let profile_str = fs::read_to_string(&profile_path)
        .with_context(|| format!("Profile not found: {:?}", profile_path))?;
    let profile: Profile = toml::from_str(&profile_str)?;

    let theme_dir = root.join("themes").join(&profile.base_theme);
    let theme_path = theme_dir.join("theme.toml");
    let theme_str = fs::read_to_string(&theme_path)
        .with_context(|| format!("Theme not found: {:?}", theme_path))?;
    let theme: Theme = toml::from_str(&theme_str)?;

    println!("   🎨 Compiling Theme: '{}' ({})", theme.meta.name, profile.base_theme);

    // 2. Prepare Paths
    let gen_src = root.join("generated/source/src");
    let gen_logic = gen_src.join("logic");
    let live_conf = root.join("live/active_session.conf");

    // 3. Inject Logic (Copy themes/x/logic -> generated/src/logic)
    if gen_logic.exists() { fs::remove_dir_all(&gen_logic)?; }
    fs::create_dir_all(&gen_logic)?;
    
    let user_logic_dir = theme_dir.join("logic");
    if !user_logic_dir.exists() {
        return Err(anyhow!("Theme is missing 'logic' directory: {:?}", user_logic_dir));
    }
    
    copy_dir_recursive(&user_logic_dir, &gen_logic)?;
    println!("   🧠 Injected Logic from {:?}", user_logic_dir);

    // 4. Load Template
    let template_path = root.join(&theme.meta.template);
    let template_content = fs::read_to_string(&template_path)
        .with_context(|| format!("Template not found: {:?}", template_path))?;

    // 5. Generate Watchers
    let (watcher_code, watcher_inits) = generate_watchers(registry);

    // 6. Generate Main Daemon
    // FIX: Using r####" to avoid conflict with inner r##" strings
    let main_code = format!(
        r####"
mod logic; // Import the user's logic

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

// The Context passed to user logic
pub struct Context {{
    pub data: HashMap<String, String>,
}}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    println!("🍚 Daemon Started for '{}'");
    
    let (tx, mut rx) = mpsc::channel(32);
    let mut cache: HashMap<String, String> = HashMap::new();
    let mut last_update = Instant::now();
    
    // --- WATCHERS ---
    {}

    // --- COORDINATOR LOOP ---
    println!("   👂 Waiting for events...");
    
    // Initial Render
    update_config(&cache);

    while let Some(event) = rx.recv().await {{
        println!("   ✨ Event: {{:?}}", event);
        
        // 1. Update State
        cache.insert(event.key, event.value);

        // 2. Debounce (Simple)
        if last_update.elapsed().as_millis() > 50 {{
            update_config(&cache);
            last_update = Instant::now();
        }}
    }}

    Ok(())
}}

fn update_config(data: &HashMap<String, String>) {{
    let ctx = Context {{ data: data.clone() }};
    
    // 1. Run User Logic (For now, we just proceed to template replacement)
    // In the future, we will call logic::resolve(&ctx) here.
    
    let template = r##"{}"##;
    let mut output = template.to_string();

    // Simple replacement: {{ key }} -> value
    for (k, v) in data {{
        output = output.replace(&format!("{{{{ {{}} }}}}", k), v);
    }}

    // Write to Live Config
    let path = "{}";
    if let Err(e) = fs::write(path, output) {{
        eprintln!("❌ Failed to write config: {{}}", e);
    }} else {{
        println!("   💾 Config Updated");
    }}
}}

// --- GENERATED WATCHER LOGIC ---
{}
"####,
        theme.meta.name,
        watcher_inits,
        template_content, 
        live_conf.to_string_lossy(),
        watcher_code
    );

    fs::write(gen_src.join("main.rs"), main_code)?;

    Ok(())
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
        match def.provider.as_str() {
            "poll_cmd" => {
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
            _ => {}
        }
    }
    (code_defs, code_inits)
}
