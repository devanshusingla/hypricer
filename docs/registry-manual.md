# hyprricer: Registry Manual

**Target Audience:** Power Users & Registry Maintainers

The **Registry** (`~/.config/hypr/hyprricer/catalog/registry/`) is the database of building blocks. A Theme Developer cannot use a "CPU Watcher" or a "Neon Window Style" unless it is first defined here.

`hyprricer` recursively reads **all `.toml` files** in this directory. You can organize them however you like (e.g., `audio.toml`, `system.toml`, `visuals.toml`).

---

## 1. The Four Component Types

Every entry in the registry falls into one of these four tables:

| Type | Table Name | Role |
| :--- | :--- | :--- |
| **Static** | `[static]` | A fixed configuration file (e.g., a specific waybar config). |
| **Tunable** | `[tunable]` | *Reserved for v2.1*. Currently acts like Static but intended for parametric configs. |
| **Watcher** | `[watcher]` | An **Async Trigger**. Spawns a thread to listen for system events. |
| **Provider** | `[provider]` | A **Sync Enricher**. Fetches data on-demand during an update cycle. |

---

## 2. Defining Components

### 2.1 Static Components
These map a unique ID to a physical file. When a theme requests `{{ apps }}`, and the registry resolves it to `apps_minimal`, the content of the referenced file is injected.

```toml
# File: catalog/registry/apps.toml

[static.apps_minimal]
path = "catalog/static/apps/minimal.conf"
description = "Terminal-only setup for focus"

[static.apps_full]
path = "catalog/static/apps/full.conf"
description = "Full desktop suite with background apps"
```

**Fields:**
* `path`: Relative path from the `hyprricer` root.
* `description`: (Optional) Helpful text for UI tools.

---

## 3. Defining Inputs (Watchers & Providers)

This is where you define how `hyprricer` talks to the OS.

### 3.1 Watchers (`[watcher]`)
Watchers run continuously in the background. They are responsible for saying **"Something changed!"**

**Required Fields:**
* `provider`: The internal Rust module to use. Currently supports:
    * `"poll_cmd"`: Runs a command repeatedly. Triggers if output changes.
    * `"stream_cmd"`: Runs a command once and listens to STDOUT line-by-line.

**Example: Battery Monitor (Polling)**
```toml
# File: catalog/registry/system.toml

[watcher.battery_status]
provider = "poll_cmd"
cmd = "cat /sys/class/power_supply/BAT0/status"
interval = 5000  # Check every 5000ms (5 seconds)
output = "string"
```

**Example: Music Monitor (Streaming)**
```toml
# File: catalog/registry/media.toml

[watcher.music_metadata]
provider = "stream_cmd"
cmd = "playerctl metadata --format '{{ title }}' --follow"
output = "string"
check = "which playerctl" # Build fails if playerctl is missing
```

### 3.2 Providers (`[provider]`)
Providers run **only when a Watcher triggers**. They gather extra context needed to make decisions. They must be fast.

**Critical Rule:** You **MUST** provide a `default` value. If the command fails or times out (200ms limit), the default value is used to prevent the desktop from freezing.

**Example: Get Current Wallpaper**
```toml
# File: catalog/registry/context.toml

[provider.current_wallpaper]
cmd = "swww query | grep 'currently displaying' | cut -d ' ' -f 8"
default = "unknown.png" # <--- REQUIRED
```

### 3.3 Dependency Safety (The `check` field)

To prevent the daemon from crashing at runtime, you can define build-time checks for your components. `hyprricer` will run these commands during the build. If any command fails (exit code != 0), the build will abort with a helpful error.

You can provide a single command or a list of commands.

**Supported on:** `[watcher]`, `[provider]`

**Examples:**

```toml
# Single Check
[watcher.time]
provider = "poll_cmd"
cmd = "date +%S"
check = "which date"

# Multiple Checks (All must pass)
[provider.weather]
cmd = "curl -s 'wttr.in?format=1' | jq -r .text"
default = "offline"
check = ["which curl", "which jq"]
```

---

## 4. Best Practices

1.  **Modularize Files:** Don't dump everything into one file. Use `system.toml` for hardware, `rice.toml` for visual styles, etc.
2.  **Unique IDs:** Keys must be globally unique. You cannot have `[static.my_bar]` in two different files.
3.  **Safe Defaults:** Always assume your command might fail. If your "Weather Provider" fails because of no internet, your default should be `"offline"`, not an empty string or crash.
4.  **Use `check`:** If your watcher relies on a tool like `playerctl` or `jq`, add a `check = "which tool"` line. This helps users debug missing dependencies during the build process.

---

## 5. Troubleshooting Registry Errors

* **Error: "Duplicate Key"** -> You defined the same ID in two files.
* **Error: "Path Not Found"** -> Your `path = "..."` does not point to a valid file relative to the config root.
* **Error: "Missing Default"** -> A `[provider]` is missing its `default` field.
