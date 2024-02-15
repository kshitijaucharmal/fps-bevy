use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::debug_tex::uv_debug_texture;

pub fn spawn_ground(
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
}
