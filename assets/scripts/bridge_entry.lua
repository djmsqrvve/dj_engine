-- bridge_entry.lua
-- Entry point for running Helix MMORPG logic inside dj_engine

print("--- [HELIX BRIDGE STARTING] ---")

-- 1. Load Roblox Bridge Shim
require("assets/scripts/core/roblox_bridge")

-- 2. Load Infrastructure
local DependencyContainer = require("assets/scripts/core/infrastructure/DependencyContainer")
local BootManager = require("assets/scripts/core/infrastructure/BootManager")

-- 3. Setup Core Mock ( HelixCore.lua )
-- We'll create a minimal version here or port it later
local HelixCore = {
    VERSION = "1.0.0-BRIDGE",
}

-- 4. Initialize BootManager
local boot = BootManager.new(DependencyContainer.new())

print("🛠 Boot Phases Starting...")
-- In Helix, boot is often triggered by a specific event or call
-- For this test, we'll manually push it through the phases
pcall(function()
    boot:start()
end)

print("--- [HELIX BRIDGE READY] ---")

-- 5. Story Graph Listeners
function on_dialogue(speaker, text)
    print(string.format("[STORY DIALOGUE] %s: %s", speaker, text))
    -- Auto-advance for "Text Game" simulation after a small delay?
    -- In a real game, this waits for user click.
    print(" (Pressing NEXT via Lua...)")
    story_next()
end

function on_story_action(action_id, params)
    print(string.format("[STORY ACTION] Triggered: %s with params: %s", action_id, params))
end
