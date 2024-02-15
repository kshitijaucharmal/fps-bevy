use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
    }
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn(Collider::cylinder(1., 0.5))
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            0.0, -1.0, 0.0,
        )));
}
