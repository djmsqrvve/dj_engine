# Product Vision: Parallel Feature Engine

> "Test Multiple Changes. Real Fast."

## 1. The Core Innovation
DJ Engine is not just a game editor; it is a **Parallel Development Environment**.
Traditional engines force you to work on one version of the game at a time. DJ Engine allows you to visualize, run, and compare **Multiple Feature Branches** simultaneously.

*   **The Main Component is the Branch Graph.**
*   **The Timeline is just the Gater.** It controls *when* things happen, but the *Branch Graph* controls *what* happens.

## 2. The Dashboard (Visual Branching)
The center of the editor is the **Branching Dashboard** (inspired by visual version control, but gamified).
*   **CORE (Heart):** The stable, "Live" version of the game.
*   **Spurs (Branches):** Nodes radiating from the Core representing active experiments (e.g., "Branch 1: New Physics", "Branch 2: Loot Overhaul").
*   **Status Indicators:** Each node shows real-time test status ("Tests Passed", "Tests Failed").

## 3. Parallel Testing Workflow
1.  **Branch:** Drag off the Core to create a new "Test Plane".
2.  **Mutate:** Make changing to Logic/Grid/Story in that plane.
3.  **Verify:** The engine runs Headless Tests in the background for *all active branches*.
4.  **Merge:** Use the "Main Bridge" to connect a healthy branch back to Core.

## 4. Key Terminology
*   **Test Plane:** An isolated sandbox for a feature branch.
*   **Node Bus:** The data connection transporting logic between branches.
*   **Main Bridge:** The interface for resolving conflicts and finalizing a merge.
*   **BOM (Bill of Materials):** The data stream manifest for a specific branch branch.
