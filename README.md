# hyprricer 🍚

<div align="center">

**The Reactive Theme Engine for Hyprland**

*Compile your rice into a high-performance, event-driven daemon*

[![Version](https://img.shields.io/badge/version-2.0-blue)](https://github.com/yourusername/hyprricer)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org)

[Features](#-features) • [Quick Start](#-quick-start) • [Documentation](#-documentation) • [Examples](#-examples) • [Contributing](#-contributing)

</div>

---

## 🎯 What is hyprricer?

**hyprricer** is not just another dotfile manager. It's a **theme transpiler** that compiles declarative theme definitions into optimized Rust binaries that run as system daemons.

### The Problem It Solves

Traditional Linux theming approaches suffer from:
- 🐌 **Shell script overhead** - Every theme change spawns new processes
- 💥 **Runtime failures** - Missing dependencies crash your desktop
- 🔄 **Manual updates** - You have to trigger theme changes yourself
- 🐛 **No validation** - Broken configs only fail at runtime

### The hyprricer Solution

- ⚡ **Zero-latency reactions** - Native code execution, no shell overhead
- 🛡️ **Build-time validation** - Dependencies checked before compilation
- 🎨 **Truly reactive themes** - Your desktop adapts to system state automatically
- 🔧 **Type-safe logic** - Theme logic is Rust code that must compile

**Example**: Window borders automatically change color based on CPU load, time of day, or active application - with zero latency and complete type safety.

---

## ✨ Features

### For Users

- **🚀 Instant Updates** - Changes apply in <1ms, compiled to native code
- **🎭 Reactive Themes** - Desktop adapts to music, battery, time, CPU, and more
- **🛡️ Dependency Safety** - Build fails if required tools aren't installed
- **📦 Modular Design** - Mix and match components from different themes
- **🔍 Transparent** - Inspect generated Rust code for debugging

### For Theme Developers

- **🦀 Rust-Powered Logic** - Write theme logic in a real programming language
- **📚 Rich Registry System** - Extensible catalog of watchers, providers, and components
- **🔄 Hot Reload** - Rebuild and restart daemon with one command
- **🎨 Template System** - Familiar `{{ variable }}` syntax in configs
- **🧪 Type Safety** - Invalid theme logic won't compile

---

## 🚀 Quick Start

### Prerequisites

- **Hyprland** (obviously!)
- **Rust & Cargo** (1.70+)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Common tools** (optional, theme-dependent):
  - `jq` - JSON parsing
  - `hyprctl` - Hyprland control (comes with Hyprland)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/hyprricer ~/.config/hypr/hyprricer
   cd ~/.config/hypr/hyprricer
   ```

2. **Build the CLI tool**
   ```bash
   cargo build --release
   ```

3. **Add to your Hyprland config**
   
   Edit `~/.config/hypr/hyprland.conf` and add at the **top**:
   ```ini
   source = ~/.config/hypr/hyprricer/live/active_session.conf
   ```

4. **Build and apply a theme**
   ```bash
   ./target/release/hyprricer build --profile seiki
   ```

### Verify Installation

```bash
# Check if the daemon is running
ps aux | grep hrm_daemon

# View live logs
tail -f ~/.config/hypr/hyprricer/live/daemon.log

# Reload Hyprland to see changes
hyprctl reload
```

---

## 📚 Documentation

### User Guides
- **[Installation Guide](docs/installation.md)** - Detailed setup instructions
- **[User Manual](docs/user-guide.md)** - Using hyprricer day-to-day
- **[Troubleshooting](docs/troubleshooting.md)** - Common issues and solutions

### Developer Guides
- **[Theme Developer Guide](docs/theme-developer-guide.md)** - Creating custom themes
- **[Registry Manual](docs/registry-manual.md)** - Extending the component catalog
- **[Architecture Overview](docs/architecture.md)** - How hyprricer works internally
- **[API Reference](docs/api-reference.md)** - Complete API documentation

### Examples
- **[Seiki Theme](themes/seiki/)** - Advanced reactive theme showcase
- **[Modern Dark](themes/modern_dark/)** - Simple starter theme

---

## 🎨 Examples

### Reactive Window Borders

**Theme logic** (`themes/myrice/logic/style.rs`):
```rust
use crate::Context;

pub fn resolve(ctx: &Context) -> String {
    let cpu = ctx.data.get("cpu_usage")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    
    let color = match cpu {
        0..=30 => "rgba(33ccffee)",    // Blue: Peace Mode
        31..=70 => "rgba(ffaa00ee)",   // Orange: Active
        _ => "rgba(ff4444ee)",         // Red: Mecha Mode!
    };
    
    format!("general {{ col.active_border = {} }}", color)
}
```

### Time-Based Themes

**Registry definition** (`catalog/registry/time.toml`):
```toml
[watcher.time_part]
provider = "poll_cmd"
interval = 60000  # Check every minute
cmd = """
hour=$(date +%H);
if [ "$hour" -ge 6 ] && [ "$hour" -lt 9 ]; then echo "Dawn";
elif [ "$hour" -ge 9 ] && [ "$hour" -lt 17 ]; then echo "Day";
elif [ "$hour" -ge 17 ] && [ "$hour" -lt 20 ]; then echo "Sunset";
else echo "Night"; fi
"""
```

**Theme logic**:
```rust
pub fn resolve(ctx: &Context) -> String {
    match ctx.data.get("time_part").map(|s| s.as_str()) {
        Some("Dawn") => "exec = swww img ~/wallpapers/dawn.png",
        Some("Day") => "exec = swww img ~/wallpapers/day.png",
        Some("Sunset") => "exec = swww img ~/wallpapers/sunset.png",
        _ => "exec = swww img ~/wallpapers/night.png",
    }.to_string()
}
```

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Build Time                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  1. hyprricer CLI                                               │
│     │                                                           │
│     ├─ Load Registry (catalog/registry/*.toml)                 │
│     │   • Watchers (system event listeners)                    │
│     │   • Providers (on-demand data fetchers)                  │
│     │   • Static components (config files)                     │
│     │                                                           │
│     ├─ Load Theme (themes/*/theme.toml)                        │
│     │   • Metadata                                             │
│     │   • Static component selections                          │
│     │   • Dynamic logic references                             │
│     │                                                           │
│     ├─ Validate Dependencies                                    │
│     │   • Run all 'check' commands                             │
│     │   • Fail fast if tools missing                           │
│     │                                                           │
│     └─ Generate Daemon Source                                   │
│         • Copy logic/*.rs → generated/src/logic/               │
│         • Generate main.rs with watchers, providers            │
│         • Inject template.conf content                         │
│                                                                 │
│  2. Cargo Build                                                 │
│     │                                                           │
│     └─ Compile generated source → Binary daemon                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                         Runtime                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Daemon (generated/target/release/hrm_daemon)                   │
│  │                                                              │
│  ├─ Watchers (background threads)                              │
│  │   └─ Push events to coordinator: Event(key, value)         │
│  │                                                              │
│  ├─ Coordinator (main loop)                                    │
│  │   ├─ Receives events from watchers                         │
│  │   ├─ Debounces (50ms) to avoid thrashing                   │
│  │   ├─ Fetches all providers (parallel, 200ms timeout)       │
│  │   ├─ Calls theme logic functions                           │
│  │   └─ Updates live/active_session.conf                      │
│  │                                                              │
│  └─ Hyprland                                                    │
│      └─ Automatically reloads when config changes              │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Key Design Principles:**
- **Separation of Concerns**: Build-time (validation) vs Runtime (execution)
- **Fail Fast**: Dependency issues caught during build, not at 3am
- **Performance**: Native code, zero shell overhead
- **Transparency**: Generated code is human-readable Rust

---

## 🎭 Featured Themes

### Seiki (The Sanctuary)
A showcase of hyprricer's capabilities featuring:
- ✨ Three distinct modes (Peace, Mecha, Focus)
- 🌅 Day/night cycle with automatic wallpaper switching
- 🎵 Music-reactive styling
- ⚡ CPU load-based border colors

[View Seiki Documentation →](themes/seiki/README.md)

### Modern Dark
A simple, elegant starter theme perfect for learning:
- 🎨 Clean, minimal aesthetics
- 📱 Good defaults for modern workflows
- 🔧 Easy to customize

[View Modern Dark Documentation →](themes/modern_dark/README.md)

---

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### Reporting Issues
- 🐛 **Bug reports**: Use the issue template, include logs
- 💡 **Feature requests**: Describe the use case clearly
- 📚 **Documentation**: Typos, clarity improvements

### Contributing Code
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test thoroughly (see [CONTRIBUTING.md](CONTRIBUTING.md))
5. Submit a pull request

### Contributing Themes
Themes are especially welcome! See the [Theme Developer Guide](docs/theme-developer-guide.md).

### Contributing Registry Items
Add new watchers, providers, or components to `catalog/registry/`. See the [Registry Manual](docs/registry-manual.md).

---

## 📋 Roadmap

### v2.1 (Next Release)
- [ ] `stream_cmd` provider (for `playerctl --follow` style commands)
- [ ] File system watchers (`inotify` integration)
- [ ] DBus signal watchers
- [ ] Web UI for theme management
- [ ] Theme marketplace/gallery

### v2.2 (Future)
- [ ] Multi-monitor support with per-monitor themes
- [ ] Theme inheritance system
- [ ] Live theme preview mode
- [ ] Performance profiling tools

[View full roadmap →](ROADMAP.md)

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **Hyprland** - The amazing Wayland compositor this is built for
- **The Rust Community** - For incredible tooling and libraries
- **Unixporn Community** - Inspiration and feedback

---

## 💬 Community

- **Discord**: [Join our server](https://discord.gg/yourserver) (coming soon)
- **Reddit**: [r/hyprricer](https://reddit.com/r/hyprricer) (coming soon)
- **Matrix**: `#hyprricer:matrix.org` (coming soon)

---

<div align="center">

**Made with ❤️ and Rust for the Hyprland Community**

*If you find this project useful, consider giving it a ⭐ on GitHub!*

[Report Bug](https://github.com/yourusername/hyprricer/issues) • [Request Feature](https://github.com/yourusername/hyprricer/issues) • [Discussions](https://github.com/yourusername/hyprricer/discussions)

</div>
