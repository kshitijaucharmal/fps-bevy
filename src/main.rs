use bevy::{
    prelude::*,
    render::{
        mesh::shape,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    window::Cursor,
};
use bevy_rapier3d::prelude::*;

use bevy_flycam::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        cursor: Cursor {
                            visible: false,
                            ..default()
                        },
                        title: "FPS Game".to_string(),
                        resolution: (640., 480.).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PlayerPlugin)
        // .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_lights)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, kill_ball_if_underground)
        .add_systems(Update, reset_camera)
        .run();
}

fn setup_lights(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}

fn reset_camera(mut camera_q: Query<&mut Transform, With<Camera3d>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::R) {
        let mut cam_transform = camera_q.single_mut();
        *cam_transform = Transform::from_xyz(-3.0, 3.0, 10.).looking_at(Vec3::ZERO, Vec3::Y);
    }
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(10.0, 0.1, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)))
        .insert(PbrBundle {
            mesh: meshes.add(shape::Box::new(20.0, 0.2, 20.0).into()),
            material: debug_material.clone(),
            transform: Transform::from_xyz(0.0, -2.0, 0.0),
            ..default()
        });

    // Box
    commands
        .spawn(Collider::cuboid(0.5, 1., 0.5))
        .insert(TransformBundle::from(Transform::from_xyz(0.6, -1., 0.6)))
        .insert(PbrBundle {
            mesh: meshes.add(shape::Box::new(1., 2., 1.).into()),
            material: debug_material.clone(),
            transform: Transform::from_xyz(0.6, -1., 0.6),
            ..default()
        });

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(1.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
        .insert(PbrBundle {
            mesh: meshes.add(
                shape::UVSphere {
                    radius: 0.5,
                    ..default()
                }
                .into(),
            ),
            material: debug_material.clone(),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        });
}

fn kill_ball_if_underground(
    mut commands: Commands,
    positions: Query<(Entity, &Transform), With<RigidBody>>,
) {
    for (entity, transform) in positions.iter() {
        if transform.translation.y <= -100. {
            commands.entity(entity).despawn();
        }
    }
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
