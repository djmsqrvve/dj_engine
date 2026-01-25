# Debugging Black Screen - Summary of Changes

## 🔍 Problem
Black screen with no visible entities in Bevy 3D Renderer

## 🎯 Root Causes Identified

### 1. **Likely Cause: Entities Not Spawning**
- Scene loading may not be completing
- Camera might have nothing to look at
- No debug visualization to confirm rendering works

### 2. **What We've Added**

#### ✅ Debug Plugin (`src/debug_plugin.rs`)
```rust
// Spawns a visible 5x5 grid of white cubes at various heights
// Spawns a red reference cube at origin (0, 0.5, 0)
// Logs all spawn positions to console
```

**Key Features:**
- Grid of 25 bright white cubes (emissive, easy to see)
- Red cube at origin (impossible to miss)
- Console logging for every spawned entity

#### ✅ Updated Main (`src/main.rs`)
```rust
// Added DebugPlugin to plugin list
// Info-level logging for camera position
// Info-level logging for entity counts
```

**Key Features:**
- Camera position printed to console
- Total mesh entities logged every frame
- Debug plugin integrated at startup

#### ✅ Enhanced Logging
```rust
info!("=== SPAWNING DEBUG VISUALIZATION ===");
info!("Spawning debug cube at ({}, {}, {})", x, y, z);
info!("Total entities with meshes: {}", count);
```

**What This Shows:**
- Whether entities are spawning
- How many entities are in the scene
- Where entities are positioned

## 🚀 Running with Debug Info

Once compiled, run with:
```bash
cargo run --release
```

**You Should See:**
```
INFO bevy_3d_renderer: === SPAWNING DEBUG VISUALIZATION ===
INFO bevy_3d_renderer: Spawning debug cube at (-4, 5, -4)
INFO bevy_3d_renderer: Spawning debug cube at (-4, 5, 2)
... [23 more cubes]
INFO bevy_3d_renderer: Spawned red reference cube at origin
INFO bevy_3d_renderer: === DEBUG VISUALIZATION COMPLETE ===
INFO bevy_3d_renderer: Camera position: Vec3(0.0, 5.0, 10.0)
INFO bevy_3d_renderer: Camera looking at: Vec3(0.0, 0.0, -1.0)
INFO bevy_3d_renderer: Total entities with meshes: 27
```

## 📊 Expected Scene

**Visible Objects:**
1. **Red cube** at origin (0, 0.5, 0) - High contrast against pale background
2. **25 white cubes** in 5x5 grid - Every 2 units, varying heights
3. **Ground plane** - Large pale rose plane at y=0
4. **PBR test objects** - Cubes and spheres with materials

**Camera View:**
- Position: (0, 5, 10) - Above and back from origin
- Looking at: (0, 0, 0) - Center of scene
- Should see: Everything at origin and in front

## 🔧 If Still Black Screen

### Check Console Output For:
- ❌ "=== SPAWNING DEBUG VISUALIZATION ===" (if missing, plugin not loading)
- ❌ Entity spawn messages (if missing, spawn system not running)
- ❌ "Total entities with meshes: 0" (if 0, nothing spawned)
- ✅ "Total entities with meshes: 27+" (should see this)

### Potential Issues:
1. **Plugin not added** - Check DebugPlugin in main.rs
2. **Systems not running** - Check add_systems calls
3. **Assets not loading** - Check AssetServer logs
4. **Camera in wrong place** - Check camera position logs

## 🎯 What Changed

| File | Change | Purpose |
|------|--------|---------|
| `src/debug_plugin.rs` | NEW FILE | Spawns visible debug objects |
| `src/main.rs` | Added DebugPlugin | Enables debug visualization |
| `src/main.rs` | Added logging | Shows entity counts & positions |

## 📈 Expected Result

**Black screen → Visible Scene**

You should now see:
- ✅ Red cube at center (impossible to miss)
- ✅ White grid of cubes around it
- ✅ Pale rose ground plane
- ✅ Test PBR objects
- ✅ Drow Ranger model (once loaded)

**If you see any debug cubes, rendering is working!**

## 📝 Logging Summary

**Startup Messages:**
```
INFO bevy_3d_renderer: === SPAWNING DEBUG VISUALIZATION ===
INFO bevy_3d_renderer: Spawning debug cube at (X, Y, Z)
...
INFO bevy_3d_renderer: === DEBUG VISUALIZATION COMPLETE ===
```

**Runtime Messages:**
```
INFO bevy_3d_renderer: Camera position: Vec3(0.0, 5.0, 10.0)
INFO bevy_3d_renderer: Total entities with meshes: 27
```

**If messages don't appear:** Plugin isn't loading. Check main.rs plugin list.

## ✅ Bottom Line

**Added:**
- ✅ 27 debug cubes (bright, impossible to miss)
- ✅ Detailed console logging
- ✅ Entity count tracking
- ✅ Camera position logging

**Result:** You should now see SOMETHING, even if it's just debug cubes. This confirms rendering is working and gives us visibility into what's happening.

**Status**: Building (compilation in progress)

---

**Next Action**: Run the application and check console for debug messages.
