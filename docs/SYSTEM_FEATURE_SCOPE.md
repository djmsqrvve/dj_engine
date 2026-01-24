# System: Feature Scope & Configuration

## 1. Challenge: Grid Complexity
In a complex game, the Feature Grid (Hex or Orthogonal) becomes crowded. A single cell might contain:
*   Navigation data (Walkable)
*   Stealth logic (Light Level)
*   Combat triggers (Spawn Rate)
*   Narrative events (Dialogue Trigger)

When working on a specific feature (e.g., "Stealth Overhaul"), relying on a "God View" of all active logic is overwhelming.

## 2. Solution: Scope Lenses
The editor implements **Scope Lenses** to filter the active view and editing context.

### The Mechanism
*   **The Global Scope:** The sum of all active logic in the current Branch.
*   **The Lens:** A viewport filter that isolates specific **Feature Sets**.
    *   *Example:* Selecting the "Stealth Lens" renders `LightLevel` and `VisionCones` overlays but hides `LootTables` and `DialogueTriggers`.

## 3. Implementation: Feature Sets
A **Feature Set** is a container for a specific domain of game logic.
*   **Definition:** A collection of `GridLayers`, `AssetBundles`, and `StorySchemas` grouped by purpose.
*   **config.json Structure:**
    ```json
    {
      "feature_set": "stealth_mechanics_v2",
      "active_layers": ["visibility", "noise_propagation"],
      "story_modules": ["guard_reaction_graph"]
    }
    ```

## 4. Branch Inheritance
How does the Branch Graph interact with Scope?
1.  **Inheritance:** A new "Test Plane" (Branch) inherits the **Active Feature Sets** of its parent (usually Core).
2.  **Overriding:** The designer can "Pin" a different version of a Feature Set to the branch.
    *   *Use Case:* Core uses `stealth_v1`. Branch A uses `stealth_v2`. Both share `inventory_v1`.
3.  **Visualization:** The **BOM** (Bill of Materials) panel lists currently active Feature Sets.

## 5. Visual Organization
*   **Hex/Grid View:** When a Lens is active, irrelevant grid cells desaturate or vanish. Relevant cells glow with the Feature Set's assigned color (e.g., Stealth = Blue, Combat = Red).
*   **Dependency Warning:** If editing a shared layer (e.g., modifying "Wall Collision" while in "Combat Mode"), the system warns that this change affects other scopes.
