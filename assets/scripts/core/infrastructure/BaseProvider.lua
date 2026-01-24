--!strict
--[[
🎯 SYSTEM: BaseProvider | VERSION: 1.0.0 | TYPE: Infrastructure/Abstract
📋 PURPOSE: Abstract base class for all providers with standardized patterns and health monitoring
🔗 DEPENDENCIES: Container-based dependency injection, Logger (injected)
⚠️  HARDCODES: None - All dependencies resolved through container
🔄 STATUS: Active - Base class for all 32+ providers in the system
📅 CREATED: 2025-11-04
📝 NOTES: Enforces consistent provider patterns across entire framework with built-in monitoring
🏷️  KEYWORDS: base-provider, abstract-class, provider-pattern, standardization, health-monitoring, error-handling
🔧 PATTERNS: AbstractClass, TemplateMethod, Provider, DependencyContainer, HealthMonitoring
--]]
-- =============================================================================
-- ROBLOX SERVICES
-- =============================================================================
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Core = ReplicatedStorage:WaitForChild("Core")


-- =============================================================================
-- SMART LOGGER SETUP
-- =============================================================================
local SmartLogger = require(Core.logging.SmartLogger)
local logger = SmartLogger.new("Infrastructure_BaseProvider_Shared:runtime")

-- Try to apply DevConfig settings
pcall(function()
    local DevConfig = require(Core.config.DevConfig)
    if DevConfig and DevConfig.Logging then
        logger:setMinLevel(DevConfig.Logging.masterLevel or "INFO")
    end
end)

-- =============================================================================
-- SAFETY UTILITIES
-- =============================================================================
local NilSafety = require(Core.infrastructure.NilSafety)
local safe = require(Core.utilities.SafetyWrapper)

-- Shim for tick() if missing (e.g. in test environment)
local tick = tick or os.clock



--[[
Async Provider Support - Enhanced lifecycle with async/await patterns

Features:
- Async initialization with timeout protection
- Parallel provider loading capabilities  
- Non-blocking error handling and recovery
- Performance monitoring integration
- Backward compatibility with sync providers
--]]

-- ============================================================================
-- 🏗️ TABLE DEFINITION - Must come BEFORE any function definitions
-- ============================================================================

local BaseProvider = {}
BaseProvider.__index = BaseProvider
BaseProvider.VERSION = "1.0.0"
BaseProvider.MODULE_NAME = "BaseProvider"

-- ============================================================================
-- 🔧 DEBUG CONFIGURATION - Production-safe debug logging
-- ============================================================================
-- Set BaseProvider.DEBUG_MODE = true to enable detailed provider lifecycle logging
-- This is production-safe and can be toggled at runtime

BaseProvider.DEBUG_MODE = false  -- Set to true for provider debug output

BaseProvider.STATE = {
    UNINITIALIZED = "uninitialized",
    INITIALIZING = "initializing",
    INITIALIZED = "initialized",
    REGISTERING = "registering",
    REGISTERED = "registered",
    BOOTING = "booting",
    ACTIVE = "active",
    ERROR = "error",
    SHUTDOWN = "shutdown"
}

local PERFORMANCE_CONFIG = {
    ENABLE_MONITORING = true,
    ENABLE_LOGGING = true,
    ENABLE_VALIDATION = true,
    INITIALIZATION_TIMEOUT = 30.0,
    REGISTRATION_TIMEOUT = 10.0,
    METRICS_UPDATE_INTERVAL = 5.0
}

local ERROR_RECOVERY_CONFIG = {
    ENABLE_AUTO_RECOVERY = true,
    MAX_RETRY_ATTEMPTS = 3,
    RETRY_DELAY = 1.0,
    ENABLE_FALLBACK_SERVICES = true,
    ERROR_THRESHOLD = 5,
    RECOVERY_COOLDOWN = 10.0
}

-- ============================================================================
-- 🎯 CORE CONSTRUCTOR - Now BaseProvider table exists before defining this
-- ============================================================================

function BaseProvider.new(container, helixCoreInstance)
    local self = {}
    setmetatable(self, BaseProvider)
    
    -- Initialize metrics tracking
    self.metrics = {
        servicesRegistered = 0,
        initializationTime = 0,
        errorCount = 0,
        warnings = {},
        optimizations = false
    }
    
    -- Store container reference for backward compatibility
    self.container = container
    
    -- Store helixCore instance for direct access (Phase 1 DI modernization)
    self.helixCore = helixCoreInstance
    
    -- Setup logging
    self.providerName = "BaseProvider"
    self.version = "1.0.0"
    
    -- Create instance-specific logger
    self.logger = SmartLogger.new("Provider_" .. (self.MODULE_NAME or "Base") .. "_Shared:runtime")
    
    -- Apply DevConfig settings if available
    pcall(function()
        local DevConfig = require(Core.config.DevConfig)
        if DevConfig and DevConfig.Logging then
            self.logger:setMinLevel(DevConfig.Logging.masterLevel or "INFO")
        else
            self.logger:setMinLevel("INFO")
        end
    end)
    
    -- Initialize state
    self.state = BaseProvider.STATE.UNINITIALIZED
    self.startTime = tick()
    
    return self
end

-- ============================================================================
-- 🔄 CORE LIFECYCLE METHODS - Default implementations
-- ============================================================================
-- These provide safe defaults that providers can override

function BaseProvider:onInitialize(container, config)
    -- Default: No-op, can be overridden by child classes
    return true
end

function BaseProvider:onRegisterServices(container)
    -- Default: No-op, can be overridden by child classes
    return true
end

function BaseProvider:onBoot(container)
    -- Default: No-op, can be overridden by child classes
    return true
end

-- ============================================================================
-- 🔍 DEBUG LOGGING SYSTEM - Auto-detects provider type
-- ============================================================================
-- Usage: self:debugInfo("message") or self:debugError("error", data)
-- Will show: [CombatProvider] [INFO] message
-- Or: [HeroSelectProvider] [ERROR] error error_details

function BaseProvider:debugLog(level, message, data)
    local currentLogger = self.logger or logger
    
    -- Always log to SmartLogger (it handles its own filtering)
    if currentLogger[level] then
        currentLogger[level](currentLogger, message, data)
    else
        currentLogger:log(level:upper(), message, data)
    end
    
    -- Traditional print only if DEBUG_MODE is explicitly enabled
    if BaseProvider.DEBUG_MODE then
        local providerName = self.MODULE_NAME or "UnknownProvider"
        local prefix = string.format("[%s] [%s]", providerName, level:upper())
        local formattedMessage = string.format("%s %s", prefix, tostring(message))
        
        if data then
            print(formattedMessage, data)
        else
            print(formattedMessage)
        end
    end
end

-- Convenience methods
function BaseProvider:debugInfo(message, data) self:debugLog("info", message, data) end
function BaseProvider:debugWarn(message, data) self:debugLog("warn", message, data) end
function BaseProvider:debugError(message, data) self:debugLog("error", message, data) end
function BaseProvider:debugDebug(message, data) self:debugLog("debug", message, data) end

function BaseProvider:initialize(container, config)
    self.state = BaseProvider.STATE.INITIALIZING
    self:debugInfo("onInitialize starting...")
    
    -- Resolve logger if not already set
    if not self.logger and container then
        pcall(function()
            self.logger = container:tryResolve("Logger")
        end)
    end

    local success, result = pcall(function()
        return self:onInitialize(container, config)
    end)
    
    if success then
        self.state = BaseProvider.STATE.INITIALIZED
        self.initTime = tick() - self.startTime
        self:debugInfo("onInitialize completed", {
            success = true,
            time = string.format("%.3fs", self.initTime)
        })
        
        if self.logger then
            self.logger:debug(string.format("[%s] Initialized in %.3fs", self.MODULE_NAME, self.initTime))
        end
    else
        self.state = BaseProvider.STATE.ERROR
        self:debugError("onInitialize failed!", {
            error = tostring(result)
        })
        
        if self.logger then
            self.logger:error(string.format("[%s] Initialization failed: %s", self.MODULE_NAME, tostring(result)))
        end
    end
    
    return success and result or false
end

function BaseProvider:registerServices(container)
    self.state = BaseProvider.STATE.REGISTERING
    self:debugInfo("onRegisterServices starting...")
    
    local success, result = pcall(function()
        return self:onRegisterServices(container)
    end)
    
    
    -- Interpret explicit false as failure, but nil/true as success
    if success and result ~= false then
        self.state = BaseProvider.STATE.REGISTERED
        self.registrationTime = tick() - (self.startTime or tick())
        self:debugInfo("onRegisterServices completed", {
            success = true,
            time = string.format("%.3fs", self.registrationTime)
        })
        
        if self.logger then
            self.logger:debug(string.format("[%s] Services registered in %.3fs", self.MODULE_NAME, self.registrationTime))
        end
        return true
    else
        self.state = BaseProvider.STATE.ERROR
        self:debugError("onRegisterServices failed!", {
            error = tostring(result)
        })
        
        if self.logger then
            self.logger:error(string.format("[%s] Service registration failed: %s", self.MODULE_NAME, tostring(result)))
        end
        return false
    end
end

function BaseProvider:boot(container)
    self.state = BaseProvider.STATE.BOOTING
    self:debugInfo("onBoot starting...")
    
    local success, result = pcall(function()
        return self:onBoot(container)
    end)
    
    if success then
        self.state = BaseProvider.STATE.ACTIVE
        self:debugInfo("onBoot completed", {
            success = true
        })
        
        if self.logger then
            self.logger:debug(string.format("[%s] Boot completed successfully", self.MODULE_NAME))
        end
    else
        self.state = BaseProvider.STATE.ERROR
        self:debugError("onBoot failed!", {
            error = tostring(result)
        })
        
        if self.logger then
            self.logger:error(string.format("[%s] Boot failed: %s", self.MODULE_NAME, tostring(result)))
        end
    end
    
    return success and result or false
end

--[[
    Report granular progress within a provider's boot/loading phase
    @param current number: Current step
    @param total number: Total steps
    @param message string: Description of current operation
]]
function BaseProvider:reportProgress(current, total, message)
    if not self.container then return end
    
    -- Try to resolve EventBus if we haven't yet
    if not self.eventBus then
        pcall(function()
            self.eventBus = self.container:tryResolve("EventBus")
        end)
    end
    
    if self.eventBus then
        self.eventBus:emit("system.boot.sub_progress", {
            provider = self.MODULE_NAME or self:getName(),
            current = current,
            total = total,
            message = message
        })
    end
end

-- ============================================================================
-- 📡 EVENT BUS WRAPPERS - Shortcut methods for event communication
-- ============================================================================

function BaseProvider:on(event: string, callback: (any) -> ())
    if not self.eventBus then
        local success, result = pcall(function()
            return self.container:resolve("EventBus")
        end)
        if success then
            self.eventBus = result
        else
            self:error("Cannot subscribe to event '" .. event .. "': EventBus not available")
            return nil
        end
    end
    return self.eventBus:on(event, callback)
end

function BaseProvider:once(event: string, callback: (any) -> ())
    if not self.eventBus then
        local success, result = pcall(function()
            return self.container:resolve("EventBus")
        end)
        if success then
            self.eventBus = result
        else
            self:error("Cannot subscribe to event '" .. event .. "' (once): EventBus not available")
            return nil
        end
    end
    return self.eventBus:once(event, callback)
end

function BaseProvider:emit(event: string, payload: any?)
    if not self.eventBus then
        local success, result = pcall(function()
            return self.container:resolve("EventBus")
        end)
        if success then
            self.eventBus = result
        else
            -- Don't hard error on emit, just log and skip
            self:warn("Cannot emit event '" .. event .. "': EventBus not available")
            return
        end
    end
    self.eventBus:emit(event, payload)
end

function BaseProvider:registerService(container, name, service, options)
    if not container then
        self:error("Cannot register service '" .. tostring(name) .. "': Container is nil")
        return false
    end
    
    local success, err = pcall(function()
        container:register(name, service, options)
    end)
    
    if success then
        if not self.registeredServices then self.registeredServices = {} end
        table.insert(self.registeredServices, name)
        self:debug("Registered service: " .. tostring(name))
        return true
    else
        self:error("Failed to register service '" .. tostring(name) .. "': " .. tostring(err))
        return false
    end
end

-- ============================================================================
-- 📝 UTILITY METHODS - Common provider functionality
-- ============================================================================

function BaseProvider:getName()
    return self.MODULE_NAME or "UnknownProvider"
end

function BaseProvider:getVersion()
    return self.VERSION or "1.0.0"
end

function BaseProvider:getState()
    return self.state or BaseProvider.STATE.UNINITIALIZED
end

function BaseProvider:isActive()
    return self.state == BaseProvider.STATE.ACTIVE
end

function BaseProvider:isHealthy()
    return self.state == BaseProvider.STATE.ACTIVE or self.state == BaseProvider.STATE.REGISTERED
end

function BaseProvider:hasErrors()
    return self.state == BaseProvider.STATE.ERROR
end

function BaseProvider:getMetrics()
    return self.metrics or {}
end

--[[
    Resolve a service by module path using ServiceModuleRegistry
    @param modulePath: Full module path (e.g., "Core.engine.systems.Hero.HeroCache")
    @return any: Resolved service instance or nil if not found
]]
function BaseProvider:resolveModule(modulePath)
    local ServiceModuleRegistry = require(Core.infrastructure.ServiceModuleRegistry)
    return ServiceModuleRegistry:resolveFromContainer(self.container, modulePath)
end

--[[
    Try to resolve a service by module path (safe version)
    @param modulePath: Full module path
    @return any: Resolved service instance or nil if not found/error
]]
function BaseProvider:tryResolveModule(modulePath)
    local ServiceModuleRegistry = require(Core.infrastructure.ServiceModuleRegistry)
    return ServiceModuleRegistry:tryResolveFromContainer(self.container, modulePath)
end

--[[
    Get module class by module path using ServiceModuleRegistry cache
    @param modulePath: Full module path
    @return table: Module class or nil if error
]]
function BaseProvider:getModuleClass(modulePath)
    local ServiceModuleRegistry = require(Core.infrastructure.ServiceModuleRegistry)
    return ServiceModuleRegistry:getModule(modulePath)
end

function BaseProvider:log(level, message)
    if self.logger and self.logger.log then
        self.logger:log(level, message)
    elseif self.logger and self.logger.info then
        self.logger:info(message)
    end
end

function BaseProvider:info(message)
    self:log("info", message)
end

function BaseProvider:warn(message)
    self:log("warn", message)
end

function BaseProvider:error(message)
    self:log("error", message)
end

function BaseProvider:debug(message)
    self:log("debug", message)
end

-- ============================================================================
-- 🔄 LOGGING API COMPATIBILITY - Wrapper methods for provider patterns
-- ============================================================================
-- Some providers use logInfo/logError/logWarn/logDebug, others use info/error/warn/debug
-- These wrappers ensure compatibility across all provider implementations

function BaseProvider:logInfo(message)
    self:info(message)
end

function BaseProvider:logError(message)
    self:error(message)
end

function BaseProvider:logWarn(message)
    self:warn(message)
end

function BaseProvider:logDebug(message)
    self:debug(message)
end

-- ============================================================================
-- 🆘 ERROR HANDLING - Default implementations
-- ============================================================================

function BaseProvider:getErrorDetails()
    return {
        provider = self:getName(),
        version = self:getVersion(),
        state = self:getState(),
        errors = self.metrics.errors or {},
        warnings = self.metrics.warnings or {},
        uptime = tick() - (self.startTime or tick())
    }
end

function BaseProvider:addWarning(warning)
    table.insert(self.metrics.warnings, {
        message = warning,
        timestamp = tick()
    })
    
    if #self.metrics.warnings > 10 then
        -- Keep only last 10 warnings
        table.remove(self.metrics.warnings, 1)
    end
end

function BaseProvider:addError(errorMessage, errorDetails)
    table.insert(self.errors, {
        message = errorMessage,
        details = errorDetails,
        timestamp = tick(),
        recovered = false
    })
    
    self.metrics.errorCount = self.metrics.errorCount + 1
    
    if #self.errors > 20 then
        -- Keep only last 20 errors
        table.remove(self.errors, 1)
    end
end

-- ============================================================================
-- 🏥 HEALTH MONITORING - Comprehensive health check
-- ============================================================================

--[[
    Get comprehensive health status of BaseProvider
    @return table: Health status with base provider metrics
--]]
function BaseProvider:getHealthStatus()
    local health = {
        provider = self.MODULE_NAME or "Unknown",
        version = self.VERSION or "1.0.0",
        status = "healthy",
        timestamp = tick(),
        uptime = self.startTime and (tick() - self.startTime) or 0,
        state = self.state or BaseProvider.STATE.UNINITIALIZED,
        metrics = {
            memoryUsage = collectgarbage("count"),
            servicesRegistered = #self.registeredServices or 0,
            lastHealthCheck = tick()
        },
        dependencies = {
            logger = self.logger ~= nil,
            eventBus = self.eventBus ~= nil,
            container = self.container ~= nil,
            allOperational = self.logger and self.eventBus and self.container
        },
        performance = {
            initTime = self.initTime or 0,
            registrationTime = self.registrationTime or 0
        }
    }

    -- Determine overall health status
    if not health.dependencies.allOperational then
        health.status = "critical"
    elseif self.state == BaseProvider.STATE.ERROR then
        health.status = "error"
    elseif self.state == BaseProvider.STATE.INITIALIZING or self.state == BaseProvider.STATE.REGISTERING then
        health.status = "starting"
    end

    return health
end

--[[
    Set health status explicitly (for providers that need to report health)
    @param status string: "healthy", "warning", "error", "critical"
    @param message string: Description of current state
--]]
function BaseProvider:setHealthStatus(status, message)
    self._healthStatus = status or "healthy"
    self._healthMessage = message or ""
    
    if self.logger then
        local level = (status == "healthy" or status == "starting") and "debug" or "warn"
        self.logger[level](self.logger, string.format("[%s] Health: %s - %s", self.MODULE_NAME, status, message or ""))
    end
end

-- ============================================================================
-- ⏰ LIFECYCLE UTILITIES - Helper functions
-- ============================================================================

function BaseProvider:executeWithTimeout(timeout, func, errorMessage)
    local startTime = tick()
    local completed = false
    local result = nil
    local errorResult = nil
    
    -- Create protected execution
    local function protectedExecute()
        local success, res = pcall(func)
        if success then
            result = res
        else
            errorResult = res
        end
        completed = true
    end
    
    -- Start execution in coroutine
    local co = coroutine.create(protectedExecute)
    coroutine.resume(co)
    
    -- Wait with timeout
    while not completed and (tick() - startTime) < timeout do
        task.wait(0.01)
    end
    
    if not completed then
        self:error((errorMessage or "Operation timed out") .. " after " .. timeout .. "s")
        return false, "timeout"
    elseif errorResult then
        self:error((errorMessage or "Operation failed") .. ": " .. tostring(errorResult))
        return false, errorResult
    else
        return true, result
    end
end

function BaseProvider:trackPerformance(label, func)
    local startTime = tick()
    local success, result = pcall(func)
    local endTime = tick()
    
    local duration = endTime - startTime
    
    if self.metrics.performance == nil then
        self.metrics.performance = {}
    end
    
    self.metrics.performance[label] = {
        duration = duration,
        success = success,
        timestamp = endTime
    }
    
    if self.logger and self.logger.debug then
        self.logger:debug(string.format("[%s] %s completed in %.3fs", self.MODULE_NAME, label, duration))
    end
    
    return success and result or false
end

return BaseProvider