# System: Visual Branching & Testing Dashboard

## 1. The Core UI
The primary view of the DJ Engine is the **Parallel Testing Dashboard** (see `docs/images/branching_vision.png`).
It visualizes the codebase not as a file list, but as a live topology of **Test Planes**.

### Components
*   **CORE:** The central hub. The "Production" build.
*   **Branches (Spurs):** Divergent lines representing features.
    *   *Visuals:* Color-coded nodes (Purple = Branch 1, Blue = Branch 2, etc.).
    *   *Commit Nodes:* Each step in a branch is a node showing its Commit ID (e.g., "Commit 322") and Health Status ("Tests Passed" / "Tests Failed").

## 2. Terminology & Architecture

### A. Test Plane
An isolated runtime environment.
*   **Function:** When you create a branch, you spawn a new Test Plane.
*   **Isolation:** Changes in Test Plane A do not affect Test Plane B.
*   **Testing:** The engine runs background headless tests (`cargo test`) against every active Plane continuously.

### B. BOM (Bill of Materials)
*   **Definition:** The manifest of data streams for a specific branch.
*   **UI Panel:** Displays distinct stream IDs (e.g., `Data Stream 1 [12345...]`) verifying that the branch has the correct asset versions.

### C. Main Bridge
*   **Function:** The merge interface.
*   **Action:** Clicking "Main Bridge" initiates the diff/merge protocol, allowing verified features from a Test Plane to be folded into CORE.

### D. Node Bus
*   **Function:** The underlying data transport that connects branches. Allows "cherry-picking" logic nodes from one branch to another without a full merge.

## 3. Workflow
1.  **Draft:** Create a branch (Test Plane).
2.  **Edit:** Modify parameters/grid/story.
3.  **Validate:** Watch the "Tests Passed" indicator light up green.
4.  **Merge:** Hit "Main Bridge" to commit to Core.
