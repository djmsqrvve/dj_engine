use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    Playing,
    Paused,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    None,
    ProtagonistMoving,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert_eq!(state, AppState::Loading);
    }

    #[test]
    fn test_app_state_variants() {
        assert_eq!(AppState::Loading, AppState::Loading);
        assert_eq!(AppState::Playing, AppState::Playing);
        assert_eq!(AppState::Paused, AppState::Paused);
        assert_ne!(AppState::Loading, AppState::Playing);
    }

    #[test]
    fn test_game_state_default() {
        let state = GameState::default();
        assert_eq!(state, GameState::None);
    }

    #[test]
    fn test_game_state_variants() {
        assert_eq!(GameState::None, GameState::None);
        assert_eq!(GameState::ProtagonistMoving, GameState::ProtagonistMoving);
        assert_ne!(GameState::None, GameState::ProtagonistMoving);
    }

    #[test]
    fn test_state_clone_and_copy() {
        let state1 = AppState::Loading;
        let state2 = state1;
        assert_eq!(state1, state2);
    }
}
