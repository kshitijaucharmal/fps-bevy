use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::prelude::*;

use crate::debug_tex::uv_debug_texture;

#[derive(Component, Default)]
pub struct Obstacle {
    size: Vec3,
    pos: Transform,
}

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Startup, spawn_3_blocks);
            .add_systems(Startup, spawn_random_blocks);
    }
}

fn spawn_random_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let pos_range = Vec2::new(-100., 100.);
    let size_range = Vec2::new(1., 15.);
    for _ in 0..100 {
        let size = Vec3 {
            x: rng.gen_range(size_range.x..size_range.y),
            y: rng.gen_range(1.0..80.),
            z: rng.gen_range(size_range.x..size_range.y),
        };
        let pos = Vec3 {
            x: rng.gen_range(pos_range.x..pos_range.y),
            y: size.y - 2.0,
            z: rng.gen_range(pos_range.x..pos_range.y),
        };

        let block = Obstacle {
            size: Vec3::new(size.x, size.y, size.z),
            pos: Transform::from_xyz(pos.x, pos.y, pos.z),
        };

        let r = rng.gen_range(0.0..1.0);
        let g = rng.gen_range(0.0..1.0);
        let b = rng.gen_range(0.0..1.0);

        commands
            .spawn(Collider::cuboid(block.size.x, block.size.y, block.size.z))
            .insert(TransformBundle::from(block.pos))
            .insert(PbrBundle {
                mesh: meshes.add(
                    shape::Box::new(block.size.x * 2., block.size.y * 2., block.size.z * 2.).into(),
                ),
                material: materials.add(Color::rgb(r, g, b).into()),
                transform: block.pos,
                ..default()
            });
    }
}

#[allow(dead_code)]
fn spawn_3_blocks(
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
            pos: Transform::from_xyz(4.6, -1., 0.6),
        },
        Obstacle {
            size: Vec3::new(0.5, 1., 0.5),
            pos: Transform::from_xyz(1.6, -1., 0.6),
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
