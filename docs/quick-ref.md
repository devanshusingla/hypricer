# hypricer Quick Reference

**One-page cheat sheet for common tasks**

---

## Installation

```bash
# Clone
git clone https://github.com/yourusername/hypricer ~/.config/hypr/hypricer
cd ~/.config/hypr/hypricer

# Build
cargo build --release

# Add to Hyprland config (at the top of hyprland.conf)
echo "source = ~/.config/hypr/hypricer/live/active_session.conf" \
  >> ~/.config/hypr/hyprland.conf
```

---

## Daily Commands

```bash
# Build and apply a theme
hypricer build --profile seiki

# Reload Hyprland (apply changes)
hyprctl reload

# View daemon logs
tail -f ~/.config/hypr/hypricer/live/daemon.log

# Check if daemon is running
pgrep -a hrm_daemon

# Kill daemon
pkill hrm_daemon
```

---

## Directory Structure

```
~/.config/hypr/hypricer/
├── catalog/registry/    # Component definitions (watchers, providers)
├── themes/              # Theme packages
├── profiles/            # Profile selections
├── generated/           # Build artifacts (auto-generated)
└── live/                # Runtime files
    ├── active_session.conf  # Active config (sourced by Hyprland)
    └── daemon           # Running binary
```

---

## Creating a Simple Theme

```bash
# 1. Create theme directory
mkdir -p themes/mytheme/logic
cd themes/mytheme

# 2. Create theme.toml
cat > theme.toml << 'EOF'
[meta]
name = "My Theme"
template = "themes/mytheme/template.conf"

[static]
apps = "apps_modern"
EOF

# 3. Create template.conf
cat > template.conf << 'EOF'
{{ apps }}

general {
    border_size = 2
}
EOF

# 4. Create profile
cat > ../../profiles/mytheme.toml << 'EOF'
base_theme = "mytheme"
EOF

# 5. Build it
cd ../../
hypricer build --profile mytheme
```

---

## Adding a Watcher

```toml
# In catalog/registry/custom.toml

[watcher.my_watcher]
provider = "poll_cmd"
interval = 5000                     # Check every 5 seconds
cmd = "echo Hello"                  # Your command here
check = "which echo"                # Dependency check
```

**Use in theme:**
```toml
# In themes/mytheme/theme.toml
inputs = ["my_watcher"]
```

---

## Adding a Provider

```toml
# In catalog/registry/custom.toml

[provider.my_data]
cmd = "echo World"
default = "fallback"                # REQUIRED
check = "which echo"
```

---

## Writing Theme Logic

```rust
// themes/mytheme/logic/style.rs
use crate::Context;

pub fn resolve(ctx: &Context) -> String {
    // Get data
    let value = ctx.data.get("my_watcher")
        .unwrap_or("default");
    
    // Return config snippet
    format!("# Value: {}", value)
}
```

**Reference in theme.toml:**
```toml
[dynamic]
my_tag = "themes/mytheme/logic/style.rs"
```

**Use in template.conf:**
```conf
{{ my_tag }}
```

---

## Common Template Tags

```conf
# Static components (from registry)
{{ apps }}           # App definitions
{{ keybinds }}       # Keybindings
{{ decorations }}    # Visual settings

# Dynamic (from logic functions)
{{ window_style }}   # Window appearance
{{ bar_config }}     # Bar settings
{{ wallpaper }}      # Wallpaper command
```

---

## Debugging

```bash
# Check generated source
cat generated/source/src/main.rs

# Manually compile daemon
cd generated/source
cargo build --release

# Run daemon in foreground (see output)
./target/release/hrm_daemon

# Check for compilation errors
cargo check
```

---

## Common Watcher Providers

| Provider | When to Use | Example |
|----------|-------------|---------|
| `poll_cmd` | Command runs repeatedly | CPU, time, battery |
| `stream_cmd` | Command outputs continuously | `playerctl --follow` |

---

## Registry Component Types

| Type | Purpose | Example |
|------|---------|---------|
| `[static.<id>]` | Fixed config file | Keybinds, app lists |
| `[watcher.<id>]` | Background monitor | CPU usage, time |
| `[provider.<id>]` | On-demand data | Current wallpaper |

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| Build fails | Check `check` commands are satisfied |
| Daemon not starting | Rebuild: `hypricer build --profile X` |
| Config not updating | Reload: `hyprctl reload` |
| Watcher not firing | Check logs: `tail -f live/daemon.log` |

---

## Environment Variables

```bash
# Enable debug logging
RUST_LOG=debug hypricer build --profile seiki
```

---

## File Locations

| File | Purpose |
|------|---------|
| `~/.config/hypr/hyprland.conf` | Your Hyprland config (add source line here) |
| `~/.config/hypr/hypricer/live/active_session.conf` | Generated config (sourced by Hyprland) |
| `~/.config/hypr/hypricer/live/daemon.log` | Runtime logs |

---

## Keyboard Shortcuts (add to hyprland.conf)

```ini
# Quick theme switcher
bind = SUPER_SHIFT, T, exec, hypricer build --profile seiki && hyprctl reload
bind = SUPER_SHIFT, M, exec, hypricer build --profile modern_dark && hyprctl reload

# Reload without rebuild
bind = SUPER_SHIFT, R, exec, hyprctl reload
```

---

## Performance Tips

```toml
# Increase watcher intervals (reduce CPU usage)
[watcher.cpu_usage]
interval = 5000  # Instead of 1000

# Use simple commands (faster)
cmd = "date +%H"  # Good: fast
cmd = "find / -name '*.log' | wc -l"  # Bad: slow
```

---

## Getting Help

- **Documentation**: `docs/` folder
- **Issues**: https://github.com/yourusername/hypricer/issues
- **Discussions**: https://github.com/yourusername/hypricer/discussions

---

## Links

- [Full Documentation](docs/)
- [User Guide](docs/user-guide.md)
- [Theme Developer Guide](docs/theme-developer-guide.md)
- [API Reference](docs/api-reference.md)
- [Troubleshooting](docs/troubleshooting.md)

---

**Print this page or bookmark it for quick reference!**
