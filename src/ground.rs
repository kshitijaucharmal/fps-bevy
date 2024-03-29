use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct GroundProps {
    size: Vec3,
    pos: Transform,
}

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let settings = GroundProps {
        size: Vec3::new(100.0, 0.1, 100.),
        pos: Transform::from_xyz(0.0, -2.0, 0.0),
    };

    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(
            settings.size.x,
            settings.size.y,
            settings.size.z,
        ))
        .insert(TransformBundle::from(settings.pos))
        .insert(PbrBundle {
            mesh: meshes.add(
                shape::Box::new(
                    settings.size.x * 2.,
                    settings.size.y * 2.,
                    settings.size.z * 2.,
                )
                .into(),
            ),
            material: materials.add(Color::GRAY.into()),
            transform: settings.pos,
            ..default()
        });
}
