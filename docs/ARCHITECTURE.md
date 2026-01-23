# Architecture Design

## Core Concept: The "Registry & Slot" Model
HRM uses a three-layer architecture to generate the final `hyprland.conf`:
1. **The Registry:** A global map of available components.
2. **The Theme:** A template defining required slots and defaults.
3. **The Profile:** The user's specific configuration state.

## Directory Structure
```text
~/.config/hypr/
├── manager/              # The Rust source code
├── hyprland.conf         # Auto-generated entry point (Do NOT edit manually)
├── catalog/              # The Component Database
│   ├── registry.toml     # Maps Logical IDs -> File Paths
│   ├── static/           # Hard dependencies (e.g., internal modules)
│   └── tunable/          # Configurable presets (visuals)
├── themes/               # Theme Definitions (Manifests)
└── profiles/             # User Recipes
