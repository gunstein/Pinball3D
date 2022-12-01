use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use std::ops::Add;

use super::Floor;
use super::Ball;
use super::HalfHeight;

use super::common;

pub struct BumperPlugin;

impl Plugin for BumperPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_bumpers)
            .add_system(handle_bumper_events)
            .add_system(change_bumper_to_dark_color);
    }
}

#[derive(Default, Component)]
struct Bumper;

#[derive(Default, Component)]
pub struct TimestampLastHit(f64);

#[derive(Default, Component)]
pub struct DarkColor(pub Color);

#[derive(Default, Component)]
pub struct LightColor(pub Color);

#[derive(Default, Component)]
struct StarBallSensor;

#[derive(Bundle, Default)]
pub struct BumperBundle{
    pub position: common::Position,
    pub rotation: common::Rotation,
    pub dark_color: DarkColor,
    pub light_color: LightColor,
}

fn spawn_bumpers(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<(Entity, &HalfHeight), With<Floor>>
)
{
    let init_bumpers : [BumperBundle;2] = [
        BumperBundle{
            position: common::Position(Vec3::new(-0.2, -0.66, 0.0)),
            rotation: common::Rotation(Quat::from_rotation_z(-0.6)),
            dark_color: DarkColor(Color::RED),
            light_color: LightColor(Color::GOLD)
        },
        BumperBundle{
            position: common::Position(Vec3::new(-0.28, -0.53, 0.0)),
            rotation: common::Rotation(Quat::from_rotation_z( std::f32::consts::PI/2.0 + 0.12)),
            dark_color: DarkColor(Color::RED),
            light_color: LightColor(Color::GOLD)
        }           
    ];

    for i in 0..init_bumpers.len() {
        let init_bumper = &init_bumpers[i];

        spawn_single_bumper(&mut commands, &init_bumper.position, &init_bumper.rotation, None, 
            &init_bumper.dark_color, &init_bumper.light_color, &mut meshes, &mut materials, &query_floors);
    }
}


pub fn spawn_single_bumper(    
    commands: &mut Commands,
    position: & common::Position,
    rotation: & common::Rotation,
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
    .insert(common::Position(position.0))
    .insert(common::Rotation(rotation.0))
    .insert(TimestampLastHit(temp_timestamp_last_hit))
    .insert(DarkColor(dark_color.0))
    .insert(LightColor(light_color.0))
    .id();

    commands.entity(floor.unwrap()).add_child(bumper);
}

//fn respawn_bumper_to_toggle_color(mut query_bumpers: Query<(Entity, &Position, &Rotation, &TimestampLastHit, &DarkColor, &LightColor), With<Bumper>>, 
fn change_bumper_to_dark_color(mut query_bumpers: Query<(&TimestampLastHit, &DarkColor, &mut Handle<StandardMaterial>), With<Bumper>>, 
        time: Res<Time>,
        //mut commands: Commands,
        //mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        //query_floors: Query<(Entity, &HalfHeight), With<Floor>>,
    ) {
    //for (entity, position, rotation, timestamp_last_hit, dark_color, light_color) in query_bumpers.iter_mut() {
    for ( timestamp_last_hit, dark_color, mut material) in query_bumpers.iter_mut() {
        let diff = time.seconds_since_startup() - timestamp_last_hit.0;
        if timestamp_last_hit.0 > 0.0 && diff > 1.0{
            //Color have been toggled for more than a second so respawn
            //let pos = position;
            //commands.entity(entity).despawn();
            //spawn_single_bumper(&mut commands, position, rotation, None, dark_color, light_color, &mut meshes, &mut materials, &query_floors);
            
            let dark_material_bumper = materials.add(dark_color.0.into());
            *material = dark_material_bumper.clone();
        }
    }
}

fn handle_bumper_events(
    mut query_bumpers: Query<(Entity, &mut TimestampLastHit, &LightColor, &mut Handle<StandardMaterial>), With<Bumper>>,
    mut query_balls: Query<(Entity, &mut ExternalImpulse, &Velocity), With<Ball>>,
    time: Res<Time>,
    mut contact_events: EventReader<CollisionEvent>,
    //mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //query_floors: Query<(Entity, &HalfHeight), With<Floor>>,
) {
    for contact_event in contact_events.iter() {
        for (entity, mut timestamp_last_hit, light_color, mut material) in query_bumpers.iter_mut() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity || h2 == &entity {
                    //Change to light color
                    *timestamp_last_hit = TimestampLastHit(time.seconds_since_startup());
                    //commands.entity(entity).despawn();
                    //spawn_single_bumper(&mut commands, position, rotation, Some(timestamp_last_hit), dark_color, light_color, &mut meshes, &mut materials, &query_floors);
                    let light_material_bumper = materials.add(light_color.0.into());
                    *material = light_material_bumper.clone();
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
