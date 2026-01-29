# User Guide

**Complete guide to using hypricer for end users**

---

## Table of Contents

1. [First-Time Setup](#first-time-setup)
2. [Daily Usage](#daily-usage)
3. [Managing Themes](#managing-themes)
4. [Configuration](#configuration)
5. [Tips & Tricks](#tips--tricks)
6. [FAQ](#faq)

---

## First-Time Setup

### System Requirements

- **OS**: Linux (tested on Arch, NixOS, Ubuntu 24.04+)
- **Compositor**: Hyprland 0.35+
- **Tools**: 
  - Rust 1.70+ (`rustc --version`)
  - Git (`git --version`)
  - Standard GNU coreutils

### Installation Steps

#### 1. Install Rust (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 2. Clone hypricer

```bash
cd ~/.config/hypr
git clone https://github.com/yourusername/hypricer
cd hypricer
```

#### 3. Build the CLI

```bash
cargo build --release
```

**Optional**: Add to PATH
```bash
sudo ln -s ~/.config/hypr/hypricer/target/release/hypricer /usr/local/bin/hypricer
```

#### 4. Integrate with Hyprland

Edit `~/.config/hypr/hyprland.conf`:

```ini
# At the very top of the file
source = ~/.config/hypr/hypricer/live/active_session.conf

# ... rest of your config
```

**Important**: This line MUST be at the top so hypricer can manage your theme.

#### 5. Apply Your First Theme

```bash
hypricer build --profile seiki
```

You should see:
```
🍚 hypricer v2.0
   📂 Root: "/home/user/.config/hypr/hypricer"
   📚 Loading Registry...
   🎨 Compiling Theme: 'Seiki (The Sanctuary)'
   🔍 Validating dependencies...
   🧠 Injected: window_style -> logic::style
   🧠 Injected: bar_style -> logic::bar
   ✅ Build Complete.
```

#### 6. Reload Hyprland

```bash
hyprctl reload
```

Your theme is now active!

---

## Daily Usage

### Checking Daemon Status

```bash
# See if daemon is running
ps aux | grep hrm_daemon

# View live logs
tail -f ~/.config/hypr/hypricer/live/daemon.log

# Follow logs in real-time
journalctl --user -f -u hrm_daemon  # if using systemd
```

### Switching Themes

```bash
# Build and apply a different theme
hypricer build --profile modern_dark

# Reload Hyprland
hyprctl reload
```

The daemon automatically restarts with the new theme.

### Viewing Active Configuration

```bash
# See the currently active config
cat ~/.config/hypr/hypricer/live/active_session.conf

# Check when it was last updated
stat ~/.config/hypr/hypricer/live/active_session.conf
```

### Updating hypricer

```bash
cd ~/.config/hypr/hypricer
git pull
cargo build --release

# Rebuild your current theme
hypricer build --profile seiki
```

---

## Managing Themes

### Listing Available Themes

```bash
ls ~/.config/hypr/hypricer/themes/
```

Each directory is a theme:
- `seiki/` - Advanced reactive theme
- `modern_dark/` - Simple starter theme
- `your_custom_theme/` - Your creations!

### Understanding Profiles

Profiles are in `profiles/*.toml` and reference themes:

```toml
# profiles/my_setup.toml
base_theme = "seiki"

[overrides]
# Future: Override specific components
```

**Why profiles?**
- Same theme, different settings
- Example: `gaming.toml` and `work.toml` both use `seiki` but with different watchers

### Installing Community Themes

```bash
cd ~/.config/hypr/hypricer/themes
git clone https://github.com/someone/amazing-theme
cd ../
hypricer build --profile amazing-theme
```

---

## Configuration

### Directory Structure

```
~/.config/hypr/hypricer/
├── catalog/
│   ├── registry/          # Component definitions
│   └── static/            # Shared config files
│
├── themes/
│   ├── seiki/             # Theme packages
│   └── modern_dark/
│
├── profiles/
│   ├── seiki.toml         # Profile definitions
│   └── default.toml
│
├── generated/             # Build artifacts (auto-generated)
│   └── source/
│
└── live/                  # Runtime files
    ├── active_session.conf  # Your active config
    └── daemon             # Running binary
```

### What You Can Modify

**✅ Safe to edit:**
- `themes/*/` - Your theme logic and templates
- `profiles/*.toml` - Profile definitions
- `catalog/registry/*.toml` - Add new watchers/providers

**⚠️ Auto-generated (changes will be overwritten):**
- `generated/` - Build artifacts
- `live/active_session.conf` - Active config

**❌ Don't touch:**
- `src/` - hypricer CLI source (unless contributing)

### Configuration Files Explained

#### Registry Files (`catalog/registry/*.toml`)

Define available components:
```toml
[watcher.cpu_usage]
provider = "poll_cmd"
interval = 2000
cmd = "top -bn1 | grep 'Cpu(s)' | awk '{print $2}'"
```

#### Theme Files (`themes/*/theme.toml`)

Wire components together:
```toml
[meta]
name = "My Theme"
template = "themes/mytheme/template.conf"

[static]
keybinds = "seiki_keybinds"

[dynamic]
window_style = "themes/mytheme/logic/style.rs"
```

#### Profile Files (`profiles/*.toml`)

Select which theme to use:
```toml
base_theme = "mytheme"

[overrides]
# Future feature
```

---

## Tips & Tricks

### Performance Tuning

**Reduce watcher intervals** for low-power setups:
```toml
# In catalog/registry/custom.toml
[watcher.battery]
interval = 30000  # 30s instead of 5s
```

**Disable unused watchers**:
Edit your theme's `theme.toml` and remove watchers from `inputs = [...]`

### Debugging

**Enable verbose logging**:
```bash
RUST_LOG=debug hypricer build --profile seiki
```

**Inspect generated code**:
```bash
cat generated/source/src/main.rs
```

**Check for compilation errors**:
```bash
cd generated/source
cargo build --release
```

### Custom Keybinds for Theme Switching

Add to your `hyprland.conf`:
```ini
# Quick theme switcher
bind = SUPER_SHIFT, T, exec, hypricer build --profile seiki && hyprctl reload
bind = SUPER_SHIFT, M, exec, hypricer build --profile modern_dark && hyprctl reload
```

### Auto-start Daemon on Login

**Using systemd**:

Create `~/.config/systemd/user/hrm_daemon.service`:
```ini
[Unit]
Description=Hypricer Theme Daemon
After=graphical-session.target

[Service]
Type=simple
ExecStart=%h/.config/hypr/hypricer/live/daemon
Restart=on-failure

[Install]
WantedBy=default.target
```

Enable:
```bash
systemctl --user enable hrm_daemon
systemctl --user start hrm_daemon
```

---

## FAQ

### Q: Can I use hypricer with other compositors?

**A:** No, hypricer is specifically designed for Hyprland. However, the concept could be adapted to other compositors.

### Q: Will this break my existing Hyprland config?

**A:** No. The `source = ...` line simply loads hypricer's config on top of yours. You can still have other settings in `hyprland.conf`.

### Q: How do I uninstall?

```bash
# Remove the source line from hyprland.conf
# Delete the directory
rm -rf ~/.config/hypr/hypricer
```

### Q: My theme isn't working!

**Checklist:**
1. Did you run `hypricer build`?
2. Did you reload Hyprland (`hyprctl reload`)?
3. Check logs: `tail -f ~/.config/hypr/hypricer/live/daemon.log`
4. Verify dependencies: Re-run build to see validation errors

### Q: Can I mix components from different themes?

**A:** Not directly yet, but it's planned! For now, you can copy logic files between themes manually.

### Q: How much RAM/CPU does the daemon use?

**A:** Minimal. Typically <5MB RAM and <1% CPU. Watchers are lightweight polling threads.

### Q: Can I contribute themes?

**A:** Absolutely! See the [Theme Developer Guide](theme-developer-guide.md) and submit a PR!

### Q: Does this work on Wayland only?

**A:** Yes, Hyprland is Wayland-only, so hypricer is too.

---

## Getting Help

- **Check logs**: Most issues are visible in `live/daemon.log`
- **Read error messages**: hypricer gives detailed errors during build
- **Ask the community**: [GitHub Discussions](https://github.com/yourusername/hypricer/discussions)
- **Report bugs**: [GitHub Issues](https://github.com/yourusername/hypricer/issues)

---

**Next Steps:**
- [Learn to create themes →](theme-developer-guide.md)
- [Explore the registry system →](registry-manual.md)
- [Understand the architecture →](architecture.md)
