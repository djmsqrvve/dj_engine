# System: Feature Grid (Spatial Logic & Data Layers)

## 1. Overview
The **Feature Grid** is a spatial data structure that manages world logic, navigation, and runtime analytics. Unlike visual tilemaps which only handle rendering, the Feature Grid handles the **Logical Topology** of the game world.

## 2. Spatial Layers
The grid partitions the world into discrete cells (typically 1x1 meter or tile units) across multiple independent data layers.

### Layer 0: Navigation (NavMesh)
*   **Purpose:** Defines walkable areas and obstructions for AI pathfinding.
*   **Data:** Binary (Walkable/Blocked) or Weighted (Cost to Traverse).
*   **Usage:** Used by the A* pathfinding system to generate entity routes.

### Layer 1: Logic Zones (Trigger Map)
*   **Purpose:** Spatially indexed triggers and event volumes.
*   **Data:** ID References to Script Events (e.g., "Zone_15" -> "SpawnEnemyPack").
*   **Usage:** Efficiently queries "What logic exists at [X, Y]?" without iterating all entities.

### Layer 2: Analytics (Heatmaps)
*   **Purpose:** Runtime data visualization for balancing and debugging.
*   **Data:** Float values representing metrics (e.g., Player Presence Time, Combat Intensity, Death Locations).
*   **Usage:** Designers view this layer to identify level flow issues (e.g., choke points or underutilized areas).

## 3. Technical Implementation
*   **Optimization:** The grid uses Sparse Sets or Chunked Hash Maps to handle large worlds efficiently, storing data only where logic exists to minimize memory footprint.
*   **Headless Validation:** Grid logic is fully decoupled from rendering. `tests/headless_tests.rs` validates that navigation and logic zones function correctly in a server-side environment.
