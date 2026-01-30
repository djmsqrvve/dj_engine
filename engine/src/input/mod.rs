//! Input abstraction for DJ Engine.
//!
//! Provides action-based input mapping so games don't need to handle raw keycodes.
//! Supports keyboard with gamepad support planned for the future.

use bevy::prelude::*;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// Logical game actions, abstracted from physical input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Default, Serialize, Deserialize)]
pub enum InputAction {
    /// Confirm selection (Space, Enter, gamepad A)
    #[default]
    Confirm,
    /// Cancel / back (Escape, Backspace, gamepad B)
    Cancel,
    /// Menu / pause (Escape, Tab, gamepad Start)
    Menu,
    /// Directional inputs
    Up,
    Down,
    Left,
    Right,
}

impl InputAction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Confirm" => Some(Self::Confirm),
            "Cancel" => Some(Self::Cancel),
            "Menu" => Some(Self::Menu),
            "Up" => Some(Self::Up),
            "Down" => Some(Self::Down),
            "Left" => Some(Self::Left),
            "Right" => Some(Self::Right),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Confirm => "Confirm",
            Self::Cancel => "Cancel",
            Self::Menu => "Menu",
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Left => "Left",
            Self::Right => "Right",
        }
    }
}

/// Resource tracking the current state of all input actions.
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ActionState {
    /// Actions that are currently held down
    pressed: HashSet<InputAction>,
    /// Actions that were just pressed this frame
    just_pressed: HashSet<InputAction>,
    /// Actions that were just released this frame
    just_released: HashSet<InputAction>,
    /// Current cursor position in screen space
    pub cursor_position: Vec2,
    /// Actions specifically mapped to mouse buttons
    mouse_buttons: HashSet<MouseButton>,
}

impl ActionState {
    /// Returns true if the action is currently held down.
    pub fn pressed(&self, action: InputAction) -> bool {
        self.pressed.contains(&action)
    }

    /// Returns true if the action was just pressed this frame.
    pub fn just_pressed(&self, action: InputAction) -> bool {
        self.just_pressed.contains(&action)
    }

    /// Returns true if the action was just released this frame.
    pub fn just_released(&self, action: InputAction) -> bool {
        self.just_released.contains(&action)
    }

    pub fn iter_pressed(&self) -> impl Iterator<Item = &InputAction> {
        self.pressed.iter()
    }

    pub fn iter_just_pressed(&self) -> impl Iterator<Item = &InputAction> {
        self.just_pressed.iter()
    }

    pub fn iter_just_released(&self) -> impl Iterator<Item = &InputAction> {
        self.just_released.iter()
    }
}

use bevy::asset::{AssetLoader, LoadContext};
use std::collections::HashMap;

/// Configuration for input mappings.
#[derive(Resource, Clone, Reflect, Asset, Deserialize)]
#[reflect(Resource)]
pub struct InputConfig {
    /// Keyboard mappings: "KeyName" -> InputAction
    pub keyboard: HashMap<String, InputAction>,
    /// Mouse button mappings: "ButtonName" -> InputAction
    pub mouse: HashMap<String, InputAction>,
    /// Gamepad button mappings: "ButtonName" -> InputAction
    pub gamepad: HashMap<String, InputAction>,
}

#[derive(Default)]
pub struct InputConfigLoader;

impl AssetLoader for InputConfigLoader {
    type Asset = InputConfig;
    type Settings = ();
    type Error = anyhow::Error;

    fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &(),
        _load_context: &mut LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let s = std::str::from_utf8(&bytes)?;
            let config: InputConfig = toml::from_str(s)?;
            Ok(config)
        }
    }

    fn extensions(&self) -> &[&str] {
        &["input.toml"]
    }
}

impl Default for InputConfig {
    fn default() -> Self {
        let mut keyboard = HashMap::new();
        // Confirm
        keyboard.insert("Space".to_string(), InputAction::Confirm);
        keyboard.insert("Enter".to_string(), InputAction::Confirm);
        keyboard.insert("Z".to_string(), InputAction::Confirm);
        // Cancel
        keyboard.insert("Escape".to_string(), InputAction::Cancel);
        keyboard.insert("X".to_string(), InputAction::Cancel);
        // Menu
        keyboard.insert("Tab".to_string(), InputAction::Menu);
        // Directions
        keyboard.insert("Up".to_string(), InputAction::Up);
        keyboard.insert("Down".to_string(), InputAction::Down);
        keyboard.insert("Left".to_string(), InputAction::Left);
        keyboard.insert("Right".to_string(), InputAction::Right);
        keyboard.insert("W".to_string(), InputAction::Up);
        keyboard.insert("S".to_string(), InputAction::Down);
        keyboard.insert("A".to_string(), InputAction::Left);
        keyboard.insert("D".to_string(), InputAction::Right);

        let mut mouse = HashMap::new();
        mouse.insert("Left".to_string(), InputAction::Confirm);
        mouse.insert("Right".to_string(), InputAction::Cancel);

        let mut gamepad = HashMap::new();
        gamepad.insert("South".to_string(), InputAction::Confirm);
        gamepad.insert("East".to_string(), InputAction::Cancel);
        gamepad.insert("Start".to_string(), InputAction::Menu);
        gamepad.insert("DPadUp".to_string(), InputAction::Up);
        gamepad.insert("DPadDown".to_string(), InputAction::Down);
        gamepad.insert("DPadLeft".to_string(), InputAction::Left);
        gamepad.insert("DPadRight".to_string(), InputAction::Right);

        Self { keyboard, mouse, gamepad }
    }
}

impl InputConfig {
    pub fn resolve_keycode(name: &str) -> Option<KeyCode> {
        match name {
            "Space" => Some(KeyCode::Space),
            "Enter" => Some(KeyCode::Enter),
            "Escape" => Some(KeyCode::Escape),
            "Backspace" => Some(KeyCode::Backspace),
            "Tab" => Some(KeyCode::Tab),
            "ShiftLeft" | "Shift" => Some(KeyCode::ShiftLeft),
            "ControlLeft" | "Control" => Some(KeyCode::ControlLeft),
            "AltLeft" | "Alt" => Some(KeyCode::AltLeft),
            "Up" | "ArrowUp" => Some(KeyCode::ArrowUp),
            "Down" | "ArrowDown" => Some(KeyCode::ArrowDown),
            "Left" | "ArrowLeft" => Some(KeyCode::ArrowLeft),
            "Right" | "ArrowRight" => Some(KeyCode::ArrowRight),
            "A" => Some(KeyCode::KeyA),
            "B" => Some(KeyCode::KeyB),
            "C" => Some(KeyCode::KeyC),
            "D" => Some(KeyCode::KeyD),
            "E" => Some(KeyCode::KeyE),
            "F" => Some(KeyCode::KeyF),
            "G" => Some(KeyCode::KeyG),
            "H" => Some(KeyCode::KeyH),
            "I" => Some(KeyCode::KeyI),
            "J" => Some(KeyCode::KeyJ),
            "K" => Some(KeyCode::KeyK),
            "L" => Some(KeyCode::KeyL),
            "M" => Some(KeyCode::KeyM),
            "N" => Some(KeyCode::KeyN),
            "O" => Some(KeyCode::KeyO),
            "P" => Some(KeyCode::KeyP),
            "Q" => Some(KeyCode::KeyQ),
            "R" => Some(KeyCode::KeyR),
            "S" => Some(KeyCode::KeyS),
            "T" => Some(KeyCode::KeyT),
            "U" => Some(KeyCode::KeyU),
            "V" => Some(KeyCode::KeyV),
            "W" => Some(KeyCode::KeyW),
            "X" => Some(KeyCode::KeyX),
            "Y" => Some(KeyCode::KeyY),
            "Z" => Some(KeyCode::KeyZ),
            "Digit0" | "0" => Some(KeyCode::Digit0),
            "Digit1" | "1" => Some(KeyCode::Digit1),
            "Digit2" | "2" => Some(KeyCode::Digit2),
            "Digit3" | "3" => Some(KeyCode::Digit3),
            "Digit4" | "4" => Some(KeyCode::Digit4),
            "Digit5" | "5" => Some(KeyCode::Digit5),
            "Digit6" | "6" => Some(KeyCode::Digit6),
            "Digit7" | "7" => Some(KeyCode::Digit7),
            "Digit8" | "8" => Some(KeyCode::Digit8),
            "Digit9" | "9" => Some(KeyCode::Digit9),
            _ => None,
        }
    }

    pub fn resolve_mousebutton(name: &str) -> Option<MouseButton> {
        match name {
            "Left" => Some(MouseButton::Left),
            "Right" => Some(MouseButton::Right),
            "Middle" => Some(MouseButton::Middle),
            "Back" => Some(MouseButton::Back),
            "Forward" => Some(MouseButton::Forward),
            _ => None,
        }
    }
    pub fn resolve_gamepad_button(name: &str) -> Option<GamepadButton> {
        match name {
            "South" | "A" => Some(GamepadButton::South),
            "East" | "B" => Some(GamepadButton::East),
            "North" | "Y" => Some(GamepadButton::North),
            "West" | "X" => Some(GamepadButton::West),
            "DPadUp" => Some(GamepadButton::DPadUp),
            "DPadDown" => Some(GamepadButton::DPadDown),
            "DPadLeft" => Some(GamepadButton::DPadLeft),
            "DPadRight" => Some(GamepadButton::DPadRight),
            "Start" => Some(GamepadButton::Start),
            "Select" => Some(GamepadButton::Select),
            _ => None,
        }
    }
}

/// Plugin providing input abstraction.
pub struct DJInputPlugin;

impl Plugin for DJInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActionState>()
            .init_resource::<InputConfig>()
            .register_type::<InputConfig>()
            .register_type::<ActionState>()
            .init_asset::<InputConfig>()
            .init_asset_loader::<InputConfigLoader>()
            .add_systems(PreUpdate, update_action_state);

        info!("DJ Input Plugin initialized");
    }
}

/// System that updates the ActionState based on keyboard input.
fn update_action_state(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    config: Res<InputConfig>,
    mut action_state: ResMut<ActionState>,
    gamepads: Option<Res<ButtonInput<GamepadButton>>>,
) {
    // Clear previous frame's states
    action_state.just_pressed.clear();
    action_state.just_released.clear();
    action_state.pressed.clear();

    // Update cursor position
    if let Ok(window) = windows.get_single() {
        if let Some(pos) = window.cursor_position() {
            action_state.cursor_position = pos;
        }
    }

    // Process Keyboard
    for (key_name, action) in &config.keyboard {
        if let Some(keycode) = InputConfig::resolve_keycode(key_name) {
            if keys.pressed(keycode) {
                action_state.pressed.insert(*action);
            }
            if keys.just_pressed(keycode) {
                action_state.just_pressed.insert(*action);
            }
            if keys.just_released(keycode) {
                action_state.just_released.insert(*action);
            }
        }
    }

    // Process Mouse
    for (button_name, action) in &config.mouse {
        if let Some(button) = InputConfig::resolve_mousebutton(button_name) {
            if mouse.pressed(button) {
                action_state.pressed.insert(*action);
            }
            if mouse.just_pressed(button) {
                action_state.just_pressed.insert(*action);
            }
            if mouse.just_released(button) {
                action_state.just_released.insert(*action);
            }
        }
    }

    // Process Gamepad
    if let Some(gamepads) = gamepads {
        for (button_name, action) in &config.gamepad {
            if let Some(button_type) = InputConfig::resolve_gamepad_button(button_name) {
                 for button in gamepads.get_pressed() {
                     if *button == button_type {
                         action_state.pressed.insert(*action);
                     }
                 }
                 for button in gamepads.get_just_pressed() {
                     if *button == button_type {
                         action_state.just_pressed.insert(*action);
                     }
                 }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_mappings() {
        let config = InputConfig::default();
        assert!(!config.keyboard.is_empty());

        // Verify confirm is mapped
        let has_confirm = config
            .keyboard
            .values()
            .any(|action| *action == InputAction::Confirm);
        assert!(has_confirm);
    }
}
