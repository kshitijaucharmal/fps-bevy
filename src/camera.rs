use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_flycam::prelude::*;

pub struct CameraRenderingPlugin;

impl Plugin for CameraRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NoCameraPlayerPlugin)
            .add_systems(Startup, setup_flycamera)
            .add_systems(Update, reset_camera);
    }
}

fn setup_flycamera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::hex("A5DCFF").unwrap()),
                ..default()
            },
            transform: Transform::from_xyz(-3.0, 3.0, 10.).looking_at(Vec3::ZERO, Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                aspect_ratio: 16. / 9.,
                near: 0.001,
                far: 1000.0,
                ..default()
            }),
            ..default()
        },
        FlyCam,
    ));
}

fn reset_camera(mut camera_q: Query<&mut Transform, With<Camera3d>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::R) {
        let mut cam_transform = camera_q.single_mut();
        *cam_transform = Transform::from_xyz(-3.0, 3.0, 10.).looking_at(Vec3::ZERO, Vec3::Y);
    }
}
