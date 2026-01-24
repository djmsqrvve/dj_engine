-- roblox_bridge.lua
-- Shims for Roblox globals to allow HelixProject logic to run in dj_engine

local function mock_wait(t)
    -- In DJ Engine, we might need a different way to handle yielded waits
    -- for now, this is a placeholder
    return t or 0, os.clock()
end

_G.game = {
    GetService = function(_, serviceName)
        if serviceName == "ReplicatedStorage" then
            return {
                WaitForChild = function(_, name)
                    return "assets/scripts/" .. name
                end
            }
        elseif serviceName == "RunService" then
            return {
                Heartbeat = {
                    Connect = function(_, fn)
                        -- Placeholder for connecting to engine update loop
                    end
                }
            }
        end
        error("Roblox Service not mocked in bridge: " .. tostring(serviceName))
    end,
    
    GetObjects = function(_, id)
        return {}
    end
}

_G.Instance = {
    new = function(className)
        return {
            Name = className,
            Parent = nil,
            Destroy = function() end
        }
    end
}

_G.wait = mock_wait
_G.tick = os.clock
_G.warn = function(...)
    local args = {...}
    for i, v in ipairs(args) do args[i] = tostring(v) end
    -- Connect to Rust logWarn if possible
    print("[ROBLOX-WARN] " .. table.concat(args, " "))
end

-- Helix often uses task.wait
_G.task = {
    wait = mock_wait,
    defer = function(fn) fn() end,
    delay = function(t, fn) end
}

_G.Enum = setmetatable({}, {
    __index = function(t, k)
        t[k] = setmetatable({}, {
            __index = function(_, val) return val end
        })
        return t[k]
    end
})

print("🚀 Roblox Bridge Initialized - Compatibility Layer Active")
