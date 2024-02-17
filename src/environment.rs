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
        brightness: 0.7,
    });

    // Pointlight
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 20000.0,
            range: 1000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}
