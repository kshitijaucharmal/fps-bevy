use bevy::input::mouse::MouseMotion;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use std::f32::consts::PI;

use crate::keybinds::InputState;

pub struct CameraRenderingPlugin;

#[derive(Component)]
pub struct FPSCamera {
    pub sensitivity: f32,
}

impl Default for FPSCamera {
    fn default() -> Self {
        Self { sensitivity: 0.2 }
    }
}

impl Plugin for CameraRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotation);
    }
}

pub fn setup_fpscam(commands: &mut Commands, pos: Vec3) -> Entity {
    let cam =
        commands.spawn((
            Camera3dBundle {
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::hex("A5DCFF").unwrap()),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, pos.z)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 180.0 * PI / 180., 0.0)),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: -80.,
                    aspect_ratio: 16. / 9.,
                    near: 0.001,
                    far: 1000.0,
                    ..default()
                }),
                ..default()
            },
            FPSCamera::default(),
        ));

    cam.id()
}

fn rotation(
    mut query: Query<(&mut Transform, &FPSCamera)>,
    motion: Res<Events<MouseMotion>>,
    mut state: ResMut<InputState>,
    time: Res<Time>,
) {
    let (mut cam_transform, camera) = query.single_mut();
    let (mut yaw, mut pitch, _) = cam_transform.rotation.to_euler(EulerRot::YXZ);

    for ev in state.reader_motion.read(&motion) {
        pitch -= camera.sensitivity * ev.delta.y * time.delta_seconds();
        yaw -= camera.sensitivity * ev.delta.x * time.delta_seconds();
    }

    pitch = pitch.clamp(-1.54, 1.54);

    cam_transform.rotation =
        Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
}
