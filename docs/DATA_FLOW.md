# Data Flow & Lifecycle

This document traces the lifecycle of a "Build Command" — what happens when the user runs `hrm build`.

## The Pipeline

```mermaid
graph TD
    A[User Profile] -->|Defines Base Theme & Overrides| B(The Builder Engine)
    C[Theme Manifest] -->|Defines Structure & Defaults| B
    D[Registry] -->|Resolves IDs to File Paths| B
    B -->|1. Load Profile| E{Resolution Logic}
    E -->|2. Load Theme| F[Identify Slots]
    F -->|3. Apply Overrides| G[Final Component List]
    G -->|4. Lookup Paths in Registry| H[List of Absolute Paths]
    H -->|5. Generate Config| I[Active Session File]
    I -->|6. Reload| J[Hyprland]
