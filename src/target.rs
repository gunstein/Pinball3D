use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::ops::Add;

use super::Ball;
use super::Floor;
use super::HalfHeight;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_target)
            .add_system(handle_target_events);
    }
}

#[derive(Component)]
struct Target;

fn spawn_target(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<(Entity, &HalfHeight), With<Floor>>,
) {
    let mut floor = None;
    let mut floor_half_height = 0.0;
    for (entity, half_height) in query_floors.iter() {
        floor = Some(entity);
        floor_half_height = half_height.0;
    }

    let target_height = 0.1;
    let target_length = 0.1;
    let target_width = 0.01;

    let target_position = Vec3::new(-0.34, -0.09, 0.03);
    let target_rotation = Quat::from_rotation_z(std::f32::consts::PI / 2.0);

    let target_mesh_handle: Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(
        target_length,
        target_width,
        target_height,
    )));

    let material_target = materials.add(Color::VIOLET.into());

    let target = commands
        .spawn(PbrBundle {
            mesh: target_mesh_handle.clone(),
            material: material_target.clone(),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            target_length / 2.0,
            target_width / 2.0,
            target_height / 2.0,
        ))
        .insert(TransformBundle::from(Transform {
            translation: Vec3::new(
                target_position.x,
                target_position.y,
                target_position.z + target_height / 2.0 + floor_half_height,
            ),
            rotation: target_rotation,
            ..default()
        }))
        .insert(Target)
        .id();

    commands.entity(floor.unwrap()).add_child(target);
}

fn handle_target_events(
    query_targets: Query<Entity, With<Target>>,
    mut query_balls: Query<(Entity, &mut ExternalImpulse, &mut Velocity), With<Ball>>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for contact_event in contact_events.iter() {
        for entity in query_targets.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity || h2 == &entity {
                    //Give ball a push in starramp direction
                    for (entity_ball, mut external_impulse, mut velocity) in query_balls.iter_mut()
                    {
                        if h1 == &entity_ball || h2 == &entity_ball {
                            velocity.linvel = Vec3::new(0.0, 0.0, 0.0);
                            external_impulse.impulse = external_impulse
                                .impulse
                                .add(Vec3::new(1.0, 1.0, 0.0) * 0.000013);
                        }
                    }
                }
            }
        }
    }
}
