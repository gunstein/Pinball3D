use avian3d::prelude::*;
use bevy::prelude::*;

use super::Ball;
use super::Floor;
use super::HalfHeight;
use super::common;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_target)
            .add_systems(Update, handle_target_events);
    }
}

#[derive(Component)]
struct Target;

fn spawn_target(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<&HalfHeight, With<Floor>>,
) {
    let mut floor_half_height = 0.0;
    for half_height in query_floors.iter() {
        floor_half_height = half_height.0;
    }

    let target_height = 0.1;
    let target_length = 0.1;
    let target_width = 0.01;
    let target_position = Vec3::new(-0.34, -0.09, 0.03);
    let target_rotation = Quat::from_rotation_z(std::f32::consts::PI / 2.0);

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(target_length, target_width, target_height)))),
        MeshMaterial3d(materials.add(Color::srgb(0.93, 0.51, 0.93))),
        RigidBody::Static,
        Collider::cuboid(target_length / 2.0, target_width / 2.0, target_height / 2.0),
        common::board_transform(Transform {
            translation: Vec3::new(
                target_position.x,
                target_position.y,
                target_position.z + target_height / 2.0 + floor_half_height,
            ),
            rotation: target_rotation,
            ..default()
        }),
        CollisionEventsEnabled,
        Target,
    ));
}

fn handle_target_events(
    query_targets: Query<Entity, With<Target>>,
    mut query_balls: Query<(Entity, Forces, &mut LinearVelocity), With<Ball>>,
    mut contact_events: MessageReader<CollisionStart>,
) {
    for event in contact_events.read() {
        for entity in query_targets.iter() {
            if event.collider1 == entity || event.collider2 == entity {
                let ball_entity = if event.collider1 == entity {
                    event.collider2
                } else {
                    event.collider1
                };
                if let Ok((_, mut forces, mut velocity)) = query_balls.get_mut(ball_entity) {
                    *velocity = LinearVelocity::ZERO;
                    forces.apply_linear_impulse(Vec3::new(1.0, 1.0, 0.0) * 0.000013);
                }
            }
        }
    }
}
