-- Test script for Hamster Narrator MVP
log("Hello from Lua! Starting hamster_test.lua...")

-- Check initial state
local initial_corruption = get_corruption()
log("Initial Corruption: " .. tostring(initial_corruption))

-- Modify state
log("Setting corruption to 75.5...")
set_corruption(75.5)

log("Setting expression to 'Angry'...")
local success = set_expression("Angry")
if success then
    log("Expression set successfully.")
else
    log("Failed to set expression!")
end

-- Validate changes (Note: In the buffered architecture, these might not update immediately in the same frame if we were reading back from engine, 
-- but our get_corruption() reads from the shared buffer which we just wrote to? 
-- Actually, in my implementation `get_corruption` reads `current_corruption` which is synced FROM ECS to Buffer at start of frame.
-- So `set_corruption` writes to `pending`, but `get_corruption` reads `current`.
-- So immediate readback won't show the change until next frame sync!
-- This is an important behavior to verify.)

local immediate_readback = get_corruption()
log("Immediate Readback (Expect 0 or previous value, not 75.5): " .. tostring(immediate_readback))

log("Test script finished executing.")
