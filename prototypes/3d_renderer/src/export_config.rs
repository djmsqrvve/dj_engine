// Export configuration - headless rendering with frame capture

use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;
use std::path::PathBuf;

/// Configuration for exporting renders instead of displaying
#[derive(Resource)]
pub struct ExportConfig {
    pub enabled: bool,
    pub output_dir: PathBuf,
    pub capture_on_startup: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            output_dir: PathBuf::from("./renders"),
            capture_on_startup: true,
        }
    }
}

pub fn setup_capture_on_startup(
    mut commands: Commands,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    export_config: Res<ExportConfig>,
) {
    if !export_config.enabled || !export_config.capture_on_startup {
        return;
    }
    
    std::fs::create_dir_all(&export_config.output_dir).unwrap();
    
    info!("Export config: capturing to {:?}", export_config.output_dir);
    
    if let Ok(window) = main_window.get_single() {
        let filepath = export_config.output_dir.join("startup.png");
        match screenshot_manager.save_screenshot_to_disk(window, &filepath) {
            Ok(()) => {
                info!("✓ Startup capture saved: startup.png");
                eprintln!("═══════════════════════════════════════════════════════════════");
                eprintln!("✓ RENDER EXPORTED: startup.png");
                eprintln!("  Location: {:?}", filepath);
                eprintln!("═══════════════════════════════════════════════════════════════");
            }
            Err(e) => {
                error!("✗ Failed startup capture: {}", e);
            }
        }
    }
}

pub fn capture_on_keypress(
    keys: Res<ButtonInput<KeyCode>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    export_config: Res<ExportConfig>,
    mut capture_count: Local<u32>,
) {
    if !export_config.enabled {
        return;
    }
    
    if keys.just_pressed(KeyCode::Space) {
        *capture_count += 1;
        
        if let Ok(window) = main_window.get_single() {
            let filename = format!("capture_{:03}.png", capture_count);
            let filepath = export_config.output_dir.join(&filename);
            
            match screenshot_manager.save_screenshot_to_disk(window, &filepath) {
                Ok(()) => {
                    info!("✓ Manual capture: {}", filename);
                    eprintln!("✓ Captured {}", filename);
                }
                Err(e) => {
                    error!("✗ Capture failed: {}", e);
                    eprintln!("✗ Capture failed: {}", e);
                }
            }
        }
    }
}
