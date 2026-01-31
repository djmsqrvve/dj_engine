use crate::scripting::{DJScriptingPlugin, LuaContext, ScriptCommand};
use bevy::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

fn setup_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin::default());
    app.add_plugins(bevy::input::InputPlugin);
    app.add_plugins(crate::story_graph::StoryGraphPlugin);
    app.add_plugins(crate::input::DJInputPlugin);
    app.add_message::<crate::audio::AudioCommand>();
    app.add_message::<crate::scene::ChangeSceneEvent>();
    app.init_resource::<crate::scene::SceneManager>();
    app.add_plugins(DJScriptingPlugin);
    app
}

#[test]
fn test_script_loading_validation() {
    let mut app = setup_test_app();
    
    // 1. Test Non-Existent File
    app.world_mut().trigger(ScriptCommand::Load {
        path: "non_existent_script.lua".to_string(),
    });
    app.update();

    // 2. Test Directory Path
    let temp_dir = tempfile::tempdir().unwrap();
    app.world_mut().trigger(ScriptCommand::Load {
        path: temp_dir.path().to_string_lossy().to_string(),
    });
    app.update();

    // 3. Test Valid File
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "print('Hello from Test')").unwrap();
    
    app.world_mut().trigger(ScriptCommand::Load {
        path: file.path().to_string_lossy().to_string(),
    });
    app.update();
}

#[test]
fn test_lua_api_registration() {
    let ctx = LuaContext::new();
    let lua = ctx.lua.lock().unwrap();
    
    super::ffi::register_core_api(&lua).unwrap();
    
    let globals = lua.globals();
    assert!(globals.get::<mlua::Function>("log").is_ok());
    assert!(globals.get::<mlua::Function>("warn").is_ok());
    assert!(globals.get::<mlua::Function>("error").is_ok());
}

#[test]
fn test_sandbox_io_blocked() {
    let lua = crate::scripting::context::LuaContext::new();
    let lua_guard = lua.lua.lock().unwrap();
    
    let result = lua_guard.load("io.open('test.txt', 'w')").exec();
    assert!(result.is_err(), "IO access should be blocked by sandbox");
    
    let os_result = lua_guard.load("os.execute('ls')").exec();
    assert!(os_result.is_err(), "OS execution should be blocked by sandbox");
}

#[test]
fn test_path_traversal_blocked() {
    let base = std::env::current_dir().unwrap();
    let result = crate::scripting::sanitize_path(&base, "../Cargo.toml");
    assert!(result.is_err(), "Should block directory traversal");
}

#[test]
fn test_infinite_loop_timeout() {
    let mut app = setup_test_app();
    
    let loop_file_path = "test_loop.lua";
    std::fs::write(loop_file_path, "while true do end").unwrap();
    
    app.world_mut().trigger(ScriptCommand::Load { 
        path: loop_file_path.to_string()
    });
    
    app.update();
    
    let _ = std::fs::remove_file(loop_file_path);
}