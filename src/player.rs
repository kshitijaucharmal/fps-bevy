use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::{
    control::KinematicCharacterController,
    dynamics::{LockedAxes, RigidBody},
    geometry::Collider,
};
use std::f32::consts::PI;

use crate::{
    camera::setup_fpscam,
    keybinds::{InputState, KeyBinds},
};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
    pub sensitivity: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, movement);
    }
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = (1., 0.5);
    let transform = Transform::from_xyz(0.0, 1.0, 0.0);

    let player_id = commands
        .spawn(Collider::cylinder(size.0, size.1))
        .insert(Player {
            speed: 10.,
            jump_force: 1000.,
            sensitivity: 0.5,
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(KinematicCharacterController::default())
        .insert(TransformBundle::from_transform(transform))
        .insert(PbrBundle {
            mesh: meshes.add(
                shape::Cylinder {
                    height: size.0 * 2.,
                    radius: size.1,
                    ..default()
                }
                .into(),
            ),
            material: materials.add(Color::GRAY.into()),
            transform: transform,
            ..default()
        })
        .id();

    // Setup camera and add it as child of player
    let cam_id = setup_fpscam(&mut commands, Vec3::new(0., 0.95, 0.1));
    commands.entity(player_id).push_children(&[cam_id]);
}

fn movement(
    keys: Res<Input<KeyCode>>,
    binds: Res<KeyBinds>,
    mut query: Query<(&mut KinematicCharacterController, &Player), With<Player>>,
    time: Res<Time>,
) {
    let mut movement = Vec3::new(0.0, 0.0, 0.);
    let (mut controller, player) = query.single_mut();

    for key in keys.get_pressed() {
        let key = *key;
        if key == binds.forward || key == binds.forward_alt {
            movement.z = 1.;
        }
        if key == binds.backward || key == binds.backward_alt {
            movement.z = -1.;
        }
        if key == binds.left || key == binds.left_alt {
            movement.x = 1.;
        }
        if key == binds.right || key == binds.right_alt {
            movement.x = -1.;
        }

        controller.translation =
            Some(movement.normalize_or_zero() * player.speed * time.delta_seconds());
    }
}
