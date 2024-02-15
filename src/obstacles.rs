use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::debug_tex::uv_debug_texture;

#[derive(Component, Default)]
pub struct Obstacle {
    size: Vec3,
    pos: Transform,
}

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_obstacles);
    }
}

fn setup_obstacles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    // Define blocks
    let blocks: [Obstacle; 3] = [
        Obstacle {
            size: Vec3::new(0.5, 1., 0.5),
            pos: Transform::from_xyz(0.6, -1., 0.6),
        },
        Obstacle {
            size: Vec3::new(0.5, 1., 0.5),
            pos: Transform::from_xyz(4.6, -1., 0.6),
        },
        Obstacle {
            size: Vec3::new(0.5, 1., 0.5),
            pos: Transform::from_xyz(-4.6, -1., 0.6),
        },
    ];

    // Spawn blocks
    for block in &blocks {
        commands
            .spawn(Collider::cuboid(block.size.x, block.size.y, block.size.z))
            .insert(TransformBundle::from(block.pos))
            .insert(PbrBundle {
                mesh: meshes.add(
                    shape::Box::new(block.size.x * 2., block.size.y * 2., block.size.z * 2.).into(),
                ),
                material: debug_material.clone(),
                transform: block.pos,
                ..default()
            });
    }
}
