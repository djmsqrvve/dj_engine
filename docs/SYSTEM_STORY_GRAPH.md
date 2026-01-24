# System: Story Graph (Visual Narrative Scripting)

## 1. Overview
The **Story Graph** is a node-based visual scripting environment designed to manage complex branching narratives and game logic flow. It replaces linear scripts with a graph data structure that is easier to verify, visualize, and debug.

## 2. Graph Architecture
The system implements a directed graph where nodes represent **Game States** and edges represent **Control Flow**.
*   **Nodes (Vertices):** Encapsulate atomic logic units (e.g., "Display Text", "Wait for Input", "Branch Condition").
*   **Edges (Connections):** Define the valid transitions between states.
*   **Execution Pointer:** The runtime executes the graph by traversing connected nodes starting from a root (Start Node).

## 3. Node Types & Logic
Data structures are defined in `crate::data::story`.

### Flow Control Nodes
*   **Start:** Entry point for a graph execution context.
*   **End:** Terminates the current graph execution.
*   **Choice (Branch):** A multi-output node that diverts flow based on user input (Action) or variable state (Condition).
*   **SubGraph (Container):** Modular encapsulation. Executes a nested graph file and returns flow upon completion.

### Content Nodes
*   **Dialogue:** Displays narrative content and waits for continuation.
*   **Action:** Executes a game command (e.g., "Add Item", "Play Sound") without blocking flow.

## 4. Implementation Details
*   **Data Model:** `StoryGraphData` contains a list of `StoryNodeData`.
*   **Serialization:** JSON format optimized for diffing.
*   **Validation:** The editor provides tooling to detect "Islands" (unreachable nodes) and "Dead Ends" (nodes with no valid exit).

## 5. Visual Editor
The editor implementation (`src/editor/ui/views.rs`) provides:
*   **Node Canvas:** Infinite panning/zooming workspace.
*   **Port-Based Linking:** Visual connections between Output Ports and Input Ports.
*   **State Visualization:** Debugging overlays show active execution paths during playtesting.
