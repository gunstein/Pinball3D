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
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_bumpers);
            //.add_system(handle_pin_events)
            //.add_system(respawn_pin_to_toggle_color);
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
    position: Vec3,
    rotation: Quat,
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
            position: Vec3::new(-0.2, -0.65, 0.0),
            rotation: Quat::from_rotation_z(1.1)
        }
    ];

    for i in 0..init_bumpers.len() {
        let init_bumper = &init_bumpers[i];

        spawn_single_bumper(&mut commands, init_bumper, None, &mut meshes, &mut materials, &query_floors);
    }
}


fn spawn_single_bumper(    
    commands: &mut Commands,
    init_bumper: & InitBumper,
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
            translation: Vec3::new(init_bumper.position.x, init_bumper.position.y, init_bumper.position.z + floor_half_height),
            rotation: init_bumper.rotation,
            ..default()
        }
    ))
    .insert(Restitution::coefficient(0.7))
    .insert(Bumper)
    .insert(Position(init_bumper.position))
    .insert(Rotation(init_bumper.rotation))
    .insert(TimestampLastHit(temp_timestamp_last_hit))
    .id();

    commands.entity(floor.unwrap()).add_child(bumper);
}
