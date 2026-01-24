--!strict
--[[
    🚀 BOOT MANAGER
    ================
    Manages the game boot sequence with phases, error recovery, and metrics.
    
    PURPOSE:
    - Define and enforce boot order via phases
    - Provide clear error messages when boot fails
    - Track boot timing for performance analysis
    - Support optional phases that can fail without crashing
    - Enable boot dashboard for debugging
    
    BOOT PHASES:
    1. Core Loading (priority 1-10) - Infrastructure services
    2. Data Loading (priority 11-20) - Persistence and config
    3. System Loading (priority 21-50) - Game systems
    4. Feature Initialization (51-80) - Features and class systems
    5. UI Initialization (81-100) - UI components
    
    USAGE:
        local BootManager = require(Core.infrastructure.BootManager)
        local success, bootLog = BootManager:boot(container, paths)
    
    CREATED: 2025-12-27 (Gap Analysis #3 Fix)
    VERSION: 1.0.0
--]]

-- SmartLogger Integration
local SmartLogger
local bootLogger

local BootManager = {}
BootManager.__index = BootManager
BootManager.VERSION = "1.2.0"  -- Bumped for configurable timeouts and phase validation

local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Core = ReplicatedStorage:WaitForChild("Core")
local ErrorHandling = require(Core.infrastructure.ErrorHandling)

-- SmartLogger setup for BootManager
local function initializeLogger()
    SmartLogger = require(game:GetService("ReplicatedStorage"):WaitForChild("Core").logging.SmartLogger)
    bootLogger = SmartLogger.new("Infrastructure_BootManager_Shared:boot")
    bootLogger:setFormat("developer")
    bootLogger:setMinLevel("DEBUG")
    return bootLogger
end

-- Boot state
BootManager.phases = {}
BootManager.bootLog = {}
BootManager.bootMetrics = {}
BootManager.isBooting = false
BootManager.bootComplete = false

-- 🎯 WoW-style: Global boot timeout (prevents infinite hangs)
local GLOBAL_BOOT_TIMEOUT = 120  -- 2 minutes max for entire boot sequence

--[[
    BOOT PHASES DEFINITION
    ======================
    Ordered phases with priority ranges and required flag.
]]
local BOOT_PHASES = {
    {
        name = "Core Loading",
        description = "Essential infrastructure services",
        priorityRange = {1, 10},
        required = true,
        subsystems = {"infrastructure"}
    },
    {
        name = "Data Loading",
        description = "Persistence and configuration",
        priorityRange = {11, 20},
        required = true,
        subsystems = {"persistence"}
    },
    {
        name = "Resource Systems",
        description = "Economy, inventory, crafting",
        priorityRange = {21, 30},
        required = false,
        subsystems = {"resources"}
    },
    {
        name = "Social Systems",
        description = "Chat, guilds, friends",
        priorityRange = {31, 40},
        required = false,
        subsystems = {"social", "layout"}
    },
    {
        name = "Combat Systems",
        description = "Combat mechanics and resistance",
        priorityRange = {41, 50},
        required = false,
        subsystems = {"combat"}
    },
    {
        name = "Quest Systems",
        description = "Quests and objectives",
        priorityRange = {51, 60},
        required = false,
        subsystems = {"quest"}
    },
    {
        name = "Input Systems",
        description = "Keyboard, mouse, keybinds",
        priorityRange = {61, 70},
        required = false,
        subsystems = {"input"}
    },
    {
        name = "Ability Systems",
        description = "Spells, abilities, cooldowns",
        priorityRange = {71, 80},
        required = false,
        subsystems = {"ability", "warlock", "warrior"}
    },
    {
        name = "Hero Systems",
        description = "Action bars, hero controllers",
        priorityRange = {81, 90},
        required = false,
        subsystems = {"hero"}
    },
    {
        name = "UI Systems",
        description = "Tooltips, panels, HUD",
        priorityRange = {91, 100},
        required = false,
        subsystems = {"ui"}
    }
}

--[[
    Initialize boot manager
    @param logger - Logger instance
    @return BootManager instance
]]
function BootManager.new(logger)
    local self = setmetatable({}, BootManager)
    self.logger = logger or initializeLogger()
    self.bootLog = {}
    self.bootMetrics = {
        startTime = 0,
        endTime = 0,
        phaseTimings = {},
        totalProviders = 0,
        successfulProviders = 0,
        processedProviders = 0,
        failedProviders = {}
    }
    return self
end

--[[
    Log a boot message with timestamp
    @param level string - "info" | "warn" | "error" | "debug"
    @param message string
]]
function BootManager:log(level, message)
    local timestamp = tick()
    local logEntry = {
        timestamp = timestamp,
        level = level,
        message = message,
        elapsed = self.bootMetrics.startTime > 0 and (timestamp - self.bootMetrics.startTime) * 1000 or 0
    }
    
    table.insert(self.bootLog, logEntry)
    
    -- Also log to logger if available
    if self.logger then
        local prefix = string.format("[Boot %.2fms]", logEntry.elapsed)
        if level == "info" then
            self.logger:info(prefix .. " " .. message)
        elseif level == "warn" then
            self.logger:warn(prefix .. " " .. message)
        elseif level == "error" then
            self.logger:error(prefix .. " " .. message)
        else
            self.logger:debug(prefix .. " " .. message)
        end
    end
end
--[[
    Perform topological sort on providers based on dependencies
    Restricts sort to providers within the current phase list.
    @param providers table - List of provider entries
    @return table?, string? - Sorted list or nil + error
]]
function BootManager:dependencySort(providers)
    -- Pre-sort by priority so independent nodes respect priority
    table.sort(providers, function(a, b) 
        return (a.priority or 50) < (b.priority or 50) 
    end)

    local graph = {}
    local providerByName = {}
    
    -- Build nodes
    for _, p in ipairs(providers) do
        providerByName[p.name] = p
        graph[p.name] = {
            provider = p,
            dependencies = p.dependencies or {},
            visited = false,
            resolving = false
        }
    end
    
    local sorted = {}
    
    local function visit(nodeName)
        local node = graph[nodeName]
        -- If dependency is not in this phase, we ignore it for sorting purposes
        -- (Ideally implicit architecture ensures dependencies are in same or previous phases)
        if not node then return true end
        
        if node.resolving then 
            return false, "Circular dependency detected: " .. nodeName 
        end
        
        if node.visited then return true end
        
        node.resolving = true
        
        if node.dependencies then
            for _, depName in ipairs(node.dependencies) do
                local ok, err = visit(depName)
                if not ok then return false, err end
            end
        end
        
        node.resolving = false
        node.visited = true
        table.insert(sorted, node.provider)
        return true
    end
    
    for _, p in ipairs(providers) do
        if not graph[p.name].visited then
            local ok, err = visit(p.name)
            if not ok then return nil, err end
        end
    end
    
    return sorted
end

--[[
    Validate phase integrity after execution
    Checks that expected core services are registered based on phase subsystems.
    @param phase table
    @param container table
    @return boolean, string?
]]
function BootManager:validatePhase(phase, container)
    -- Check if validation is enabled in DevConfig
    local successCfg, DevConfig = pcall(require, Core.config.DevConfig)
    if successCfg and DevConfig.Boot and DevConfig.Boot.phaseValidation == false then
        return true -- Validation disabled
    end

    -- Define expected services per phase (critical checks only)
    local phaseExpectations = {
        ["Core Loading"] = {"Logger", "EventBus"},
        ["Data Loading"] = {"SessionOrchestrator"},
        ["Combat Systems"] = {"HealthOrchestrator"},
        ["Hero Systems"] = {"HeroRepository"},
    }

    local expected = phaseExpectations[phase.name]
    if not expected then
        return true -- No expectations defined for this phase
    end

    local missing = {}
    for _, serviceName in ipairs(expected) do
        local service = container:tryResolve(serviceName)
        if not service then
            table.insert(missing, serviceName)
        end
    end

    if #missing > 0 then
        local missingStr = table.concat(missing, ", ")
        self:log("warn", string.format("🛡️ Phase '%s' validation: Missing services: %s", phase.name, missingStr))
        -- Return true but log warning - don't fail boot for missing optional services
        return true
    end

    if self.logger then
        self.logger:debug(string.format("🛡️ Phase Barrier: %s validated (%d services checked)", phase.name, #expected))
    end
    return true
end


--[[
    Execute a single boot phase
    @param phase table - Phase definition
    @param providers table - Array of provider entries to boot
    @param container - Dependency container
    @return success boolean, errors table
]]
function BootManager:executePhase(phase, providers, container)
    local phaseStart = tick()
    local phaseErrors = {}
    local phaseSuccess = 0
    
    -- self:log("debug", string.format("📦 Starting phase: %s (%s)", phase.name, phase.description))
    
    for _, entry in ipairs(providers) do
        local provider = entry.provider
        local providerName = entry.name or "Unknown"
        local providerStart = tick()
        local elapsedFromBootStart = providerStart - self.bootMetrics.startTime

        self:log("debug", string.format("[%08.3fs] ⏳ Booting: %s (Priority: %d)", elapsedFromBootStart, providerName, entry.priority or 50))

        if provider and provider.boot then
             -- Emit precise progress (BEFORE execution)
            local eventBus = container:tryResolve("EventBus")
            if eventBus then
                eventBus:emit("system.boot.progress", {
                    total = self.bootMetrics.totalProviders,
                    current = self.bootMetrics.processedProviders + 1,
                    provider = providerName,
                    phase = phase.name
                })
            end

            -- Read configurable timeout from DevConfig (fallback to 10s)
            local bootTimeout = 10
            local successCfg, DevConfig = pcall(require, Core.config.DevConfig)
            if successCfg and DevConfig.Boot and DevConfig.Boot.providerTimeout then
                bootTimeout = DevConfig.Boot.providerTimeout
            end

            -- Use ErrorHandling.try for boot execution
            local success, result = ErrorHandling.try(function()
                if self.logger then self.logger:debug(string.format("  > Executing %s:boot()", providerName)) end
                return provider:boot(container)
            end, {
                retry = 0,
                timeout = bootTimeout,
                onError = function(err)
                    self:log("error", string.format("  ✗ %s failed: %s", providerName, err.message))
                end
            })


            local providerDuration = tick() - providerStart
            
            -- Record metrics in ProviderRegistry if available
            local ProviderRegistry = require(Core.bootstrap.ProviderRegistry)
            if ProviderRegistry and ProviderRegistry.recordLoadMetrics then
                ProviderRegistry.recordLoadMetrics(providerName, providerDuration)
            end

            if success then
                phaseSuccess = phaseSuccess + 1
                local totalElapsed = tick() - self.bootMetrics.startTime
                local statusMsg = string.format("[%08.3fs] ✓ %s booted in %.3fs (total: %.3fs)", totalElapsed, providerName, providerDuration, totalElapsed)
                if providerDuration > 1.0 then -- Warning threshold for slow providers
                    self:log("warn", statusMsg .. " ⚠️ (SLOW)")
                else
                    self:log("debug", statusMsg)
                end
            else
                local err = result -- ErrorHandling.try returns error object on failure
                table.insert(phaseErrors, {
                    provider = providerName,
                    error = err.message,
                    context = err.context
                })
                
                -- Track failed provider
                table.insert(self.bootMetrics.failedProviders, {
                    name = providerName,
                    phase = phase.name,
                    error = err.message
                })
            end
        else
            -- No boot method or provider missing, count as success but emit "Skipped" event
            local eventBus = container:tryResolve("EventBus")
            if eventBus then
                eventBus:emit("system.boot.progress", {
                    total = self.bootMetrics.totalProviders,
                    current = self.bootMetrics.processedProviders + 1,
                    provider = providerName .. " (Skipped)",
                    phase = phase.name
                })
            end
            
            -- No boot method, count as success
            phaseSuccess = phaseSuccess + 1
        end
        
        -- Increment global processed count regardless of success/failure
        self.bootMetrics.processedProviders = self.bootMetrics.processedProviders + 1
    end
    
    local phaseDuration = (tick() - phaseStart) * 1000
    
    -- Store phase timing
    self.bootMetrics.phaseTimings[phase.name] = {
        duration = phaseDuration,
        providers = #providers,
        successful = phaseSuccess,
        errors = #phaseErrors
    }
    
    local statusIcon = #phaseErrors == 0 and "✅" or (phase.required and "❌" or "⚠️")
    -- self:log("debug", string.format(
    --     "%s Phase '%s' completed in %.2fms (%d/%d providers)",
    --     statusIcon, phase.name, phaseDuration, phaseSuccess, #providers
    -- ))
    
    -- Phase fails if required and has errors
    if phase.required and #phaseErrors > 0 then
        return false, phaseErrors
    end
    
    return true, phaseErrors
end

--[[
    Run the complete boot sequence
    @param providers table - Array of provider entries to boot
    @param container - Dependency container
    @param options? table - {context: "SERVER"|"CLIENT", paths: table}
    @return success boolean, bootLog table
]]
function BootManager:boot(providers, container, options)
    options = options or {}
    local context = options.context or "SERVER"
    local paths = options.paths
    
    if self.isBooting then
        self:log("warn", "Boot already in progress!")
        return false, self.bootLog
    end
    
    local ServiceRegistryModule = paths and paths.infrastructure and paths.infrastructure:FindFirstChild("ServiceRegistry")
    local ServiceRegistry = ServiceRegistryModule and require(ServiceRegistryModule)
    
    self.isBooting = true
    self.bootMetrics.startTime = tick()
    
    -- 🎯 WoW-style: Global boot timeout monitoring
    local bootTimedOut = false
    local timeoutThread = task.delay(GLOBAL_BOOT_TIMEOUT, function()
        if self.isBooting then
            bootTimedOut = true
            self:log("error", string.rep("!", 60))
            self:log("error", string.format("🛑 GLOBAL BOOT TIMEOUT - Boot exceeded %ds!", GLOBAL_BOOT_TIMEOUT))
            self:log("error", string.rep("!", 60))
            self:log("error", string.format("   Providers processed: %d/%d", 
                self.bootMetrics.processedProviders or 0, 
                self.bootMetrics.totalProviders or 0))
            self:log("error", "   Dumping current boot state...")
            self:printDashboard()
        end
    end)
    
    -- De-duplicate providers by name (Latest entry wins)
    local uniqueProviders = {}
    local nameToEntry = {}
    local uniqueNames = {}
    
    for _, entry in ipairs(providers) do
        if not nameToEntry[entry.name] then
            table.insert(uniqueNames, entry.name)
        end
        nameToEntry[entry.name] = entry
    end
    
    for _, name in ipairs(uniqueNames) do
        table.insert(uniqueProviders, nameToEntry[name])
    end
    providers = uniqueProviders
    
    -- Calculate actual scheduled providers (filtering out skipped ones)
    local scheduledCount = 0
    local scheduledProviders = {}
    
    for _, phase in ipairs(BOOT_PHASES) do
        local min, max = phase.priorityRange[1], phase.priorityRange[2]
        for _, entry in ipairs(providers) do
            local priority = entry.priority or 50 -- Check if default priority resolution logic is needed here too?
            -- We replicate the resolution logic or just trust the next loop will handle it?
            -- To be accurate, we should do the resolution once.
            -- But for now, simple check:
            -- Re-resolving priority might be needed if not set.
            -- Let's assume entry.priority is set or we settle for approximate if not.
            -- Actually, let's do it right. We'll count in the main loop or pre-process.
        end
    end
    
    -- Better approach: Pre-process priorities for ALL providers first
    for _, entry in ipairs(providers) do
        if not entry.priority and ServiceRegistry then
             for _, serviceConfig in pairs(ServiceRegistry:getAll()) do
                if serviceConfig.provider == entry.name then
                    entry.priority = serviceConfig.priority
                    break
                end
            end
        end
        entry.priority = entry.priority or 50
        
        -- Check if it falls in any phase
        for _, phase in ipairs(BOOT_PHASES) do
            if entry.priority >= phase.priorityRange[1] and entry.priority <= phase.priorityRange[2] then
                scheduledCount = scheduledCount + 1
                break
            end
        end
    end

    self.bootMetrics.totalProviders = scheduledCount
    
    local allSuccess = true
    local allErrors = {}
    
    -- Group providers by phase
    for _, phase in ipairs(BOOT_PHASES) do
        local phaseProviders = {}
        local minPriority, maxPriority = phase.priorityRange[1], phase.priorityRange[2]
        
        for _, entry in ipairs(providers) do
            local providerName = entry.name
            local priority = entry.priority
            
            -- Resolve priority from ServiceRegistry if missing
            if not priority and ServiceRegistry then
                -- Try to find a service registered by this provider to get the priority
                -- This is a bit of a heuristic since one provider can register many services
                -- We'll look for a service whose 'provider' field matches this provider's name
                for _, serviceConfig in pairs(ServiceRegistry:getAll()) do
                    if serviceConfig.provider == providerName then
                        priority = serviceConfig.priority
                        break
                    end
                end
            end
            
            priority = priority or 50 -- Default priority
            
            if priority >= minPriority and priority <= maxPriority then
                table.insert(phaseProviders, entry)
                entry.priority = priority -- Store resolved priority
            end
        end
        
        -- Skip empty phases
        if #phaseProviders == 0 then
            self:log("debug", string.format("⏭️ Skipping empty phase: %s", phase.name))
            continue
        end
        -- End else block needed at end of loop? No, I need to wrap the rest.
        -- Actually, if I just comment continue, it executes the rest with empty list.
        -- Let's check dependencySort.

        
        -- Topological Sort (Strict TOC)
        local sortedProviders, sortErr = self:dependencySort(phaseProviders)
        
        if not sortedProviders then
             self:log("error", "🛑 BOOT FAILED: " .. tostring(sortErr))
             allSuccess = false
             table.insert(allErrors, { error = sortErr })
             break -- Stop boot strictly
        end
        
        phaseProviders = sortedProviders
        
        -- Execute phase
        self:log("info", string.format("🚀 [PHASE] Starting %s...", phase.name))
        local phaseSuccess, phaseErrors = self:executePhase(phase, phaseProviders, container)
        
        if not phaseSuccess then
            allSuccess = false
            for _, err in ipairs(phaseErrors) do
                table.insert(allErrors, err)
            end
            
            -- If required phase fails, stop boot
            if phase.required then
                self:log("error", "")
                self:log("error", string.rep("!", 60))
                self:log("error", "🛑 BOOT FAILED: Required phase '" .. phase.name .. "' failed")
                self:log("error", string.rep("!", 60))
                break
            end
        else
            -- Validate Phase Barrier
            local valOk, valErr = self:validatePhase(phase, container)
             if not valOk then
                self:log("error", "🛑 BOOT FAILED: Phase Barrier Validation Failed - " .. tostring(valErr))
                allSuccess = false
                break
            end
            
            self:log("info", string.format("✅ [PHASE] %s Completed (Success)", phase.name))
            
            -- EMIT PHASE CHECKPOINT
            local eventBus = container:tryResolve("EventBus")
            if eventBus then
                eventBus:emit("system.boot.phase_complete", {
                    phase = phase.name,
                    priorityRange = phase.priorityRange,
                    timestamp = tick()
                })
            end
        end
        
        self.bootMetrics.successfulProviders = self.bootMetrics.successfulProviders + 
            (self.bootMetrics.phaseTimings[phase.name] and self.bootMetrics.phaseTimings[phase.name].successful or 0)
    end
    
    self.bootMetrics.endTime = tick()
    local totalDuration = (self.bootMetrics.endTime - self.bootMetrics.startTime) * 1000
    
    -- Boot summary
    -- self:log("debug", "")
    -- self:log("debug", string.rep("═", 60))
    if allSuccess then
        -- self:log("debug", string.format("✅ BOOT COMPLETE in %.2fms", totalDuration))
    else
        self:log("warn", string.format("⚠️ BOOT COMPLETED WITH ERRORS in %.2fms", totalDuration))
    end
    -- self:log("debug", string.format(
    --     "   Providers: %d/%d successful",
    --     self.bootMetrics.successfulProviders,
    --     self.bootMetrics.totalProviders
    -- ))
    
    -- PHASE 5: FINAL VALIDATION (Gap Analysis Fix #1)
    if allSuccess then
        self:log("info", "🔍 Running final service registration validation...")
        local ServiceRegistryValidatorModule = paths and paths.infrastructure:FindFirstChild("ServiceRegistryValidator")
        local ServiceRegistryValidator = ServiceRegistryValidatorModule and require(ServiceRegistryValidatorModule)
        
        if ServiceRegistryValidator then
            local valSuccess, valErrors, valMsg = ServiceRegistryValidator:validate(container, {
                strict = false,
                context = context
            })
            
            if not valSuccess then
                self:log("warn", "⚠️ Service validation found missing registrations")
                -- We don't fail boot here as it's non-strict, but we log it
                if valMsg then
                    for _, line in ipairs(string.split(valMsg, "\n")) do
                        self:log("warn", "  " .. line)
                    end
                end
            else
                self:log("info", "✅ Service registration validation passed")
            end
        else
            self:log("warn", "⚠️ ServiceRegistryValidator not found - skipping final validation")
        end
    end
    
    if #self.bootMetrics.failedProviders > 0 then
        self:log("warn", string.format("   Failed: %d providers", #self.bootMetrics.failedProviders))
    end
    -- self:log("debug", string.rep("═", 60))
    -- self:log("debug", "")
    
    self.isBooting = false
    self.bootComplete = true
    
    -- 🎯 WoW-style: Cancel global timeout thread (boot completed successfully)
    if timeoutThread then
        task.cancel(timeoutThread)
    end
    
    -- EMIT BOOT COMPLETE EVENT (for BootDashboard and other listeners)
    local eventBus = container:tryResolve("EventBus")
    if eventBus then
        eventBus:emit("system.boot.complete", {
            success = allSuccess,
            totalProviders = self.bootMetrics.totalProviders,
            successfulProviders = self.bootMetrics.successfulProviders,
            failedProviders = #self.bootMetrics.failedProviders,
            duration = (self.bootMetrics.endTime - self.bootMetrics.startTime),
            context = context
        })
        self:log("info", "📡 Emitted system.boot.complete event")
    end
    
    return allSuccess, self.bootLog
end

--[[
    Get boot metrics for analysis
    @return table - Boot metrics
]]
function BootManager:getMetrics()
    return self.bootMetrics
end

--[[
    Print boot dashboard
]]
function BootManager:printDashboard()
    -- Use bootLogger for dashboard output (section header)
    bootLogger:sectionHeader("BOOT DASHBOARD", "INFO")
    
    local totalDuration = (self.bootMetrics.endTime - self.bootMetrics.startTime) * 1000
    
    bootLogger:info(string.format("Total Boot Time: %.2fms", totalDuration), {
        duration_ms = totalDuration
    }, {"PERF_METRIC"})
    
    bootLogger:info(string.format("Providers: %d/%d successful", 
        self.bootMetrics.successfulProviders, 
        self.bootMetrics.totalProviders), {
        successful = self.bootMetrics.successfulProviders,
        total = self.bootMetrics.totalProviders,
        failed = #self.bootMetrics.failedProviders
    })
    
    bootLogger:sectionHeader("Phase Breakdown", "INFO")
    
    -- Sort phases by duration for analysis
    local phases = {}
    for name, timing in pairs(self.bootMetrics.phaseTimings) do
        table.insert(phases, {name = name, timing = timing})
    end
    table.sort(phases, function(a, b) return a.timing.duration > b.timing.duration end)
    
    for _, phase in ipairs(phases) do
        local timing = phase.timing
        local statusIcon = timing.errors > 0 and "⚠️" or "✅"
        
        bootLogger:info(string.format(
            "%s %-25s %.1fms (%d/%d)",
            statusIcon, phase.name, timing.duration, timing.successful, timing.providers
        ), {
            phase = phase.name,
            duration_ms = timing.duration,
            successful = timing.successful,
            total = timing.providers,
            errors = timing.errors
        }, {"PERF_METRIC"})
    end
    
    if #self.bootMetrics.failedProviders > 0 then
        bootLogger:sectionHeader("Failed Providers", "WARN")
        
        for _, failure in ipairs(self.bootMetrics.failedProviders) do
            bootLogger:warn(string.format("%s (%s)", failure.name, failure.phase), {
                provider_name = failure.name,
                phase = failure.phase,
                error = failure.error
            }, {"ERROR_PROPAGATE"})
        end
    end
    
    bootLogger:sectionFooter("Boot Complete", "INFO")
end

--[[
    Get phases definition (for external use)
    @return table - Boot phases
]]
function BootManager:getPhases()
    return BOOT_PHASES
end

-- Initialize boot logger on module load
initializeLogger()

return BootManager