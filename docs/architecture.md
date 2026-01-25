# System Architecture

**Project:** hyprricer
**Version:** 2.0
**Architecture Type:** Reactive State-Machine Transpiler

## 1. High-Level Overview

`hyprricer` is not a traditional runtime interpreter. It functions as a **Compiler/Transpiler**. It reads abstract Theme definitions (TOML/Conf) and compiles them into a highly optimized, type-safe **Rust Binary** (The Daemon).

This architecture separates the "Build Phase" from the "Runtime Phase," ensuring zero parsing overhead and maximum stability during actual usage.

### 1.1 The Compilation Pipeline
When the user runs `hyprricer build`, the following steps occur:
1.  **Parsing:** `hyprricer` reads the Registry, Profile, and Theme package.
2.  **Validation:** It executes all `check` commands defined in the Registry. If any tool (e.g., `playerctl`, `jq`) is missing, the build stops immediately.
3.  **Scaffolding:** It creates a persistent Cargo project in the config folder.
    * **Location:** `~/.config/hypr/hyprricer/generated/source/`
    * *Benefit:* Users can inspect the generated Rust code for debugging.
4.  **Injection:**
    * **Watchers:** Spawns threads for registered inputs (e.g., `playerctl`, `file_watch`).
    * **Logic:** Copies user-defined Rust logic (`logic/*.rs`) into the crate.
    * **Templates:** Converts `template.conf` into Rust format strings.
5.  **Compilation:** Runs `cargo build --release`.
6.  **Installation:** Moves the final binary to the `live/` directory.
7.  **Hot Reload:** Kills the old daemon and starts the new one.

---

## 2. The Runtime Daemon (The Generated Binary)

The generated binary is a **Single-Threaded Coordinator** that manages asynchronous events. It uses the **Actor Model** to prevent race conditions and deadlocks.

### 2.1 The "Four Pillars" of Runtime
The system is built on four distinct concepts that separate concerns:

| Component | Role | Behavior |
| :--- | :--- | :--- |
| **Watcher** | **Trigger** | Spawns a background thread. Pushes lightweight "Change Events" to the Coordinator. (e.g., "Song Changed") |
| **Provider** | **Enricher** | A short-lived task that fetches heavy data on-demand. Has a strict timeout. (e.g., "Get Current Wallpaper") |
| **Context** | **State** | A read-only snapshot containing all current data (Watcher Cache + Provider Results). |
| **Logic** | **Resolver** | Pure Rust functions provided by the Theme. Takes `Context` -> Returns `Component IDs`. |

### 2.2 The Coordinator Loop
To handle rapid events (e.g., scrolling through a playlist) without freezing the UI, the Daemon implements a **Debounced State Machine**.

1.  **Event Received:** Watcher sends `Event("music", "playing")`.
2.  **Debounce:** Coordinator updates Cache & Resets Debounce Timer (e.g., 50ms).
3.  **Enrichment Phase (Parallel):**
    * Coordinator spawns all Providers simultaneously.
    * **Global Timeout:** Any provider taking >200ms is killed.
    * **Fallback:** Killed providers return their `default` value from Registry.
4.  **Resolution Phase (Sequential):**
    * **Construct Context:** Merges Watcher Cache + Provider Results.
    * **Resolve Logic:** Calls `logic::resolve(Context)`.
5.  **Actuation:** Writes Config to `live/active_session.conf` (If Changed).

---

## 3. Directory Structure

`hyprricer` uses a clear separation between **Source Configs** (Themes/Registry) and **Runtime Artifacts** (`live/`).

```text
~/.config/hypr/hyprricer/
├── live/               # Runtime Artifacts (The "Active" System)
│   ├── active_session.conf # The Final Output (Source this in hyprland.conf)
│   ├── daemon              # The Compiled Binary (Running process)
│   └── daemon.log          # Runtime Logs
│
├── generated/          # Intermediate Build Artifacts
│   └── source/         # The Generated Rust Project (Inspectable)
│
├── catalog/
│   ├── registry/       # The Definition Layer
│   │   ├── system.toml     # Defines: Battery, CPU, Memory watchers
│   │   ├── media.toml      # Defines: Playerctl watcher, CoverArt provider
│   │   └── styles.toml     # Defines: Window Styles, Animations
│   │
│   ├── static/         # The File Layer (Raw Configs)
│   └── tunable/        # The Parameter Layer
│
├── themes/
│   └── modern_dark/    # The Package Layer
│       ├── theme.toml      # Manifest: Wires Watchers -> Logic
│       ├── template.conf   # Structure: {{ tags }}
│       └── logic/          # The Brain Layer
│           ├── mod.rs
│           ├── derived.rs  # Setup hook (Data -> Semantics)
│           └── window.rs   # Component Resolver
│
└── profiles/
    └── my_setup.toml   # The Selection Layer
```

---

## 4. Failure Handling Strategies

### 4.1 "Slow Neighbor" Protection
If a Theme requires multiple pieces of data (e.g., Battery + Network), a slow network response must not block the UI.
* **Strategy:** All Providers are spawned in `FuturesUnordered` (Parallel).
* **Constraint:** The Enrichment Phase has a **Global Timeout** (e.g., 200ms).
* **Outcome:** If Network takes 2s, it is killed at 200ms. The Context is built using the `default` value defined in the Registry.

### 4.2 Zombie Process Cleanup
Since Watchers are threads inside the Daemon process:
* **Switching Themes:** `hyprricer` sends `SIGTERM` to the old Daemon.
* **Result:** The OS immediately closes all threads, file handles (inotify), and socket connections (dbus). No cleanup code is required.

---

## 5. Development Workflow

### 5.1 For Registry Maintainers
* Define **capabilities** in `catalog/registry/*.toml`.
* Provide safe defaults for all Providers.

### 5.2 For Theme Developers
* **Input:** Declare required inputs in `theme.toml` (`inputs = ["music", "battery"]`).
* **Processing:** Write standard Rust functions in `logic/`.
* **Output:** Return Component IDs that match keys in the Registry.

---

## 6. Future Extensibility
The Registry supports a `provider` field for Watchers. This allows us to easily add new backends in the future without changing the core architecture:
* `provider = "poll_cmd"` (Implemented)
* `provider = "fs_watch"` (Planned)
* `provider = "dbus_signal"` (Planned)
* `provider = "socket_listen"` (Planned)
```
