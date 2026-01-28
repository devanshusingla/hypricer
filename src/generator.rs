use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
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

    // --- VALIDATION STEP ---
    validate_requirements(registry)?;

    // Prepare Paths
    let gen_src = root.join("generated/source/src");
    let gen_logic = gen_src.join("logic");
    let live_conf = root.join("live/active_session.conf");

    // Inject Logic
    if gen_logic.exists() { fs::remove_dir_all(&gen_logic)?; }
    fs::create_dir_all(&gen_logic)?;

    let module_map = if !theme.dynamic.is_empty() {
        let map = inject_logic_modules(root, &theme, &gen_logic)?;
        generate_logic_mod(&gen_logic, &map)?;
        map
    } else {
        fs::write(gen_logic.join("mod.rs"), "")?;
        HashMap::new()
    };

    // Load Template
    let template_path = root.join(&theme.meta.template);
    let template_content = fs::read_to_string(&template_path).with_context(|| format!("Template not found: {:?}", template_path))?;

    // Generators
    let (watcher_code, watcher_inits) = generate_watchers(registry);
    let provider_logic = generate_providers(registry);
    let static_inits = generate_statics(root, &theme, registry);
    let logic_calls = generate_logic_calls(&module_map);

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
            refresh_and_render(&mut cache).await;
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
    let ctx = Context {{ data: data.clone() }};
    
    // Call dynamic logic functions
    let mut dynamic_results = HashMap::new();
{}
    
    // Merge data + dynamic results
    let mut all_data = data.clone();
    all_data.extend(dynamic_results);
    
    // Template Replacement
    let template = r##"{}"##;
    let mut output = template.to_string();

    // Replace {{{{ key }}}} with value
    for (k, v) in &all_data {{
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
        logic_calls,
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
    
    // Check Providers
    for (id, def) in &registry.providers {
        if let Some(checks) = &def.check {
            for (i, cmd) in checks.iter().enumerate() {
                run_check(id, &format!("Provider (check #{})", i+1), cmd)?;
            }
        }
    }

    // Check Watchers
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

fn generate_watchers(registry: &Registry) -> (String, String) {
    let mut code_defs = String::new();
    let mut code_inits = String::new();

    for (id, def) in &registry.watchers {
        let func_name = format!("watch_{}", id.replace(".", "_"));
        if def.provider == "poll_cmd" {
            let cmd = &def.cmd;
            let interval = def.interval.unwrap_or(5000);
            
            code_defs.push_str(&format!(
                r####"
async fn {func_name}(tx: mpsc::Sender<Event>) {{
    let mut interval = tokio::time::interval(Duration::from_millis({interval}));
    let mut last_val = String::new();
    loop {{
        interval.tick().await;
        let output = Command::new("sh").arg("-c").arg(r#"{cmd}"#).output();
        match output {{
            Ok(o) => {{
                let val = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if val != last_val {{
                    let _ = tx.send(Event {{ key: "{id}".to_string(), value: val.clone() }}).await;
                    last_val = val;
                }}
            }}
            Err(e) => eprintln!("Watcher failed: {{}}", e),
        }}
    }}
}}
"####,
                func_name = func_name,
                interval = interval,
                cmd = cmd,
                id = id
            ));

            code_inits.push_str(&format!(
                "    let tx_clone = tx.clone();\n    tokio::spawn({func_name}(tx_clone));\n",
                func_name = func_name
            ));
        }
    }
    (code_defs, code_inits)
}

fn generate_providers(registry: &Registry) -> String {
    let mut calls = String::new();
    for (id, def) in &registry.providers {
        // FIX 1: Use r##" (double hash) because the content contains r#" (single hash)
        calls.push_str(&format!(
            r##"    results.insert("{id}".to_string(), run_provider(r#"{cmd}"#, r#"{default}"#).await);
"##,
            id = id,
            cmd = def.cmd,
            default = def.default
        ));
    }

    // FIX 2: Reduced {{{{ to {{. 
    // format! turns {{ into { (which is what we want for valid Rust code).
    format!(
        r#"
async fn fetch_providers() -> HashMap<String, String> {{
    let mut results = HashMap::new();
{calls}
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
        calls = calls
    )
}

fn generate_logic_calls(module_map: &HashMap<String, String>) -> String {
    let mut calls = String::new();
    
    for (tag, module) in module_map {
        calls.push_str(&format!(
            r#"    dynamic_results.insert("{tag}".to_string(), logic::{module}::resolve(&ctx));
"#,
            tag = tag,
            module = module
        ));
    }
    
    calls
}

fn inject_logic_modules(
    root: &Path, 
    theme: &Theme, 
    gen_logic: &Path
) -> Result<HashMap<String, String>> {
    let mut module_map = HashMap::new();
    
    for (tag, rel_path) in &theme.dynamic {
        let src_file = root.join(rel_path);
        
        if !src_file.exists() {
            return Err(anyhow!("Dynamic logic file not found: {:?}", src_file));
        }
        
        let module_name = src_file
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid filename: {:?}", src_file))?;
        
        // Copy the file - avoid .rs being interpreted as a prefix
        let extension = ['.' as char, 'r' as char, 's' as char].iter().collect::<String>();
        let dst_file = gen_logic.join(format!("{}{}", module_name, extension));
        std::fs::copy(&src_file, &dst_file)
            .with_context(|| format!("Failed to copy {:?}", src_file))?;
        
        module_map.insert(tag.clone(), module_name.to_string());
        println!("   🧠 Injected: {} -> logic::{}", tag, module_name);
    }
    
    Ok(module_map)
}

fn generate_logic_mod(gen_logic: &Path, modules: &HashMap<String, String>) -> Result<()> {
    let mut mod_content = String::from("// Auto-generated module exports\n\n");
    
    for module_name in modules.values() {
        mod_content.push_str(&format!("pub mod {};\n", module_name));
    }
    
    std::fs::write(gen_logic.join("mod.rs"), mod_content)?;
    Ok(())
}
