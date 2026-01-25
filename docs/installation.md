# hyprricer: Installation & Usage Guide

**hyprricer** is a next-generation theme engine for Hyprland. It compiles themes into native code for maximum performance and instant responsiveness.

---

## 1. Prerequisites

Before installing, ensure you have the following dependencies:

* **Hyprland:** (Obviously)
* **Rust & Cargo:** Required to compile the themes.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    ```
* **Common Tools:** Most themes will rely on these (though strictly optional for the core engine):
    * `playerctl` (For music status)
    * `swww` or `hyprpaper` (For wallpapers)
    * `jq` (For parsing JSON data)

---

## 2. Installation

### Step 1: Clone the Repository
Clone `hyprricer` into your Hyprland config directory.

```bash
mkdir -p ~/.config/hypr
git clone [https://github.com/YourRepo/hyprricer.git](https://github.com/YourRepo/hyprricer.git) ~/.config/hypr/hyprricer
cd ~/.config/hypr/hyprricer
```

### Step 2: Build the CLI Tool
Build the main `hyprricer` management tool.

```bash
cargo build --release
# Optional: Add to PATH or symlink
sudo ln -s ~/.config/hypr/hyprricer/target/release/hyprricer /usr/local/bin/hyprricer
```

---

## 3. Configuration

### Step 1: Hook into Hyprland
Open your main `hyprland.conf` (usually `~/.config/hypr/hyprland.conf`) and add this line at the **top**:

```ini
# Load the active hyprricer session
source = ~/.config/hypr/hyprricer/live/active_session.conf
```

*Note: If the file doesn't exist yet, don't worry. The first build will create it.*

### Step 2: Select a Profile
`hyprricer` comes with default profiles. List them and pick one.

```bash
# List available profiles
hyprricer list profiles

# Example output:
# - default
# - minimal_work
# - gaming_rgb
```

---

## 4. Usage

### Building & Applying a Theme
To compile and apply a theme, use the `build` command with a profile.

```bash
hyprricer build --profile default
```

**What happens next?**
1.  `hyprricer` reads the profile and the associated theme.
2.  It compiles a custom **Daemon** specifically for that theme.
3.  It starts the Daemon in the background.
4.  Your Hyprland config updates instantly.

### Troubleshooting
If something isn't working (e.g., wallpaper isn't changing), check the live logs:

```bash
tail -f ~/.config/hypr/hyprricer/live/daemon.log
```

---

## 5. Updates
To update `hyprricer` itself:

```bash
cd ~/.config/hypr/hyprricer
git pull
cargo build --release
```
