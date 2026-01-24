use image::io::Reader as ImageReader;
use image::Rgba;
use std::collections::HashMap;
use std::path::Path;
// I'll implement a simple recursive walker since I shouldn't assume 'walkdir' crate is available unless I check Cargo.toml.
// Checking Cargo.toml first is safer, but I can write a 5-line recursive function easily.

mod music;

fn main() {
    // Assuming running from workspace root
    let assets_dir = Path::new("games/dev/new_horizon/assets/sprites/hamster_parts");
    let music_dir = Path::new("games/dev/new_horizon/assets/music");
    std::fs::create_dir_all(music_dir).expect("Failed to create music dir");

    // Generate Overworld Theme
    let midi_bytes = music::generate_overworld_theme();
    let midi_path = music_dir.join("overworld_theme.mid");
    std::fs::write(&midi_path, midi_bytes).expect("Failed to write MIDI file");
    println!("Generated MIDI: {:?}", midi_path);

    // Scalable Processing Rules: Folder Name -> Target Size (Width, Height)
    let mut processing_rules = HashMap::new();
    processing_rules.insert("body", (64, 64));
    processing_rules.insert("head", (32, 32));
    processing_rules.insert("limbs", (16, 16)); // Assuming we organize limbs here
                                                // Fallback for current flat structure if needed, but better to enforce folders.
                                                // For the current structure in the user's snippet, 'paw_left' was in 'body/'.
                                                // Let's refine the rules to match filenames if folder logic is ambiguous,
                                                // OR better: handle the specific existing layout dynamically.

    // Let's iterate and match patterns.
    if assets_dir.exists() {
        process_directory_recursive(assets_dir, &processing_rules);
    }
}

fn process_directory_recursive(dir: &Path, rules: &HashMap<&str, (u32, u32)>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                process_directory_recursive(&path, rules);
            } else if let Some(extension) = path.extension() {
                if extension == "png" || extension == "jpg" || extension == "jpeg" {
                    // Determine size based on folder or filename
                    let (w, h) = determine_target_size(&path);
                    process_asset(path, w, h);
                }
            }
        }
    }
}

fn determine_target_size(path: &Path) -> (u32, u32) {
    // 1. Check parent folder name
    if let Some(parent) = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
    {
        match parent {
            "head" => return (32, 32),
            "body" => {
                // Special case: Limbs are currently in 'body' folder in the old code?
                // The old code said: assets_dir.join("body/paw_left.png")
                // So 'body' folder contains both 64x64 body and 16x16 limbs.
                // We must distinguish by filename.
                let filename = path.file_stem().and_then(|n| n.to_str()).unwrap_or("");
                if filename.contains("paw") || filename.contains("foot") {
                    return (16, 16);
                }
                return (64, 64); // Main body
            }
            _ => {}
        }
    }

    // Default fallback
    (32, 32)
}

fn process_asset(path: std::path::PathBuf, w: u32, h: u32) {
    if !path.exists() {
        // Should not happen with iteration, but good for safety
        return;
    }

    println!(
        "Processing {:?} -> {}x{}",
        path.file_name().unwrap_or_default(),
        w,
        h
    );

    // Read file bytes
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(_) => return, // Skip if read fails
    };

    // Load image guessing format from bytes
    let img = match ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .expect("Failed to guess format")
        .decode()
    {
        Ok(i) => i,
        Err(e) => {
            println!(
                "   Warn: Failed to decode image {:?}: {}",
                path.file_name(),
                e
            );
            return;
        }
    };

    // Resize
    let resized = img.resize_exact(w, h, image::imageops::FilterType::Nearest);
    let mut rgba = resized.to_rgba8();

    // Chroma key (make black/dark background transparent)
    for pixel in rgba.pixels_mut() {
        let Rgba([r, g, b, _]) = *pixel;
        // Threshold for "black" background
        if r < 30 && g < 30 && b < 30 {
            *pixel = Rgba([0, 0, 0, 0]);
        } else {
            // Ensure full opacity for non-background
            *pixel = Rgba([r, g, b, 255]);
        }
    }

    // Save as proper PNG
    // Note: This overwrites the source file. Ideally we'd output to a 'processed' folder,
    // but for this generator tool, in-place update seems to be the intent.
    let _ = rgba.save(&path);
}
