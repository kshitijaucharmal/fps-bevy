use bevy::prelude::*;

// This plugin will contain all the lighting settings
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_lights);
    }
}

fn setup_lights(mut commands: Commands) {
    // Global AmbientLight
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.0,
    });
}

pub fn setup_player_light(commands: &mut Commands) -> Entity {
    let light_id = commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 2000.0,
                range: 100.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        })
        .id();

    light_id
}
