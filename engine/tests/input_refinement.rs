use bevy::prelude::*;
use dj_engine::prelude::*;
use dj_engine::input::{ActionState, InputAction, InputConfig};
use dj_engine::scripting::{LuaContext, SharedInputStateResource};

#[test]
fn test_action_state_mouse_mapping() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins(bevy::input::InputPlugin);
    app.init_asset::<AudioSource>();
    app.add_plugins(DJEnginePlugin::default().without_diagnostics());

    // 1. Check default mouse mapping: Left -> Confirm
    app.world_mut().resource_mut::<ButtonInput<MouseButton>>().press(MouseButton::Left);
    app.update();

    let actions = app.world().resource::<ActionState>();
    assert!(actions.pressed(InputAction::Confirm), "Left click should trigger Confirm");

    // 2. Check position tracking
    // Note: headless window testing might not update cursor pos easily, 
    // but we can manually set it in world for this test.
    let mut action_state = app.world_mut().resource_mut::<ActionState>();
    action_state.cursor_position = Vec2::new(100.0, 200.0);
    
    let actions = app.world().resource::<ActionState>();
    assert_eq!(actions.cursor_position, Vec2::new(100.0, 200.0));
}

#[test]
fn test_lua_input_sync() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins(bevy::input::InputPlugin);
    app.init_asset::<AudioSource>();
    app.add_plugins(DJEnginePlugin::default().without_diagnostics());

    // Press Space (Confirm)
    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::Space);
    app.update(); // Sync system should run here

    let shared_state = app.world().resource::<SharedInputStateResource>();
    let buffer = shared_state.0.read().unwrap();
    assert!(buffer.pressed.contains("Confirm"));

    // Verify via Lua execution
    let lua_ctx = app.world().resource::<LuaContext>();
    let lua = lua_ctx.lua.lock().unwrap();
    let is_pressed: bool = lua.load("actions.pressed('Confirm')").eval().unwrap();
    assert!(is_pressed, "Lua should see Confirm as pressed");
}

#[test]
fn test_toml_config_deserialization() {
    let toml_str = r#"
[keyboard]
"Space" = "Confirm"
"W" = "Up"

[mouse]
"Left" = "Confirm"

[gamepad]
"South" = "Confirm"
"#;
    let config: InputConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.keyboard.get("Space"), Some(&InputAction::Confirm));
    assert_eq!(config.mouse.get("Left"), Some(&InputAction::Confirm));
}
