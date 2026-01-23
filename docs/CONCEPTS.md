# Core Concepts

If you are new to the codebase, start here. The HyprRice Manager (HRM) operates on four main entities. Think of it like ordering food at a restaurant.

## 1. The Component (The Dish)
**What is it?** A raw text file containing a valid chunk of Hyprland configuration.
**Analogy:** A single dish, like "Spicy Fries" or "Cola".
**Location:** `catalog/tunable/` or `catalog/static/`
* *Example:* A file defining a specific window border style.

## 2. The Registry (The Menu)
**What is it?** A central list that assigns a unique "Logical ID" to every Component file path.
**Analogy:** The physical menu that says "Item #45 is Spicy Fries."
**Why?** If the chef moves the fries to a different fridge, the menu number (#45) stays the same. The customer doesn't need to know where the fries are stored.
**Location:** `catalog/registry.toml`

## 3. The Theme (The Combo Meal)
**What is it?** A definition of a visual style. It lists which "Slots" (categories) are available and what the default choices are.
**Analogy:** A "Burger Combo" deal. It dictates that you *must* get a Drink and a Side, and defaults to standard Fries and Coke.
**Location:** `themes/modern_dark.toml`

## 4. The Profile (The Custom Order)
**What is it?** The user's specific configuration. It selects a base Theme and overrides specific Slots.
**Analogy:** "I want the Burger Combo, but swap the Fries for Onion Rings."
**Location:** `profiles/my_setup.toml`
