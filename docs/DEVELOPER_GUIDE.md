# Developer Guide: The "Lifecycle" of a Component

Welcome! This guide is designed to take you from "Zero" to "Hero" by walking you through a single, concrete task.

**The Task:** You want to add a new **"Neon"** window style (glowing borders) to the system and use it in your setup.

---

## The Mental Model
Before we touch code, understand the flow. You cannot just "add a file." You must register it, expose it, and then select it.

1.  **Implementation:** Write the actual Hyprland config.
2.  **Registration:** Give it a Logical ID in the Registry.
3.  **Exposure:** Ensure the Theme allows this slot.
4.  **Selection:** Pick it in your User Profile.

---

## Step 1: Implementation (The Raw Config)
*Goal: Create the standard Hyprland config file.*

Create a new file at: `catalog/tunable/window_styles/neon.conf`

```ini
# catalog/tunable/window_styles/neon.conf
general {
    gaps_in = 4
    gaps_out = 10
    border_size = 3
    # Neon Pink and Blue gradient
    col.active_border = rgb(ff00ff) rgb(00ffff) 45deg
    col.inactive_border = rgb(595959)
}

decoration {
    rounding = 4
    shadow {
        enabled = true
        range = 20
        render_power = 3
        color = rgb(ff00ff)
    }
}

    Note: We do NOT include keybinds or monitors here. This component focuses strictly on visual window style.
```

Step 2: Registration (The Menu Entry)

Goal: Assign a "Logical ID" so themes can find it.

Open catalog/registry.toml. Add a new entry under the [tunable] section.
Ini, TOML

# catalog/registry.toml

[tunable.win_neon]  # <--- This is the Logical ID
path = "tunable/window_styles/neon.conf"
description = "High contrast neon borders with heavy shadow glow"

    Why do this? If you later decide to move neon.conf to a folder called catalog/experimental/, you only have to update this one line. The ID win_neon stays the same forever.

Step 3: Verification (The Theme Check)

Goal: Ensure your theme supports "Window Styles".

Open your theme definition (e.g., themes/modern_dark.toml).
Ini, TOML

# themes/modern_dark.toml

[slots]
# Check if a "window_style" slot exists.
# If it does, you don't need to change anything! 
# Your new component "fits" into this slot.
window_style = { default = "win_sleek" } 

    Developer Note: If you were adding a completely new category of component (like "cursor_theme"), you would need to define a new slot here. Since we are just adding a new option for an existing slot, no changes are needed here.

Step 4: Selection (The User Profile)

Goal: Actually use the new component.

Open your profile (e.g., profiles/my_setup.toml).
Ini, TOML

# profiles/my_setup.toml

[meta]
base_theme = "modern_dark"

[overrides]
# Tell the system to swap the default (Sleek) for your new one (Neon)
window_style = "win_neon"

Step 5: The Build

Goal: Generate the config and reload Hyprland.

Run the manager tool (assuming you are in the project root):
Bash

cargo run -- build --profile my_setup

What happens in the background?

    Resolver: Reads my_setup.toml. Sees you want win_neon.

    Lookup: Checks registry.toml. Finds win_neon -> catalog/tunable/window_styles/neon.conf.

    Generator: Writes hyprland.conf (or the sourced file):
    Ini, TOML

    # GENERATED FILE - DO NOT EDIT
    source = ~/.config/hypr/catalog/tunable/window_styles/neon.conf
    ... other components ...

    Reloader: Executes hyprctl reload.

Result: Your window borders should instantly turn neon pink and blue!
Troubleshooting
"Error: Slot 'window_style' not found in theme"

Cause: You tried to override window_style in your profile, but the Base Theme (modern_dark.toml) doesn't define that slot. Fix: Add window_style = { ... } to the [slots] section of the theme file.
"Error: Registry ID 'win_neon' not found"

Cause: Typo in registry.toml or you forgot to save the file. Fix: Ensure the key in registry.toml matches the string in your profile EXACTLY.
Hyprland crashes/shows red screen

Cause: Your neon.conf has invalid Hyprland syntax. Fix: Run hyprctl reload manually in a terminal to see the specific error message from Hyprland.
