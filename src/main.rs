// Imports
use bevy::{prelude::*, render::mesh::shape, window::Cursor};
use bevy_rapier3d::prelude::*;

// File imports
use camera::CameraRenderingPlugin;
use debug_tex::uv_debug_texture;
use environment::EnvironmentPlugin;
use keybinds::{InputState, KeyBinds};
use obstacles::ObstaclePlugin;
use player::PlayerPlugin;

// Mods
mod camera;
mod debug_tex;
mod environment;
mod ground;
mod keybinds;
mod obstacles;
mod player;

fn main() {
    App::new()
        // Default Settings
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        cursor: Cursor {
                            visible: false,
                            grab_mode: bevy::window::CursorGrabMode::Locked,
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
        .insert_resource(KeyBinds::default())
        .insert_resource(InputState::default())
        // Rapier
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EnvironmentPlugin)
        .add_plugins(CameraRenderingPlugin)
        .add_plugins(ObstaclePlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, ground::spawn_ground)
        .add_systems(Startup, setup_ball)
        // Run
        .run();
}

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
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
