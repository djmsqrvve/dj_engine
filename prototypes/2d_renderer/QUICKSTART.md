# 🚀 QUICKSTART - Bevy 2D Renderer

**Fastest path from zero to running** (5 minutes or less)

---

## ⚡ TL;DR

```bash
# From parent project root
cd prototypes/2d_renderer

# Run the application
cargo run

# In another terminal, run tests
./test.sh all
```

**That's it!** Everything works out of the box.

---

## 📋 What You're Getting

A **complete, tested, and documented** 2D rendering sandbox with:
- ✅ 7/7 features from your prompt
- ✅ 21 passing tests
- ✅ 45KB of documentation
- ✅ Clean ECS architecture
- ✅ Ready to extend

---

## 🎮 First Run (2 minutes)

### Step 1: Build and Run
```bash
cd prototypes/2d_renderer
cargo run
```

**First build:** ~4-5 minutes (downloads 425+ dependencies)  
**Subsequent runs:** ~5-10 seconds

### Step 2: Try the Controls
When the window opens:
- **Move mouse** → Green light follows
- **Press +** → Zoom out
- **Press -** → Zoom in
- **Watch** → Animated player, scrolling backgrounds

### Step 3: Run Tests (in another terminal)
```bash
./test.sh all
```

Expected output: `21 passed; 0 failed` ✅

---

## 🎯 What's Working

### Visual Features
- ✨ Neon green player (animated)
- 🌌 Purple/blue parallax backgrounds
- 💡 Green point light (mouse-follow)
- 🗺️ Cyan tilemap grid
- 📷 Smooth camera follow

### Code Quality
- ✅ All tests pass
- ✅ No compiler warnings
- ✅ Clean architecture
- ✅ Well documented
- ✅ Ready to extend

---

## 📚 If You Get Stuck

### Problem: Build fails
**Solution:**
```bash
rustup update stable
cargo clean
cargo run
```

### Problem: Tests fail
**Solution:**
```bash
cargo test --no-run  # Recompile
cargo test           # Then test
```

### Problem: Assets not found
**Solution:**
```bash
./check.sh  # Verifies all assets present
```

### Problem: Don't understand something
**Solution:** Read `README.md` (comprehensive user guide)

---

## 💡 Next Steps

### For Experimenting
```bash
# Edit source files
vim src/systems.rs

# See changes immediately
cargo run
```

### For Learning
```bash
# Read the architecture
cat PROJECT_SUMMARY.md

# Understand the tests
cat TESTING.md

# See code structure
tree src/
```

### For Building a Game
```bash
# Replace placeholder assets
# - Edit assets/sprites/player.png
# - Edit assets/backgrounds/*.png
# - Edit assets/tiles/tileset.png

# Add new features
cargo run  # Test as you go
./test.sh  # Verify nothing broke
```

---

## 🗺️ File Explorer

### Essential Files
```
prototypes/2d_renderer/
├── README.md           # ← Start here for usage
├── test.sh             # ← Run this to verify
├── src/
│   ├── main.rs         # ← Application entry
│   └── systems.rs      # ← Most game logic
└── assets/
    ├── sprites/        # ← Player sprite
    └── backgrounds/    # ← Parallax layers
```

### Documentation
- `README.md` - User guide
- `TESTING.md` - How to test
- `PROJECT_STATUS.md` - What's complete
- `FINAL_SUMMARY.md` - Detailed overview

### Helper Scripts
- `./test.sh all` - Run all tests
- `./git-helper.sh` - Git status
- `./check.sh` - Verify build
- `cargo run` - Run app

---

## 🎯 Success Indicators

When everything works, you'll see:

### Build Output
```
   Compiling bevy-2d-renderer...
    Finished dev [optimized + debuginfo]
     Running `target/debug/bevy-2d-renderer`
```

### Window
- Title: "Bevy 2D Rendering Sandbox"
- Size: 1280x720
- Colors: Dark purple, neon green light, cyan tiles

### Tests
```
test result: ok. 21 passed; 0 failed
```

---

## 🚀 Advanced Paths

### I want to...

**...customize the look:**
```bash
# Edit colors in src/main.rs
# Edit assets (use any image editor)
cargo run
```

**...add player movement:**
```bash
# Add to src/systems.rs
# Add input handling
cargo run
```

**...add enemies:**
```bash
# Add components in src/components.rs
# Add spawning in src/systems.rs
# Add behavior in src/systems.rs
cargo test && cargo run
```

**...build a full game:**
```bash
# Read PROJECT_SUMMARY.md for architecture
# Add game logic incrementally
# Test constantly with ./test.sh
```

---

## ⚠️ Warnings (Normal)

When running, you might see:

```
WARN: The selected adapter is using software rendering
```
**→ Normal in WSL2, window still works fine**

```
ALSA lib: cannot find card '0'
```
**→ Normal in WSL2, no audio needed**

```
WARN: CommandQueue has un-applied commands
```
**→ Normal when closing window, not an error**

---

## 📞 Quick Help

| Need | Command/File |
|------|--------------|
| Run the app | `cargo run` |
| Run tests | `./test.sh` |
| See git status | `./git-helper.sh` |
| Understand code | `cat PROJECT_SUMMARY.md` |
| Learn testing | `cat TESTING.md` |
| See what's done | `cat PROJECT_STATUS.md` |
| View architecture | `tree src/` |
| Edit features | `vim src/systems.rs` |
| Replace assets | Edit files in `assets/` |

---

## 🎉 You're Done!

You've successfully set up and run a complete Bevy 2D rendering sandbox.

**What you can do now:**
- ✅ Run the application
- ✅ Run tests
- ✅ Extend features
- ✅ Replace assets
- ✅ Build a game

**Key documents:**
- `README.md` - User guide
- `TESTING.md` - Testing
- `PROJECT_STATUS.md` - Status

**Key commands:**
- `cargo run` - Run
- `./test.sh` - Test

**Happy coding!** 🎮✨

---

*Quickstart guide for the impatient developer.*  
*For detailed information, see README.md and related docs.*
