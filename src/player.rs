use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
}
