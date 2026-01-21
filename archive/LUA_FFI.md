# Lua FFI Specification – DJ Engine

**Version**: 1.0  
**Last Updated**: 2026-01-20  
**Language**: Rust ↔ Lua (via mlua)

---

## Overview

This document specifies the boundary between Rust and Lua in DJ Engine. It defines:
- Which Rust functions are exposed to Lua
- Type conversions and marshaling
- Error handling across the boundary
- Hot-reload semantics
- Best practices for Lua script development

---

## 1. FFI Architecture

### Responsibility Division

| Layer | Responsibility |
|-------|-----------------|
| **Rust (Engine)** | Game state management, rendering, performance-critical systems |
| **Lua (Scripts)** | Game logic, dialogue flow, event routing, designer-friendly customization |

**Principle**: Lua scripts should NOT access internal data structures directly. Only call exposed APIs.

### Module Structure

```
engine/src/scripting/
├── mod.rs           # Plugin registration + public API
├── ffi.rs           # Lua-exposed Rust functions (the boundary)
├── hot_reload.rs    # File watching + script reloading
├── error.rs         # Script error types
└── context.rs       # Lua VM management
```

---

## 2. Exposed Rust Functions

### Hamster State API

These functions are **always available** in Lua script context:

#### `set_corruption(value: f32) -> ()`

Updates the hamster's corruption level.

**Signature**:
```lua
set_corruption(float: number) -> nil
```

**Behavior**:
- Clamps input to [0.0, 100.0]
- Triggers corruption effects immediately
- Updates palette, CRT intensity, and animation state

**Example**:
```lua
set_corruption(50.0)    -- Mid-corruption
set_corruption(100.0)   -- Maximum corruption
```

**Rust implementation**:
```rust
pub fn lua_set_corruption(lua: &Lua, value: f32) -> mlua::Result<()> {
    let mut state = STATE.write().unwrap();
    state.set_corruption(value.clamp(0.0, 100.0));
    Ok(())
}
```

---

#### `get_corruption() -> f32`

Reads the current corruption level.

**Signature**:
```lua
get_corruption() -> number
```

**Example**:
```lua
local current = get_corruption()
if current > 80.0 then
    set_expression("corrupted")
end
```

---

#### `set_expression(name: String) -> bool`

Changes the hamster's facial expression.

**Signature**:
```lua
set_expression(string: name) -> boolean
```

**Valid Expressions**:
- `"neutral"`
- `"happy"`
- `"amused"`
- `"angry"`
- `"sad"`
- `"corrupted"`
- `"confused"`

**Returns**:
- `true` if expression was valid and set
- `false` if expression name not recognized

**Example**:
```lua
if set_expression("happy") then
    print("Expression changed to happy")
else
    print("Invalid expression")
end
```

---

#### `get_expression() -> string`

Returns the current expression name.

**Signature**:
```lua
get_expression() -> string
```

**Example**:
```lua
local expr = get_expression()
print("Current expression: " .. expr)
```

---

#### `log(message: String) -> ()`

Writes a message to engine debug output.

**Signature**:
```lua
log(string: message) -> nil
```

**Output destination**:
- Bevy logs (visible with `RUST_LOG=debug`)
- Prefixed with `[Lua]` for easy filtering

**Example**:
```lua
log("Player chose option A")
log("Corruption: " .. get_corruption())
```

---

### Event Callbacks

These functions are **called by Rust**, not called FROM Lua:

#### `init()`

Called once on script load or hot-reload.

**Signature**:
```lua
function init()
    -- Initialize script state here
end
```

**When called**:
- On game startup (if script is auto-loaded)
- On script hot-reload (file change detected)

**Should do**:
- Initialize local state tables
- Set up event listeners (if applicable)
- Log script version/info

**Example**:
```lua
function init()
    print("Hamster dialogue script loaded")
    dialogue_state = {
        current_choice = 0,
        num_choices = 0
    }
    log("Dialogue system ready")
end
```

---

#### `on_key_press(key: String) -> ()`

Called when user presses a key.

**Signature**:
```lua
function on_key_press(key)
    -- Handle key press
end
```

**Key names** (from `winit` / Bevy):
- `"A"`, `"B"`, ... `"Z"` (letter keys)
- `"1"`, `"2"`, ... `"9"`, `"0"` (number keys)
- `"Space"`, `"Enter"`, `"Escape"`, `"Tab"`
- `"ArrowUp"`, `"ArrowDown"`, `"ArrowLeft"`, `"ArrowRight"`

**Example**:
```lua
function on_key_press(key)
    if key == "A" then
        -- Player made choice A (nice)
        set_corruption(math.max(0, get_corruption() - 10))
        set_expression("happy")
    elseif key == "D" then
        -- Player made choice D (mean)
        set_corruption(math.min(100, get_corruption() + 10))
        set_expression("angry")
    end
end
```

---

#### `on_dialogue_event(event_name: String, data: table?) -> ()`

Called when a named dialogue event is triggered from Rust.

**Signature**:
```lua
function on_dialogue_event(event_name, data)
    -- Handle event
end
```

**Example**:
```lua
function on_dialogue_event(name, data)
    if name == "choice_made" then
        local choice_id = data.choice_id  -- Optional data table
        log("Choice made: " .. choice_id)
    end
end
```

---

## 3. Type Conversions

### Lua → Rust

| Lua Type | Rust Type | Notes |
|----------|-----------|-------|
| `number` | `f32` or `f64` | No loss of precision in Lua→Rust direction |
| `string` | `String` | Converted to owned Rust String |
| `boolean` | `bool` | Direct mapping |
| `nil` | `Option::None` | Only in return values |
| `table` | `mlua::Table` | Keep reference, access in Rust |

### Rust → Lua

| Rust Type | Lua Type | Notes |
|-----------|----------|-------|
| `f32` / `f64` | `number` | Automatic conversion |
| `String` | `string` | Automatic conversion |
| `bool` | `boolean` | Automatic conversion |
| `()` | `nil` | Void functions return nil |
| `Result<T, E>` | Lua error or `T` | Errors become Lua exceptions |

---

## 4. Error Handling

### Rust → Lua Errors

When a Rust function returns an error, Lua sees it as an exception:

```rust
// In Rust FFI binding
fn lua_set_expression(lua: &Lua, name: String) -> mlua::Result<bool> {
    let mut state = STATE.write().unwrap();
    match state.set_expression(&name) {
        Ok(success) => Ok(success),
        Err(e) => Err(mlua::Error::external(format!(
            "set_expression failed: {}",
            e
        ))),
    }
}
```

### Lua Error Handling

Scripts should use `pcall` for error recovery:

```lua
function on_key_press(key)
    local success, result = pcall(function()
        return set_expression("happy")
    end)
    
    if not success then
        log("Error setting expression: " .. tostring(result))
    else
        log("Expression set successfully: " .. tostring(result))
    end
end
```

### Silent Error Policy

- **Don't panic in Rust FFI** – return Lua errors gracefully
- **Log all errors** – include context for debugging
- **Let Lua scripts decide recovery** – don't force shutdown

---

## 5. Hot-Reload Semantics

### How Hot-Reload Works

1. **File watch** detects change to `assets/scripts/hamster_dialogue.lua`
2. **Lua VM reloads** the script text
3. **`init()` called again** with fresh state
4. **Rust-side game state preserved** (corruption, expression, etc.)

### Important: State Preservation

Lua script state is **NOT preserved** across reload:

```lua
-- Before reload
my_global_variable = 42

-- After reload (file changes detected)
-- my_global_variable is nil; it's a fresh Lua VM
```

**Solution**: Store persistent state in Rust, read it back:

```lua
function init()
    -- Restore state from Rust
    local current_corruption = get_corruption()
    local current_expr = get_expression()
    log("Reloaded with corruption=" .. current_corruption .. 
        ", expr=" .. current_expr)
end
```

### Debouncing

Hot-reload debounces file changes: waits 500ms after last write before reloading. This prevents reloads from rapid edits.

---

## 6. Script Lifecycle

### Load Order

```
Game Start
    ↓
1. Lua VM initialized (mlua context)
2. FFI functions registered (set_corruption, etc.)
3. Script file loaded from disk
4. init() called
    ↓
Game Running (event loop)
    ↓
on_key_press() called on each input
(possible hot-reload on file change)
    ↓
Game End
```

### Typical Script Structure

```lua
-- Initialization
local state = {}

function init()
    state.current_choice = 0
    state.corruption_history = {}
    log("Script loaded, ready for input")
end

-- Event handlers
function on_key_press(key)
    if key == "A" then
        handle_choice_a()
    elseif key == "D" then
        handle_choice_d()
    elseif key == "Escape" then
        log("Game exited by player")
    end
end

function on_dialogue_event(name, data)
    if name == "scene_start" then
        set_expression("neutral")
    end
end

-- Helper functions
function handle_choice_a()
    set_corruption(math.max(0, get_corruption() - 10))
    set_expression("happy")
    table.insert(state.corruption_history, get_corruption())
end

function handle_choice_d()
    set_corruption(math.min(100, get_corruption() + 10))
    set_expression("angry")
    table.insert(state.corruption_history, get_corruption())
end
```

---

## 7. Debugging Lua Scripts

### Accessing Logs

Run the game with Lua logging enabled:

```bash
RUST_LOG=dj_engine::scripting=debug cargo run -p doomexe
```

### Common Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `attempt to call a nil value` | Function not registered in FFI | Check spelling of `set_corruption` etc. |
| `bad argument #1 to 'set_corruption'` | Wrong type passed (string instead of number) | Ensure `set_corruption(tonumber(x))` |
| `Attempt to yield from C function` | Trying to use `coroutine.yield` | Not supported in FFI calls |
| Script doesn't reload | File not in right path, or still being written | Check `assets/scripts/` path, wait after saving |

### Using `log()` for Debugging

```lua
function on_key_press(key)
    log("Key pressed: " .. key)
    local current = get_corruption()
    log("Current corruption: " .. current)
    
    if set_expression("happy") then
        log("Successfully set expression to happy")
    else
        log("Failed to set expression")
    end
end
```

---

## 8. Performance Considerations

### FFI Overhead

Each FFI call has ~1–2 microseconds overhead. Avoid calling FFI functions 1000s of times per frame:

```lua
-- ❌ Bad: FFI call inside hot loop
for i = 1, 1000 do
    log("Iteration " .. i)  -- 1000 FFI calls!
end

-- ✅ Good: Batch calls
local message = "Started loop"
log(message)
for i = 1, 1000 do
    -- Do Lua-only work
end
log("Loop finished")
```

### Caching Values

```lua
-- ❌ Bad: Calls Rust every iteration
for i = 1, 100 do
    local corruption = get_corruption()  -- 100 FFI calls
    if corruption > 50 then print("High") end
end

-- ✅ Good: Cache once
local corruption = get_corruption()      -- 1 FFI call
for i = 1, 100 do
    if corruption > 50 then print("High") end
end
```

---

## 9. Future Extensions (Beyond Milestone 1)

These are planned but **not in Milestone 1**:

- **`trigger_animation(name)`** – Play hamster animations from Lua
- **`wait_frames(count)`** – Async yield pattern
- **`play_sound(path)`** – Trigger sound effects
- **`get_scene_state()`** – Read game state (inventory, stats, etc.)
- **`emit_dialogue_choice(choices[])`** – Show branching dialogue UI
- **Lua tables as config** – Pass complex data structures

---

## 10. Testing FFI Code

### Unit Test Example

```rust
// In engine/scripting/tests/ffi_tests.rs
#[test]
fn test_set_corruption_clamps_to_bounds() {
    let lua = create_test_lua_context();
    
    lua.globals().set("set_corruption", lua.create_function(|_, value: f32| {
        // Implementation...
        Ok(())
    })?)?;
    
    lua.load(r#"
        set_corruption(-50.0)
        assert(get_corruption() == 0.0, "Should clamp to 0")
        
        set_corruption(150.0)
        assert(get_corruption() == 100.0, "Should clamp to 100")
    "#).eval()?;
    
    Ok(())
}
```

### Integration Test Example

```rust
// In games/dev/doomexe/tests/lua_integration.rs
#[test]
fn test_hamster_dialogue_script_loads() {
    let app = setup_test_app();
    app.update();
    
    // Verify script was loaded and init() was called
    assert!(app.world.get_resource::<ScriptState>().is_some());
}
```

---

## 11. Best Practices

### DO

✅ Use clear function names in scripts (`handle_choice_a` not `f`)  
✅ Log state transitions for debugging  
✅ Handle Lua errors with `pcall`  
✅ Comment complex logic  
✅ Keep scripts < 500 lines (split into multiple files if needed)  

### DON'T

❌ Access Rust internals directly (no `entity.components.transform`)  
❌ Call FFI functions thousands of times per frame  
❌ Store complex mutable state in Lua (use Rust for that)  
❌ Assume Lua state persists across hot-reloads  
❌ Ignore FFI error returns  

---

## 12. Example: Complete Dialogue Script

```lua
-- assets/scripts/hamster_dialogue.lua
-- Main dialogue handler for the hamster narrator

local state = {
    choice_count = 0,
    corruption_history = {},
    last_expression = "neutral",
}

function init()
    state.choice_count = 0
    state.corruption_history = {}
    state.last_expression = "neutral"
    log("Hamster dialogue system initialized")
end

function on_key_press(key)
    log("Key pressed: " .. key)
    
    if key == "A" then
        log("Player chose A (nice option)")
        handle_nice_choice()
    elseif key == "D" then
        log("Player chose D (mean option)")
        handle_mean_choice()
    elseif key == "Escape" then
        log("Player exited")
    else
        log("Unknown key: " .. key)
    end
end

function handle_nice_choice()
    state.choice_count = state.choice_count + 1
    
    -- Lower corruption, show happy expression
    local new_corruption = math.max(0, get_corruption() - 15)
    set_corruption(new_corruption)
    
    local success = set_expression("happy")
    if success then
        log("Hamster is now happy (corruption: " .. new_corruption .. ")")
    else
        log("ERROR: Failed to set expression")
    end
    
    table.insert(state.corruption_history, new_corruption)
end

function handle_mean_choice()
    state.choice_count = state.choice_count + 1
    
    -- Raise corruption, show angry expression
    local new_corruption = math.min(100, get_corruption() + 15)
    set_corruption(new_corruption)
    
    local success = set_expression("angry")
    if success then
        log("Hamster is now angry (corruption: " .. new_corruption .. ")")
    else
        log("ERROR: Failed to set expression")
    end
    
    table.insert(state.corruption_history, new_corruption)
    
    -- At very high corruption, shift to corrupted expression
    if new_corruption >= 90 then
        set_expression("corrupted")
        log("Hamster is highly corrupted!")
    end
end

function on_dialogue_event(name, data)
    if name == "hamster_speaks" then
        log("Hamster has a message for you")
    end
end
```

---

## Quick Reference

| Function | Purpose | Returns |
|----------|---------|---------|
| `set_corruption(f32)` | Set hamster corruption | - |
| `get_corruption()` | Read current corruption | f32 |
| `set_expression(string)` | Change expression | bool |
| `get_expression()` | Read current expression | string |
| `log(string)` | Debug output | - |
| `init()` | Called on load/reload | - |
| `on_key_press(key)` | Called on key input | - |
| `on_dialogue_event(name, data?)` | Called on events | - |

---

## Conclusion

The Lua FFI boundary is carefully designed to:
- **Expose only what scripts need** (no internal APIs)
- **Keep performance predictable** (minimize FFI overhead)
- **Support hot-reload safely** (preserve Rust state)
- **Fail gracefully** (Lua errors don't crash Rust)

Keep scripts simple, debug with `log()`, and test with `pcall()`.
