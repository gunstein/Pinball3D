use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use std::ops::Add;
use rand::Rng;

use super::Floor;
use super::Ball;
use super::HalfHeight;
use super::spawn_single_ball;
use super::ball;
use super::bumper;

use super::common;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_star)
            .add_system(handle_star_ball_sensor_events);
    }
}

#[derive(Default, Component)]
struct Star;

#[derive(Default, Component)]
struct StarBallSensor;

fn spawn_star(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<(Entity, &HalfHeight), With<Floor>>
)
{
    let init_star_bumpers : [bumper::BumperBundle;4] = [
        bumper::BumperBundle{
            position: common::Position(Vec3::new(-0.05, 0.3, 0.0)),
            rotation: common::Rotation(Quat::from_rotation_z(1.0)),
            dark_color: bumper::DarkColor(Color::YELLOW),
            light_color: bumper::LightColor(Color::ANTIQUE_WHITE)
        },  
        bumper::BumperBundle{
            position: common::Position(Vec3::new(0.05, 0.3, 0.0)),
            rotation: common::Rotation(Quat::from_rotation_z(std::f32::consts::PI - 1.0)),
            dark_color: bumper::DarkColor(Color::YELLOW),
            light_color: bumper::LightColor(Color::ANTIQUE_WHITE)
        },  
        bumper::BumperBundle{
            position: common::Position(Vec3::new(0.05, 0.17, 0.0)),
            rotation: common::Rotation(Quat::from_rotation_z(1.0)),
            dark_color: bumper::DarkColor(Color::YELLOW),
            light_color: bumper::LightColor(Color::ANTIQUE_WHITE)
        },
        bumper::BumperBundle{
            position: common::Position(Vec3::new(-0.05, 0.17, -0.03)),
            rotation: common::Rotation(Quat::from_rotation_z(std::f32::consts::PI - 1.0)),
            dark_color: bumper::DarkColor(Color::YELLOW),
            light_color: bumper::LightColor(Color::ANTIQUE_WHITE)
        },                
    ];

    for i in 0..init_star_bumpers.len() {
        let init_bumper = &init_star_bumpers[i];

        bumper::spawn_single_bumper(&mut commands, &init_bumper.position, &init_bumper.rotation, None, 
            &init_bumper.dark_color, &init_bumper.light_color, &mut meshes, &mut materials, &query_floors);
    }

    //spawn star_ball_sensor. Used to detect balls arriving in star and spawn new ball in launcher.
    let mut floor = None;
    let mut floor_half_height = 0.0;
    for (entity, half_height) in query_floors.iter(){
        floor = Some(entity);
        floor_half_height = half_height.0;
    }

    let star_ball_sensor_position = Vec3::new(0.0, 0.22, floor_half_height + 0.09);
    let star_ball_sensor = commands.spawn()
    .insert(Collider::cuboid(0.06, 0.06, 0.01))
    .insert(Sensor)
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(star_ball_sensor_position.x, star_ball_sensor_position.y, star_ball_sensor_position.z),
            rotation: Quat::from_rotation_z(std::f32::consts::PI/4.5),
            ..default()
        }
    ))
    //.insert_bundle(TransformBundle::from(Transform::from_xyz(star_ball_sensor_position.x, star_ball_sensor_position.y, star_ball_sensor_position.z)))
    .insert(StarBallSensor)
    .id(); 

    commands.entity(floor.unwrap()).add_child(star_ball_sensor);
}

fn handle_star_ball_sensor_events(
    query_star_ball_sensors: Query<Entity, With<StarBallSensor>>,
    //mut query_balls: Query<(Entity, &mut CollisionGroups), With<Ball>>,
    //query_ball: Query<Entity, With<Ball>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for contact_event in contact_events.iter() {
        for sensor_entity in query_star_ball_sensors.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &sensor_entity || h2 == &sensor_entity {
                    //spawn new ball
                    let color_selection : [Color; 5]= [
                        Color::YELLOW,
                        Color::ORANGE,
                        Color::YELLOW_GREEN,
                        Color::GREEN,
                        Color::PINK
                    ];
                    let mut rng = rand::thread_rng();
                    let chosen_index = rng.gen_range(0..5);
                    spawn_single_ball(&mut commands, &mut meshes, &mut materials, &ball::INIT_BALL_POSITION, &ball::MaterialColor(color_selection[chosen_index].into()));
                }
            }
        }
    }
}