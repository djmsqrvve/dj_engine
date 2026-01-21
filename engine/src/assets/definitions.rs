//! Asset definitions for DJ Engine.
//!
//! Provides data structures for sprite metadata, palettes, and part libraries.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Definition for a single hamster sprite part loaded from JSON.
#[derive(Serialize, Deserialize, Clone, Debug, Asset, TypePath)]
pub struct HamsterPartDefinition {
    /// Identifier used in code (e.g., "body", "head")
    pub part_name: String,
    /// Relative path to PNG sprite file
    pub sprite_file: String,
    /// Full PNG dimensions in pixels
    pub sprite_size: IVec2,
    /// Position offset in composite hamster image
    #[serde(default)]
    pub original_offset: IVec2,
    /// Z-order (0 = back, higher = front)
    #[serde(default)]
    pub layer_index: u32,
    /// Rotation/scale center point
    pub pivot: Vec2,
    /// Optional bounding box of actual drawn content
    #[serde(default)]
    pub trim_rect: Option<URect>,
}

/// Definition for a color palette loaded from JSON.
#[derive(Serialize, Deserialize, Clone, Debug, Asset, TypePath)]
pub struct PaletteDefinition {
    /// Palette name identifier
    pub palette_name: String,
    /// Color entries (index → RGB)
    pub colors: Vec<ColorEntry>,
}

/// A single color entry in a palette.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ColorEntry {
    /// Palette index (0–255)
    pub index: u32,
    /// Red component (0–255)
    pub r: u8,
    /// Green component (0–255)
    pub g: u8,
    /// Blue component (0–255)
    pub b: u8,
}

impl ColorEntry {
    /// Converts to RGBA bytes (alpha always 255).
    pub fn to_rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, 255]
    }

    /// Converts to Bevy Color.
    pub fn to_color(&self) -> Color {
        Color::srgba_u8(self.r, self.g, self.b, 255)
    }
}

/// Manifest listing all hamster parts to load.
#[derive(Serialize, Deserialize, Clone, Debug, Asset, TypePath)]
pub struct HamsterPartsManifest {
    /// List of parts to load
    pub parts: Vec<PartEntry>,
}

/// Entry in the parts manifest.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PartEntry {
    /// Part name identifier
    pub part_name: String,
    /// Subdirectory containing the part
    pub directory: String,
    /// Filename of the metadata JSON
    pub metadata_file: String,
}

/// Resource storing all loaded hamster parts.
#[derive(Resource, Default)]
pub struct HamsterPartLibrary {
    /// Map of part name to (definition, image handle)
    pub parts: HashMap<String, (HamsterPartDefinition, Handle<Image>)>,
}

impl HamsterPartLibrary {
    /// Creates a new empty library.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a part to the library.
    pub fn insert(
        &mut self,
        name: String,
        definition: HamsterPartDefinition,
        image: Handle<Image>,
    ) {
        self.parts.insert(name, (definition, image));
    }

    /// Gets a part by name.
    pub fn get(&self, name: &str) -> Option<&(HamsterPartDefinition, Handle<Image>)> {
        self.parts.get(name)
    }

    /// Returns the number of loaded parts.
    pub fn len(&self) -> usize {
        self.parts.len()
    }

    /// Returns true if no parts are loaded.
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
}
