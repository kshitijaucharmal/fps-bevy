use bevy::prelude::*;

/// Key configuration
#[derive(Resource)]
pub struct KeyBinds {
    pub forward: KeyCode,
    pub forward_alt: KeyCode,
    pub backward: KeyCode,
    pub backward_alt: KeyCode,
    pub left: KeyCode,
    pub left_alt: KeyCode,
    pub right: KeyCode,
    pub right_alt: KeyCode,
}

impl Default for KeyBinds {
    fn default() -> Self {
        Self {
            forward: KeyCode::W,
            forward_alt: KeyCode::Up,
            backward: KeyCode::S,
            backward_alt: KeyCode::Down,
            left: KeyCode::A,
            left_alt: KeyCode::Left,
            right: KeyCode::D,
            right_alt: KeyCode::Right,
        }
    }
}
