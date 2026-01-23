# HyprRice Manager (HRM)

HRM is a robust, modular configuration manager for Hyprland, built in Rust. 

Unlike traditional "dotfile" collections that rely on fragile shell scripts and manual copying, HRM acts as a **Configuration Engine**. It separates the *definition* of a theme from the *implementation* of its components.

## Key Features
* **Registry System:** Decouples logical components (e.g., "sleek_window") from physical file paths. Move files without breaking themes.
* **Declarative Themes:** Themes define "Slots" (interfaces) that users can fill with different presets.
* **Profile Management:** Users create Recipes (Profiles) that mix and match themes and components dynamically.
* **Safe & Fast:** Built in Rust with strict schema validation.

## Roadmap
- [x] Core Schema Definition (Registry, Theme, Profile)
- [ ] Profile Builder Engine (Rust)
- [ ] CLI Tooling
- [ ] AUR Package Release
