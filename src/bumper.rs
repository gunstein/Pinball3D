use std::ops::Add;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::Floor;
use super::Ball;
use super::HalfHeight;

pub struct BumperPlugin;

impl Plugin for BumperPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_bumpers)
            .add_system(handle_bumper_events)
            .add_system(respawn_bumper_to_toggle_color);
    }
}

#[derive(Component)]
struct Bumper;

#[derive(Component)]
struct TimestampLastHit(f64);

#[derive(Component)]
struct Position(Vec3);

#[derive(Component)]
struct Rotation(Quat);


struct InitBumper{
    position: Position,
    rotation: Rotation,
}

fn spawn_bumpers(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<(Entity, &HalfHeight), With<Floor>>
)
{
    let init_bumpers : [InitBumper;1] = [
        InitBumper{
            position: Position(Vec3::new(-0.2, -0.65, 0.0)),
            rotation: Rotation(Quat::from_rotation_z(1.1))
        }
    ];

    for i in 0..init_bumpers.len() {
        let init_bumper = &init_bumpers[i];

        spawn_single_bumper(&mut commands, &init_bumper.position, &init_bumper.rotation, None, &mut meshes, &mut materials, &query_floors);
    }
}


fn spawn_single_bumper(    
    commands: &mut Commands,
    position: & Position,
    rotation: & Rotation,
    timestamp_last_hit: Option<f64>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    query_floors: & Query<(Entity, &HalfHeight), With<Floor>>,
)
{
    let mut floor = None;
    let mut floor_half_height = 0.0;
    for (entity, half_height) in query_floors.iter(){
        floor = Some(entity);
        floor_half_height = half_height.0;
    }

    let bumper_depth = 0.17;
    let bumper_radius = 0.025;
    let bumper_mesh_handle:Handle<Mesh> = meshes.add(Mesh::from(shape::Capsule {
        depth: bumper_depth,
        radius: bumper_radius,
        ..default()
    }));

    let temp_timestamp_last_hit = timestamp_last_hit.unwrap_or(0.0);

    let mut color = Color::GREEN;
    if temp_timestamp_last_hit == 0.0{
        color = Color::TEAL;
    }

    let material_bumper = materials.add(color.into());

    let bumper = commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: bumper_mesh_handle.clone(),
        material: material_bumper.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::round_cylinder(bumper_depth / 2.0, bumper_radius, 0.002))
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(position.0.x, position.0.y, position.0.z + floor_half_height),
            rotation: rotation.0,
            ..default()
        }
    ))
    .insert(Restitution::coefficient(0.7))
    .insert(Bumper)
    .insert(Position(position.0))
    .insert(Rotation(rotation.0))
    .insert(TimestampLastHit(temp_timestamp_last_hit))
    .id();

    commands.entity(floor.unwrap()).add_child(bumper);
}

fn respawn_bumper_to_toggle_color(mut query_bumpers: Query<(Entity, &Position, &Rotation, &TimestampLastHit), With<Bumper>>, 
        time: Res<Time>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        query_floors: Query<(Entity, &HalfHeight), With<Floor>>,
    ) {
    for (entity, position, rotation, timestamp_last_hit) in query_bumpers.iter_mut() {
        let diff = time.seconds_since_startup() - timestamp_last_hit.0;
        if timestamp_last_hit.0 > 0.0 && diff > 1.0{
            //Color have been toggled for more than a second so respawn
            let pos = position;
            commands.entity(entity).despawn();
            spawn_single_bumper(&mut commands, position, rotation, None, &mut meshes, &mut materials, &query_floors);
        }
    }
}

fn handle_bumper_events(
    query_bumpers: Query<(Entity, &Position, &Rotation, &TimestampLastHit), With<Bumper>>,
    mut query_balls: Query<(Entity, &mut ExternalImpulse, &Velocity), With<Ball>>,
    time: Res<Time>,
    mut contact_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<(Entity, &HalfHeight), With<Floor>>,
) {
    for contact_event in contact_events.iter() {
        for (entity, position, rotation, timestamp_last_hit) in query_bumpers.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity || h2 == &entity {
                    //Respawn to change color
                    let timestamp_last_hit = time.seconds_since_startup();
                    commands.entity(entity).despawn();
                    spawn_single_bumper(&mut commands, position, rotation, Some(timestamp_last_hit), &mut meshes, &mut materials, &query_floors);
                }
            }
            if let CollisionEvent::Stopped(h1, h2, _event_flag) = contact_event {
                if h1 == &entity || h2 == &entity {
                    //Give ball a push in velocity direction
                    for (entity_ball, mut external_impulse, velocity) in query_balls.iter_mut() {
                        if h1 == &entity_ball || h2 == &entity_ball {
                            let normalized_velocity = velocity.linvel.normalize();
                            external_impulse.impulse = external_impulse.impulse.add(normalized_velocity * 0.000003);
                        }
                    }
                }
            }
        }
    }
}