# Debug Console System

## Overview

The Bevy 2D Renderer includes a real-time debug console that displays important game state information directly on the screen. This is invaluable for debugging and development.

## Features

The debug console displays:
- **FPS** (Frames Per Second) - Performance monitoring
- **Mouse Position** - World coordinates of the mouse cursor
- **Zoom Level** - Current camera zoom multiplier
- **Player Position** - Position of the player entity
- **Elapsed Time** - Total time since application started

## How to Use

### Running with Debug Console

```bash
cd prototypes/2d_renderer

# Option 1: Run directly
cargo run

# Option 2: Use demo script (shows what's on console)
./demo.sh
```

### Console Location

The debug console appears in the **top-right corner** of the window:

```
┌─────────────────────────────────────────┐
│ FPS: 60.0                               │
│ Mouse: (125.3, -45.7)                   │
│ Zoom: 1.00x                             │
│ Player: (0.0, 0.0)                      │
│ Time: 5.23s                             │
└─────────────────────────────────────────┘
```

## Console Components

### DebugConsole Resource

Stores the debug messages in a queue with a maximum size limit.

**Location:** `src/resources.rs`

```rust
pub struct DebugConsole {
    pub messages: Vec<String>,
    pub max_messages: usize,
}

impl DebugConsole {
    pub fn new(max_messages: usize) -> Self
    pub fn log(&mut self, message: String)
    pub fn clear(&mut self)
    pub fn get_messages(&self) -> &[String]
}
```

### DebugConsoleUI Component

Marker component for the console UI entity.

**Location:** `src/components.rs`

```rust
#[derive(Component)]
pub struct DebugConsoleUI;
```

### Console Systems

#### `setup_debug_console`

Initializes the console UI text element with styling.

**Location:** `src/systems.rs`

**Features:**
- Positioned in top-right corner
- Two text sections (header + content)
- Styled with mint cyberpunk colors
- Maximum width to prevent overflow

#### `update_debug_console`

Updates the console display every frame with latest game state.

**Location:** `src/systems.rs`

**Updates:**
- FPS calculation: `1.0 / time.delta_seconds()`
- Mouse world position from `MousePosition` resource
- Current zoom from `CameraSettings` resource
- Player position from player entity's `Transform`
- Total elapsed time from `Time` resource

## Adding New Debug Information

### Example: Add Enemy Count to Console

1. **Query for enemies in the system:**

```rust
pub fn update_debug_console(
    // ... existing parameters ...
    enemy_query: Query<(), With<Enemy>>,  // Add this
) {
    if let Ok(mut text) = query.get_single_mut() {
        let fps = 1.0 / time.delta_seconds();
        let mouse_pos = mouse.world_position;
        let zoom = camera_settings.current_zoom;
        let player_pos = player_query.single().translation;
        let enemy_count = enemy_query.iter().len();  // Add this
        
        let debug_text = format!(
            "FPS: {:.1}\n\
             Mouse: ({:.1}, {:.1})\n\
             Zoom: {:.2}x\n\
             Player: ({:.1}, {:.1})\n\
             Enemies: {}\n\
             Time: {:.2}s\n",  // Add enemies line
            fps,
            mouse_pos.x, mouse_pos.y,
            zoom,
            player_pos.x, player_pos.y,
            enemy_count,  // Add this
            time.elapsed_seconds()
        );
        
        if let Some(section) = text.sections.get_mut(1) {
            section.value = debug_text;
        }
    }
}
```

2. **Add the system parameter in main.rs:**

```rust
.add_systems(Update, update_debug_console
    .after(update_mouse_position))
```

### Example: Log Custom Debug Messages

You can also log custom messages to the console:

```rust
fn some_system(
    mut console: ResMut<DebugConsole>,
) {
    console.log("Player collected power-up!".to_string());
    
    // Or with variables
    let score = 100;
    console.log(format!("Score increased by {}", score));
}

// In main.rs
.add_systems(Update, some_system)
```

## Styling the Console

### Change Colors

In `src/systems.rs`, find the `setup_debug_console` function:

```rust
// Header color (mint cyberpunk)
color: Color::srgb(0.0, 1.0, 0.5),

// Content color (light gray)
color: Color::srgb(0.8, 0.8, 0.8),
```

### Change Position

In the same function, modify the `Style`:

```rust
// Top-right (current)
right: Val::Px(10.0),
top: Val::Px(10.0),

// Top-left
left: Val::Px(10.0),
top: Val::Px(10.0),

// Bottom-right
right: Val::Px(10.0),
bottom: Val::Px(10.0),
```

### Change Font Size

```rust
// Header font size
font_size: 14.0,

// Content font size
font_size: 12.0,
```

## Disabling the Console

### For Release Builds

You can conditionally compile the console only for debug builds:

```rust
// In main.rs
#[cfg(debug_assertions)]
app.insert_resource(DebugConsole::new(10))
    .add_systems(Startup, setup_debug_console)
    .add_systems(Update, update_debug_console);
```

### Runtime Toggle

Add a toggle keybinding:

1. **Add a resource for visibility:**

```rust
#[derive(Resource)]
pub struct ConsoleVisibility {
    pub visible: bool,
}

impl Default for ConsoleVisibility {
    fn default() -> Self {
        Self { visible: true }
    }
}
```

2. **Add a toggle system:**

```rust
pub fn toggle_console(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut visibility: ResMut<ConsoleVisibility>,
    mut query: Query<&mut Visibility, With<DebugConsoleUI>>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        visibility.visible = !visibility.visible;
        
        for mut console_visibility in query.iter_mut() {
            *console_visibility = if visibility.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
```

3. **Add to main.rs:**

```rust
.init_resource::<ConsoleVisibility>()
.add_systems(Update, toggle_console)
```

## Testing the Console

### Run the Demo

```bash
./demo.sh
```

This script will:
1. Explain what's displayed on the console
2. Show expected controls
3. Describe the visual elements
4. Run the application

### Manual Testing

1. **Check FPS:**
   - Should be around 60 FPS
   - Higher or lower depending on hardware

2. **Test Mouse Tracking:**
   - Move mouse around screen
   - Values should update in real-time
   - Negative values are normal (world coordinates)

3. **Test Zoom:**
   - Press `+` to zoom out
   - Press `-` to zoom in
   - Zoom value should change in console

4. **Test Player Position:**
   - Player starts at (0.0, 0.0)
   - Position updates as camera follows
   - Should be smooth and continuous

5. **Test Time:**
   - Increases continuously
   - Resets when application restarts

## Debugging with the Console

### Common Debugging Patterns

**1. Tracking State Changes:**

```rust
fn track_player_state(
    mut console: ResMut<DebugConsole>,
    player_query: Query<&PlayerState, Changed<PlayerState>>,
) {
    if let Ok(state) = player_query.get_single() {
        console.log(format!("Player state changed to {:?}", state));
    }
}
```

**2. Counting Events:**

```rust
fn count_collisions(
    mut collision_count: Local<u32>,
    mut console: ResMut<DebugConsole>,
    collision_events: EventReader<CollisionEvent>,
) {
    *collision_count += collision_events.len() as u32;
    
    if collision_events.len() > 0 {
        console.log(format!(
            "Collision #{} detected", 
            *collision_count
        ));
    }
}
```

**3. Performance Monitoring:**

```rust
fn monitor_performance(
    mut console: ResMut<DebugConsole>,
    time: Res<Time>,
    mut frame_count: Local<u32>,
) {
    *frame_count += 1;
    
    if *frame_count % 300 == 0 {  // Every 5 seconds at 60 FPS
        let fps = 1.0 / time.delta_seconds();
        console.log(format!("Performance check: {:.1} FPS", fps));
    }
}
```

### Performance Considerations

The debug console is designed to be lightweight:

- **Updates once per frame** - minimal overhead
- **Limited message history** - prevents memory growth
- **Simple formatting** - no complex layout calculations
- **Conditional compilation ready** - can be disabled in release

### Console Performance Impact

- **FPS display:** Negligible (< 1%)
- **Position updates:** Negligible (< 1%)
- **String formatting:** Minimal impact
- **Text rendering:** Minimal impact (Bevy UI is efficient)

**Total estimated impact: < 5% FPS reduction**

## Extending the Console

### Add More Metrics

Additional ideas for console information:

- **Entity count:** Total entities in world
- **Component count:** Components per entity type
- **Render statistics:** Draw calls, vertices
- **Memory usage:** RAM, VRAM
- **Network stats:** Latency, packets (for multiplayer)
- **Save status:** Last save time, autosave
- **Achievement progress:** Current objectives
- **Quest log:** Active quests with progress

### Multiple Console Windows

You can create multiple debug windows:

```rust
// Performance console
.add_systems(Update, update_performance_console)

// Game state console
.add_systems(Update, update_game_state_console)

// Network console
.add_systems(Update, update_network_console)
```

Each with its own position and styling.

## Troubleshooting

### Console Not Appearing

**Problem:** Console doesn't show up

**Solutions:**
1. Check that system is added to main.rs:
   ```rust
   .add_systems(Update, update_debug_console)
   ```

2. Verify resource is initialized:
   ```rust
   .insert_resource(DebugConsole::new(10))
   ```

3. Check for entity spawning:
   ```rust
   .add_systems(Startup, setup_debug_console)
   ```

4. Look for errors in console output

### Text Not Updating

**Problem:** Console shows but values don't change

**Solutions:**
1. Verify resources are added to app
2. Check system ordering (use `.after()`)
3. Ensure queries can find entities
4. Add `Changed` filters if needed

### Performance Issues

**Problem:** Console causes FPS drop

**Solutions:**
1. Reduce update frequency (not every frame)
2. Decrease max message count
3. Simplify formatting
4. Use `#[cfg(debug_assertions)]` to disable in release

## Best Practices

### ✅ Do
- Keep console updates efficient
- Limit message history size
- Use meaningful debug labels
- Update layout infrequently
- Test on target platforms

### ❌ Don't
- Log every frame unnecessarily
- Store large strings in console
- Update text style every frame
- Ignore performance in loops
- Forget to disable in production

## Integration with Other Systems

### Console + Logging

Combine with Bevy's logging:

```rust
use bevy::log;

fn system_with_logging(
    mut console: ResMut<DebugConsole>,
) {
    let message = format!("FPS: {:.1}", fps);
    
    // Console display
    console.log(message.clone());
    
    // File logging
    log::info!("{}", message);
}
```

### Console + Profiling

Use with profiling tools:

```rust
#[cfg(debug_assertions)]
fn profile_function(
    mut console: ResMut<DebugConsole>,
) {
    let start = std::time::Instant::now();
    
    // ... expensive operation ...
    
    let duration = start.elapsed();
    if duration.as_millis() > 16 {  // > 1 frame at 60 FPS
        console.log(format!("⚠️ Slow operation: {:?}", duration));
    }
}
```

## Summary

The debug console system provides:

- ✅ Real-time game state visualization
- ✅ Performance monitoring (FPS)
- ✅ Input tracking (mouse position)
- ✅ Camera state (zoom level)
- ✅ Entity tracking (player position)
- ✅ Game time monitoring
- ✅ Easy to extend with custom data
- ✅ Customizable styling and position
- ✅ Lightweight and efficient
- ✅ Production-ready toggle

This console will significantly speed up your development and debugging process!

---

*See the demo with `./demo.sh`*  
*For more debugging tips, see TESTING.md*
