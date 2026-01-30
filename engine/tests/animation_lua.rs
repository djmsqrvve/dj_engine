use bevy::prelude::*;
use dj_engine::prelude::*;
use dj_engine::animation::components::ExpressionController;

#[test]
fn test_animation_lua_integration() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(bevy::audio::AudioPlugin::default());
    app.add_plugins(bevy::input::InputPlugin); // Needed for DJInputPlugin
    app.init_asset::<TextureAtlasLayout>(); // Needed for Sprite
    
    // We need DJEnginePlugin for scripting setup, or manually setup
    // DJEnginePlugin includes everything.
    // But we need to disable TimePlugin if we wanted manual time, but here we don't care about time.
    // Using DJEnginePlugin might be heavy but safe.
    app.add_plugins(DJEnginePlugin::default().without_diagnostics());

    // Spawn entity
    app.world_mut().spawn((
        Name::new("hero"),
        ExpressionController::new(),
        Sprite::default(),
        Transform::default(),
    ));

    // Execute Lua script
    // We can't easily inject a script file without IO, but we can use the LuaContext resource directly
    // or use a temporary file.
    // Accessing LuaContext is better.
    
    let lua_ctx = app.world().resource::<dj_engine::scripting::LuaContext>();
    let result = if let Ok(lua) = lua_ctx.lua.lock() {
        lua.load(r#"
            animation.set_expression("hero", "angry")
        "#).exec()
    } else {
        panic!("Lua lock failed");
    };
    
    assert!(result.is_ok(), "Lua script execution failed: {:?}", result.err());

    // Run updates to flush command queue and process events
    app.update(); // Flush commands to events
    app.update(); // Process events

    // Verify
    let mut query = app.world_mut().query::<(&Name, &ExpressionController)>();
    let mut found = false;
    for (name, controller) in query.iter(app.world()) {
        if name.as_str() == "hero" {
            assert_eq!(controller.current_expression, "angry");
            found = true;
        }
    }
    assert!(found, "Hero entity not found");
}
