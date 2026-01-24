--!strict
-- =============================================================================
-- ROBLOX SERVICES
-- =============================================================================
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Core = ReplicatedStorage:WaitForChild("Core")

-- =============================================================================
-- SMART LOGGER SETUP
-- =============================================================================
local SmartLogger = require(Core.logging.SmartLogger)
local logger = SmartLogger.new("Infrastructure_DependencyContainer_Shared:runtime")
logger:setFormat("developer")
logger:setMinLevel("DEBUG")

-- =============================================================================
-- SAFETY UTILITIES
-- =============================================================================
local NilSafety = require(Core.infrastructure.NilSafety)
local safe = require(Core.utilities.SafetyWrapper)




local NilSafety = safe.require(Core.infrastructure.NilSafety)

--[[
🎯 SYSTEM: DependencyContainer | VERSION: 1.1.1 | TYPE: Infrastructure/DependencyContainer
📋 PURPOSE: Simple, focused dependency injection container following AAA principles
🔗 DEPENDENCIES: None - pure dependency injection container
⚠️  HARDCODES: None - explicit registration only
🔄 STATUS: Enhanced - Added service inspection for health monitoring
📅 CREATED: 2025-11-11 (Replacing bloated ConsolidatedDependencyContainer)
📝 NOTES: Only handles registration and resolution - all other concerns handled by separate services
🏷️  KEYWORDS: dependency-injection, service-container, registration, resolution, singleton
🔧 PATTERNS: DependencyContainer, Container, Factory, Singleton, AAA
--]]

local DependencyContainer = {}
DependencyContainer.__index = DependencyContainer
DependencyContainer.VERSION = "1.1.1"
DependencyContainer.MODULE_NAME = "DependencyContainer"

function DependencyContainer.new(initialPhase: string?)
    local self = setmetatable({}, DependencyContainer)
    self.services = {}
    self.singletons = {}
    self.factories = {}
    self.registrationOrder = {}  -- Track registration order for debugging
    
    -- 🔧 PHASE-GATING: Prevent resolve() during registration
    self.phase = initialPhase or "registration"  -- Starts in registration, transitions to "boot" then "ready"
    self._allowEarlyResolve = {
        -- Core infrastructure services safe to resolve anytime
        EventBus = true,
        Logger = true,
        SmartLogger = true,
        Registry = true,
        DevConfig = true,
        ServiceRegistration = true, -- CoreProvider uses this during INIT
        
        -- 🎯 Phase violation fixes: Services commonly needed by factory functions
        NetworkManager = true,         -- Used by network-dependent factories
        ThemeProvider = true,          -- Used by UI component factories
        VFXOrchestrator = true,        -- Used by visual effect factories
        MonitoringService = true,      -- Used by metrics/telemetry factories
        DragDropManager = true,        -- Used by UI/inventory factories
    }
    
    return self
end

--[[
    Lock the container to a specific phase.
    @param phase string: "registration", "boot", or "ready"
]]
function DependencyContainer:lockPhase(phase)
    if phase ~= "registration" and phase ~= "boot" and phase ~= "ready" then
        error("[DependencyContainer] Invalid phase: " .. tostring(phase))
    end
    logger:info(string.format("📌 Container phase locked: %s -> %s", self.phase, phase))
    self.phase = phase
end

function DependencyContainer:register(name, factory, options)

    -- Parameter validation
    safe.assertType(name, "string", "name")
    safe.assertNotNil(factory, "factory", "DependencyContainer:register")
    options = options or {}  -- Apply default before validation
    safe.assertType(options, "table", "options")
    self.services[name] = factory
    if options.singleton then
        self.singletons[name] = nil -- Will be created on first use
    end
    self.factories[name] = options
    table.insert(self.registrationOrder, name)  -- Track order
    return true  -- Return success for ServiceRegistration compatibility
end

function DependencyContainer:resolve(name)
    -- Parameter validation
    safe.assertType(name, "string", "name")
    
    -- 🔧 PHASE-GATING: Block premature resolution during registration
    if self.phase == "registration" and not self._allowEarlyResolve[name] then
        error(string.format(
            "[PHASE VIOLATION] Cannot resolve '%s' during registration phase. " ..
            "Move resolution to onBoot() or add to _allowEarlyResolve whitelist.",
            name
        ))
    end
    
    local factory = NilSafety.assertNotNil(
        self.services[name],
        "Service not found: " .. tostring(name),
        "DependencyContainer:resolve"
    )

    -- Checkpoint log
    if logger then 
        logger:debug(string.format("Testing Resolve Checkpoint: %s (Start)", name))
    end

    -- Handle singleton pattern
    if self.factories[name] and self.factories[name].singleton then
        if not self.singletons[name] then
            if type(factory) == "function" then
                self.singletons[name] = factory()
            else
                self.singletons[name] = factory
            end
        end
        return self.singletons[name]
    end

    -- Handle factory function
    if logger then 
        logger:debug(string.format("Testing Resolve Checkpoint: %s (Factory)", name))
    end
    
    if type(factory) == "function" then
        return factory()
    else
        return factory
    end
end

--[[
    Try to resolve a service, returning nil instead of erroring if not found
    @param name: Service name to resolve
    @return any: Service instance or nil if not found/error
]]
function DependencyContainer:tryResolve(name)
    -- 🔧 PHASE-GATING: Block premature resolution during registration (soft fail for tryResolve)
    if self.phase == "registration" and not self._allowEarlyResolve[name] then
        logger:warn(string.format(
            "[PHASE WARNING] Attempted to resolve '%s' during registration phase. " ..
            "Move resolution to onBoot() or add to _allowEarlyResolve whitelist.",
            name
        ))
        return nil
    end
    
    local factory = self.services[name]
    if not factory then
        return nil
    end

    -- Handle singleton pattern
    if self.factories[name] and self.factories[name].singleton then
        if not self.singletons[name] then
            if type(factory) == "function" then
                local success, result = pcall(factory)
                if success then
                    self.singletons[name] = result
                else
                    return nil
                end
            else
                self.singletons[name] = factory
            end
        end
        return self.singletons[name]
    end

    -- Handle factory function
    if type(factory) == "function" then
        local success, result = pcall(factory)
        return success and result or nil
    else
        return factory
    end
end

-- ============================================================================
-- SECTION: Service Inspection (REQUIRED for HealthMonitor - ADDED 2025-12-09)
-- ============================================================================

--[[
    Get all registered services (including unresolved) for debugging
    @return table: All services keyed by name
]]
function DependencyContainer:getAllRegisteredServices(): { [string]: any }
    local services = {}
    for name, _ in pairs(self.services) do
        services[name] = true
    end
    return services
end

--[[
    Get all RESOLVED services (active instances) for health monitoring
    @return table: All active service instances keyed by name
    @note Only returns successfully resolved services
]]
function DependencyContainer:getAllServices(): { [string]: any }
    local services = {}
    
    -- Include all singletons (these are resolved on first use)
    for name, instance in pairs(self.singletons) do
        if instance ~= nil then
            services[name] = instance
        end
    end
    
    -- Also include any resolved factories that returned instances
    for name, factory in pairs(self.services) do
        if not self.singletons[name] and not services[name] then
            -- Try to resolve if it's safe (no error handling needed, just check)
            local success, instance = pcall(function()
                return self:resolve(name)
            end)
            if success and instance ~= nil then
                services[name] = instance
            end
        end
    end
    
    return services
end

--[[
    Get service statistics for monitoring
    @return table: {total, registered, resolved, singletons, factories}
]]
function DependencyContainer:getServiceStatistics(): { any }
    local stats = {
        totalRegistered = 0,
        singletonsConfigured = 0,
        singletonsResolved = 0,
        factoriesConfigured = 0,
    }
    
    -- Count registered services
    for name, _ in pairs(self.services) do
        stats.totalRegistered = stats.totalRegistered + 1
    end
    
    -- Count singletons
    for name, _ in pairs(self.singletons) do
        stats.singletonsConfigured = stats.singletonsConfigured + 1
        if self.singletons[name] ~= nil then
            stats.singletonsResolved = stats.singletonsResolved + 1
        end
    end
    
    -- Count factories
    for name, options in pairs(self.factories) do
        if not options.singleton then
            stats.factoriesConfigured = stats.factoriesConfigured + 1
        end
    end
    
    return stats
end

--[[
    Check if a service is registered
    @param name string: Service name to check
    @return boolean: Whether service is registered
]]
function DependencyContainer:isRegistered(name: string): boolean
    return self.services[name] ~= nil
end

--[[
    Check if a service is registered (alias for isRegistered for compatibility)
    @param name string: Service name to check
    @return boolean: Whether service is registered
]]
function DependencyContainer:has(name: string): boolean
    return self.services[name] ~= nil
end

--[[
    Check if a singleton service has been resolved
    @param name string: Service name to check
    @return boolean: Whether singleton is resolved
]]
function DependencyContainer:isResolved(name: string): boolean
    return self.singletons[name] ~= nil
end

--[[
    Get list of all registered service names (compatibility with ServiceContainer)
    @return {string}: Array of registered service names
]]
function DependencyContainer:listServices(): {string}
    local list = {}
    for name, _ in pairs(self.services) do
        table.insert(list, name)
    end
    return list
end

-- ============================================================================
-- SECTION: Teardown / Cleanup
-- ============================================================================

--[[
    Teardown the container and all services
    Calls :shutdown() or :destroy() on any resolved singletons
]]
function DependencyContainer:teardown()
    logger:info("Tearing down container services")
    
    -- 1. Teardown all resolved singletons in reverse registration order (if possible) 
    -- or just iterate. Reverse order is safer if we tracked it properly.
    -- We have self.registrationOrder.
    
    for i = #self.registrationOrder, 1, -1 do
        local name = self.registrationOrder[i]
        local instance = self.singletons[name]
        
        if instance then
            -- Try to call shutdown methods
            if type(instance) == "table" or type(instance) == "userdata" then
                local method = instance.shutdown or instance.destroy or instance.Cleanup or instance.cleanup
                if method and type(method) == "function" then
                    pcall(function()
                        method(instance)
                        logger:debug(string.format("Shut down service: %s", name))
                    end)
                end
            end
        end
    end
    
    -- 2. Clear all references
    table.clear(self.services)
    table.clear(self.singletons)
    table.clear(self.factories)
    table.clear(self.registrationOrder)
    
    logger:info("Teardown complete")
end

return DependencyContainer