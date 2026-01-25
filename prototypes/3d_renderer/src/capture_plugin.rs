// Frame capture plugin - saves renders to PNG files

use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;
use std::path::PathBuf;

pub struct CapturePlugin {
    pub output_dir: PathBuf,
    pub capture_interval: f32, // seconds between captures
}

impl Default for CapturePlugin {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./captures"),
            capture_interval: 5.0, // capture every 5 seconds
        }
    }
}

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        std::fs::create_dir_all(&self.output_dir).unwrap();
        
        app.insert_resource(CaptureConfig {
            output_dir: self.output_dir.clone(),
            interval: self.capture_interval,
            last_capture: 0.0,
            frame_count: 0,
        })
        .add_systems(Update, capture_frames);
        
        info!("Capture plugin initialized - saving to {:?}", self.output_dir);
    }
}

#[derive(Resource)]
struct CaptureConfig {
    output_dir: PathBuf,
    interval: f32,
    last_capture: f32,
    frame_count: u32,
}

fn capture_frames(
    time: Res<Time>,
    mut config: ResMut<CaptureConfig>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    main_window: Query<Entity, With<PrimaryWindow>>,
) {
    // Capture frame every N seconds
    if time.elapsed_seconds() - config.last_capture >= config.interval {
        config.last_capture = time.elapsed_seconds();
        config.frame_count += 1;
        
        let filename = format!("frame_{:04}.png", config.frame_count);
        let filepath = config.output_dir.join(&filename);
        
        if let Ok(window) = main_window.get_single() {
            match screenshot_manager.save_screenshot_to_disk(window, &filepath) {
                Ok(()) => {
                    info!("✓ Captured frame: {}", filename);
                    eprintln!("✓ Captured frame: {} to {:?}", filename, filepath);
                }
                Err(e) => {
                    error!("✗ Failed to capture frame: {}", e);
                    eprintln!("✗ Failed to capture frame: {}", e);
                }
            }
        }
    }
}
