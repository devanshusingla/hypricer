# API Reference

**Complete reference for all hyprricer interfaces**

---

## Table of Contents

1. [CLI Commands](#cli-commands)
2. [Registry Specification](#registry-specification)
3. [Theme Specification](#theme-specification)
4. [Logic API](#logic-api)
5. [Template Syntax](#template-syntax)

---

## CLI Commands

### `hyprricer build`

Compile and apply a theme.

**Usage:**
```bash
hyprricer build --profile <PROFILE_NAME>
```

**Arguments:**
- `--profile, -p <NAME>`: Profile to build (required)

**Example:**
```bash
hyprricer build --profile seiki
```

**Process:**
1. Load registry from `catalog/registry/*.toml`
2. Load profile from `profiles/<NAME>.toml`
3. Load theme referenced in profile
4. Validate all dependencies (`check` commands)
5. Generate daemon source code
6. Copy to `generated/source/`
7. Theme is ready for manual compilation

**Exit Codes:**
- `0`: Success
- `1`: Build error (missing file, TOML parse error, etc.)
- `2`: Validation error (missing dependency)

---

### Future Commands (v2.1+)

```bash
# List available themes
hyprricer list themes

# List available profiles  
hyprricer list profiles

# Show daemon status
hyprricer status

# View logs
hyprricer logs [-f|--follow]

# Create new theme from template
hyprricer new theme <NAME>

# Validate without building
hyprricer check --profile <NAME>
```

---

## Registry Specification

Registry files define available components. They live in `catalog/registry/*.toml`.

### Static Components

**Purpose:** Reference to a fixed configuration file.

**Syntax:**
```toml
[static.<ID>]
path = "relative/path/from/root.conf"
description = "Optional human-readable description"
```

**Example:**
```toml
[static.apps_minimal]
path = "catalog/static/apps/minimal.conf"
description = "Terminal-only app suite"
```

**Fields:**
- `path` (required, string): Relative path from project root
- `description` (optional, string): Documentation

**Usage in themes:**
```toml
[static]
apps = "apps_minimal"  # References static.apps_minimal
```

---

### Watchers

**Purpose:** Background threads that monitor system state and emit events.

**Syntax:**
```toml
[watcher.<ID>]
provider = "poll_cmd" | "stream_cmd"  # Type of watcher
cmd = "shell command to execute"
interval = 5000  # Milliseconds (only for poll_cmd)
output = "string" | "json"  # Optional, default: string
check = "command" | ["cmd1", "cmd2"]  # Optional dependency check
```

**Providers:**

| Provider | Behavior | Use Case |
|----------|----------|----------|
| `poll_cmd` | Runs command periodically, emits event if output changes | CPU usage, time, battery |
| `stream_cmd` | Runs command once, reads stdout line-by-line | `playerctl --follow`, log tailing |

**Example (poll_cmd):**
```toml
[watcher.cpu_usage]
provider = "poll_cmd"
cmd = "top -bn1 | grep 'Cpu(s)' | awk '{print $2}'"
interval = 2000
check = "which top"
```

**Example (stream_cmd):**
```toml
[watcher.music_title]
provider = "stream_cmd"
cmd = "playerctl metadata --format '{{ title }}' --follow"
check = "which playerctl"
```

**Fields:**
- `provider` (required): `"poll_cmd"` or `"stream_cmd"`
- `cmd` (required, string): Shell command to execute
- `interval` (required for poll_cmd, number): Milliseconds between polls
- `output` (optional, string): Output format (currently unused)
- `check` (optional, string or array): Dependency validation command(s)

**Generated Code:**
```rust
async fn watch_<ID>(tx: mpsc::Sender<Event>) {
    // For poll_cmd:
    loop {
        let output = Command::new("sh").arg("-c").arg(cmd).output();
        if output_changed {
            tx.send(Event { key: "<ID>", value: output }).await;
        }
        sleep(interval);
    }
}
```

---

### Providers

**Purpose:** On-demand data fetchers called when events occur.

**Syntax:**
```toml
[provider.<ID>]
cmd = "shell command"
default = "fallback value if command fails"
check = "command" | ["cmd1", "cmd2"]  # Optional
```

**Example:**
```toml
[provider.current_wallpaper]
cmd = "swww query | grep 'currently displaying' | cut -d' ' -f8"
default = "unknown.png"
check = "which swww"
```

**Fields:**
- `cmd` (required, string): Shell command to execute
- `default` (required, string): Fallback value (used on timeout or error)
- `check` (optional, string or array): Dependency validation

**Timeout:** All providers have a 200ms timeout. If exceeded, `default` is used.

**Generated Code:**
```rust
async fn run_provider(cmd: &str, default_val: &str) -> String {
    let timeout = Duration::from_millis(200);
    match tokio::time::timeout(timeout, execute_cmd(cmd)).await {
        Ok(output) => output,
        Err(_) => default_val.to_string()
    }
}
```

---

## Theme Specification

Theme files define how components are wired together. They live in `themes/<NAME>/theme.toml`.

### Theme Structure

```toml
[meta]
name = "Display Name"
version = "1.0"
author = "Your Name"  # Optional
template = "path/to/template.conf"

[inputs]
# (Currently unused, reserved for future)

[static]
tag_name = "registry_static_id"

[dynamic]
tag_name = "path/to/logic.rs"
```

### Meta Section

**Fields:**
- `name` (required, string): Display name
- `version` (optional, string): Theme version
- `author` (optional, string): Author name
- `template` (required, string): Path to template file (relative to project root)

**Example:**
```toml
[meta]
name = "Cyber Punk 2077"
version = "2.1.0"
author = "Johnny Silverhand"
template = "themes/cyberpunk/template.conf"
```

---

### Static Section

Maps template tags to registry static components.

**Syntax:**
```toml
[static]
<template_tag> = "<registry_static_id>"
```

**Example:**
```toml
[static]
apps = "apps_minimal"
keybinds = "seiki_keybinds"
```

**In template:**
```conf
{{ apps }}
{{ keybinds }}
```

**Result:**
```conf
source = /full/path/to/catalog/static/apps/minimal.conf
source = /full/path/to/themes/seiki/components/keybinds.conf
```

---

### Dynamic Section

Maps template tags to Rust logic files.

**Syntax:**
```toml
[dynamic]
<template_tag> = "path/to/logic/file.rs"
```

**Example:**
```toml
[dynamic]
window_style = "themes/mytheme/logic/window.rs"
bar_config = "themes/mytheme/logic/bar.rs"
```

**Logic file requirements:**
- Must export a function: `pub fn resolve(ctx: &Context) -> String`
- Path is relative to project root

**In template:**
```conf
{{ window_style }}
{{ bar_config }}
```

**At runtime:**
The daemon calls `logic::window::resolve(&ctx)` and replaces `{{ window_style }}` with the returned string.

---

## Logic API

### Context Structure

Your logic functions receive a `Context` containing all current system state.

**Definition:**
```rust
pub struct Context {
    pub data: HashMap<String, String>,
}
```

**The `data` HashMap contains:**
- All watcher values (key = watcher ID)
- All provider values (key = provider ID)
- All static component values (key = static tag from theme)

**Example:**
```rust
{
    "cpu_usage": "45",
    "time_part": "Day",
    "current_window": "firefox",
    "apps": "source = /path/to/apps.conf"
}
```

---

### Logic Function Signature

**Required:**
```rust
use crate::Context;

pub fn resolve(ctx: &Context) -> String {
    // Your logic here
    "result".to_string()
}
```

**Parameters:**
- `ctx`: Reference to `Context` struct

**Returns:**
- `String`: The text to replace the template tag with

**Example:**
```rust
use crate::Context;

pub fn resolve(ctx: &Context) -> String {
    // Get CPU usage (with fallback)
    let cpu = ctx.data.get("cpu_usage")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    
    // Conditional logic
    let color = if cpu > 70 {
        "rgba(ff4444ee)"
    } else {
        "rgba(33ccffee)"
    };
    
    // Return config snippet
    format!("general {{ col.active_border = {} }}", color)
}
```

---

### Available Utilities

You can use any Rust std library:

```rust
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

pub fn resolve(ctx: &Context) -> String {
    // File system access
    if Path::new("/tmp/dark_mode").exists() {
        return "dark".to_string();
    }
    
    // Environment variables
    if let Ok(user) = env::var("USER") {
        if user == "work" {
            return "minimal".to_string();
        }
    }
    
    // Complex data parsing
    let values: Vec<i32> = ctx.data.values()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    format!("average = {}", values.iter().sum::<i32>() / values.len() as i32)
}
```

**Note:** Logic runs inside the daemon, so:
- ✅ Fast (compiled code)
- ✅ Type-safe (won't compile if broken)
- ❌ Can't make network requests (no tokio in logic scope)
- ❌ Should be deterministic (avoid randomness)

---

### Testing Logic Functions

You can unit test your logic:

```rust
// themes/mytheme/logic/window.rs
use crate::Context;
use std::collections::HashMap;

pub fn resolve(ctx: &Context) -> String {
    // ... your logic
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_high_cpu() {
        let mut data = HashMap::new();
        data.insert("cpu_usage".to_string(), "80".to_string());
        
        let ctx = Context { data };
        let result = resolve(&ctx);
        
        assert!(result.contains("ff4444"));  // Red color
    }
}
```

Run tests:
```bash
cd generated/source
cargo test
```

---

## Template Syntax

Templates are standard Hyprland config files with variable substitution.

### Basic Syntax

```conf
# Regular Hyprland config
monitor=,preferred,auto,1
input {
    kb_layout = us
}

# Variable substitution
{{ tag_name }}

# Variables can be anywhere
$my_var = {{ some_value }}
bind = $mainMod, T, exec, {{ terminal_command }}
```

### Tag Resolution

Tags are replaced based on theme definition:

**Static tags:**
```toml
# In theme.toml
[static]
apps = "apps_minimal"
```

```conf
# In template.conf
{{ apps }}

# Becomes:
source = /full/path/to/catalog/static/apps/minimal.conf
```

**Dynamic tags:**
```toml
# In theme.toml
[dynamic]
window_style = "themes/mytheme/logic/window.rs"
```

```conf
# In template.conf
{{ window_style }}

# Becomes (at runtime, based on logic function):
general { col.active_border = rgba(33ccffee) }
```

**Provider/Watcher tags:**
These are available in the `Context` passed to logic functions, not directly in templates.

---

### Multi-line Tags

Tags are replaced with single-line or multi-line content:

```conf
# Single-line replacement
$terminal = {{ terminal_app }}

# Multi-line replacement
{{ window_decorations }}
# Could become:
# decoration {
#     rounding = 10
#     blur {
#         enabled = true
#     }
# }
```

---

### Escaping

If you need literal `{{` in your config:

```conf
# This will be replaced
{{ tag }}

# This will stay as-is (future feature for escaping)
\{{ not_a_tag }}
```

**Current limitation:** No escaping mechanism exists yet. Avoid `{{` in comments or strings.

---

### Best Practices

1. **Use descriptive tag names:**
   ```conf
   # ❌ Unclear
   {{ s1 }}
   
   # ✅ Clear
   {{ window_decorations }}
   ```

2. **Group related tags:**
   ```conf
   # Startup
   {{ autostart_apps }}
   {{ daemon_services }}
   
   # Visuals
   {{ window_rules }}
   {{ decorations }}
   ```

3. **Comment your template:**
   ```conf
   # =================================================================
   # WINDOW MANAGEMENT
   # =================================================================
   {{ window_rules }}
   
   # =================================================================
   # KEYBINDS
   # =================================================================
   {{ keybinds }}
   ```

4. **Keep static content in template:**
   ```conf
   # Don't dynamically generate this
   monitor=,preferred,auto,1
   
   # Do dynamically generate this
   {{ theme_colors }}
   ```

---

## Type Reference

### Event Structure

```rust
#[derive(Debug, Clone)]
pub struct Event {
    pub key: String,    // Watcher ID
    pub value: String,  // Command output
}
```

### Context Structure

```rust
pub struct Context {
    pub data: HashMap<String, String>,
}
```

**Access patterns:**
```rust
// Safe access with fallback
let cpu = ctx.data.get("cpu_usage").unwrap_or("0");

// Parse to number
let cpu_num: i32 = ctx.data.get("cpu_usage")
    .and_then(|s| s.parse().ok())
    .unwrap_or(0);

// Check existence
if ctx.data.contains_key("music_playing") {
    // ...
}

// Iterate all values
for (key, value) in &ctx.data {
    println!("{}: {}", key, value);
}
```

---

## Version Compatibility

| hyprricer Version | Rust Version | Hyprland Version |
|-------------------|--------------|------------------|
| 2.0.x             | 1.70+        | 0.35+            |
| 2.1.x (planned)   | 1.75+        | 0.36+            |

**Breaking changes between versions will be documented in [CHANGELOG.md](../CHANGELOG.md).**

---

## See Also

- [User Guide](user-guide.md) - Using hyprricer
- [Theme Developer Guide](theme-developer-guide.md) - Creating themes
- [Registry Manual](registry-manual.md) - Extending the registry
- [Architecture](architecture.md) - How it all works
