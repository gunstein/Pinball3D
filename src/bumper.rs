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

#[derive(Default, Component)]
struct Bumper;

#[derive(Default, Component)]
struct TimestampLastHit(f64);

#[derive(Default, Component)]
struct Position(Vec3);

#[derive(Default, Component)]
struct Rotation(Quat);

#[derive(Default, Component)]
struct DarkColor(Color);

#[derive(Default, Component)]
struct LightColor(Color);

#[derive(Bundle, Default)]
struct BumperBundle{
    position: Position,
    rotation: Rotation,
    dark_color: DarkColor,
    light_color: LightColor,
}

fn spawn_bumpers(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<(Entity, &HalfHeight), With<Floor>>
)
{
    let init_bumpers : [BumperBundle;6] = [
        BumperBundle{
            position: Position(Vec3::new(-0.2, -0.66, 0.0)),
            rotation: Rotation(Quat::from_rotation_z(-0.6)),
            dark_color: DarkColor(Color::RED),
            light_color: LightColor(Color::GOLD)
        },
        BumperBundle{
            position: Position(Vec3::new(-0.28, -0.53, 0.0)),
            rotation: Rotation(Quat::from_rotation_z( std::f32::consts::PI/2.0 + 0.12)),
            dark_color: DarkColor(Color::RED),
            light_color: LightColor(Color::GOLD)
        },
        BumperBundle{
            position: Position(Vec3::new(-0.05, 0.3, 0.0)),
            rotation: Rotation(Quat::from_rotation_z(1.0)),
            dark_color: DarkColor(Color::YELLOW),
            light_color: LightColor(Color::ANTIQUE_WHITE)
        },  
        BumperBundle{
            position: Position(Vec3::new(0.05, 0.3, 0.0)),
            rotation: Rotation(Quat::from_rotation_z(std::f32::consts::PI - 1.0)),
            dark_color: DarkColor(Color::YELLOW),
            light_color: LightColor(Color::ANTIQUE_WHITE)
        },  
        BumperBundle{
            position: Position(Vec3::new(0.05, 0.17, 0.0)),
            rotation: Rotation(Quat::from_rotation_z(1.0)),
            dark_color: DarkColor(Color::YELLOW),
            light_color: LightColor(Color::ANTIQUE_WHITE)
        },
        BumperBundle{
            position: Position(Vec3::new(-0.05, 0.17, -0.03)),
            rotation: Rotation(Quat::from_rotation_z(std::f32::consts::PI - 1.0)),
            dark_color: DarkColor(Color::YELLOW),
            light_color: LightColor(Color::ANTIQUE_WHITE)
        },                
    ];

    for i in 0..init_bumpers.len() {
        let init_bumper = &init_bumpers[i];

        spawn_single_bumper(&mut commands, &init_bumper.position, &init_bumper.rotation, None, 
            &init_bumper.dark_color, &init_bumper.light_color, &mut meshes, &mut materials, &query_floors);
    }
}


fn spawn_single_bumper(    
    commands: &mut Commands,
    position: & Position,
    rotation: & Rotation,
    timestamp_last_hit: Option<f64>,
    dark_color: &DarkColor,
    light_color: &LightColor,
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

    let bumper_height = 0.1;
    let bumper_length = 0.17;
    let bumper_width = 0.02;
    let bumper_mesh_handle:Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(bumper_length, bumper_width, bumper_height)));

    let temp_timestamp_last_hit = timestamp_last_hit.unwrap_or(0.0);

    let mut color = light_color.0.as_rgba();
    if temp_timestamp_last_hit == 0.0{
        color = dark_color.0.as_rgba();
    }

    let material_bumper = materials.add(color.into());

    let bumper = commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: bumper_mesh_handle.clone(),
        material: material_bumper.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(bumper_length / 2.0, bumper_width / 2.0, bumper_height / 2.0))
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(position.0.x, position.0.y, position.0.z + bumper_height/2.0 + floor_half_height),
            rotation: rotation.0,
            ..default()
        }
    ))
    .insert(Restitution::coefficient(0.7))
    .insert(Bumper)
    .insert(Position(position.0))
    .insert(Rotation(rotation.0))
    .insert(TimestampLastHit(temp_timestamp_last_hit))
    .insert(DarkColor(dark_color.0))
    .insert(LightColor(light_color.0))
    .id();

    commands.entity(floor.unwrap()).add_child(bumper);
}

fn respawn_bumper_to_toggle_color(mut query_bumpers: Query<(Entity, &Position, &Rotation, &TimestampLastHit, &DarkColor, &LightColor), With<Bumper>>, 
        time: Res<Time>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        query_floors: Query<(Entity, &HalfHeight), With<Floor>>,
    ) {
    for (entity, position, rotation, timestamp_last_hit, dark_color, light_color) in query_bumpers.iter_mut() {
        let diff = time.seconds_since_startup() - timestamp_last_hit.0;
        if timestamp_last_hit.0 > 0.0 && diff > 1.0{
            //Color have been toggled for more than a second so respawn
            let pos = position;
            commands.entity(entity).despawn();
            spawn_single_bumper(&mut commands, position, rotation, None, dark_color, light_color, &mut meshes, &mut materials, &query_floors);
        }
    }
}

fn handle_bumper_events(
    query_bumpers: Query<(Entity, &Position, &Rotation, &TimestampLastHit, &DarkColor, &LightColor), With<Bumper>>,
    mut query_balls: Query<(Entity, &mut ExternalImpulse, &Velocity), With<Ball>>,
    time: Res<Time>,
    mut contact_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<(Entity, &HalfHeight), With<Floor>>,
) {
    for contact_event in contact_events.iter() {
        for (entity, position, rotation, timestamp_last_hit, dark_color, light_color) in query_bumpers.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity || h2 == &entity {
                    //Respawn to change color
                    let timestamp_last_hit = time.seconds_since_startup();
                    commands.entity(entity).despawn();
                    spawn_single_bumper(&mut commands, position, rotation, Some(timestamp_last_hit), dark_color, light_color, &mut meshes, &mut materials, &query_floors);
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