use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use rand::Rng;

use super::ball;
use super::bumper;
use super::spawn_single_ball;
use super::Ball;
use super::Floor;
use super::HalfHeight;

use super::common;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_star)
            .add_system(handle_star_ball_sensor_events)
            .add_system(despawn_collector_when_endgame);
    }
}

#[derive(Default, Component)]
struct Star;

#[derive(Default, Component)]
struct CollectorSensor;

fn spawn_star(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<(Entity, &HalfHeight), With<Floor>>,
) {
    let init_star_bumpers: [bumper::BumperBundle; 4] = [
        bumper::BumperBundle {
            position: common::Position(Vec3::new(-0.06, 0.3, 0.0)),
            rotation: common::Rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
            dark_color: bumper::DarkColor(Color::YELLOW),
            light_color: bumper::LightColor(Color::ANTIQUE_WHITE),
            despawn_in_endgame: false,
        },
        bumper::BumperBundle {
            position: common::Position(Vec3::new(0.06, 0.3, 0.0)),
            rotation: common::Rotation(Quat::from_rotation_z(-std::f32::consts::PI / 4.0)),
            dark_color: bumper::DarkColor(Color::YELLOW),
            light_color: bumper::LightColor(Color::ANTIQUE_WHITE),
            despawn_in_endgame: false,
        },
        bumper::BumperBundle {
            position: common::Position(Vec3::new(0.06, 0.19, 0.0)),
            rotation: common::Rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
            dark_color: bumper::DarkColor(Color::YELLOW),
            light_color: bumper::LightColor(Color::ANTIQUE_WHITE),
            despawn_in_endgame: true,
        },
        bumper::BumperBundle {
            position: common::Position(Vec3::new(-0.06, 0.19, -0.025)),
            rotation: common::Rotation(Quat::from_rotation_z(-std::f32::consts::PI / 4.0)),
            dark_color: bumper::DarkColor(Color::YELLOW),
            light_color: bumper::LightColor(Color::ANTIQUE_WHITE),
            despawn_in_endgame: false,
        },
    ];

    for i in 0..init_star_bumpers.len() {
        let init_bumper = &init_star_bumpers[i];

        bumper::spawn_single_bumper(
            &mut commands,
            &init_bumper.position,
            &init_bumper.rotation,
            None,
            &init_bumper.dark_color,
            &init_bumper.light_color,
            &mut meshes,
            &mut materials,
            &query_floors,
            init_bumper.despawn_in_endgame,
        );
    }

    //spawn ball_collector_collider_box
    let collector_collider_position = Vec3::new(0.0, 0.235, 0.01);
    let collector_collider_element = Collider::cuboid(0.06, 0.003, 0.07);
    let collector_collider = commands
        .spawn(RigidBody::Fixed)
        .with_children(|children| {
            children
                .spawn(collector_collider_element.clone())
                .insert(TransformBundle::from(Transform {
                    translation: Vec3::new(-0.04, 0.06, 0.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
                    ..default()
                }));
            children
                .spawn(collector_collider_element.clone())
                .insert(TransformBundle::from(Transform {
                    translation: Vec3::new(0.04, 0.06, 0.0),
                    rotation: Quat::from_rotation_z(-std::f32::consts::PI / 4.0),
                    ..default()
                }));
            children
                .spawn(collector_collider_element.clone())
                .insert(TransformBundle::from(Transform {
                    translation: Vec3::new(-0.04, -0.035, 0.0),
                    rotation: Quat::from_rotation_z(-std::f32::consts::PI / 4.0),
                    ..default()
                }));
            children
                .spawn(collector_collider_element.clone())
                .insert(TransformBundle::from(Transform {
                    translation: Vec3::new(0.04, -0.035, 0.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
                    ..default()
                }));
        })
        .insert(CollisionGroups {
            memberships: Group::GROUP_2,
            filters: Group::GROUP_3,
        })
        .insert(TransformBundle::from(Transform {
            translation: Vec3::new(
                collector_collider_position.x,
                collector_collider_position.y,
                collector_collider_position.z,
            ),
            //rotation: Quat::from_rotation_z(-1.1),
            ..default()
        }))
        .insert(common::DespawnInEndGame)
        .id();

    //spwan one way lid on ball_collector_box, so that balls will stay inside box.
    let oneway_collector_lid = commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(0.07, 0.07, 0.001))
        .insert(CollisionGroups {
            memberships: Group::GROUP_5,
            filters: Group::GROUP_3,
        })
        .insert(TransformBundle::from(Transform {
            translation: Vec3::new(
                collector_collider_position.x,
                collector_collider_position.y,
                collector_collider_position.z + 0.06,
            ),
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
            ..default()
        }))
        .insert(common::DespawnInEndGame)
        .id();

    //spawn star_ball_sensor. Used to detect balls arriving in star and spawn new ball in launcher.
    //  also change group of ball so one way lid does its job.
    let collector_sensor = commands
        .spawn(Sensor)
        .insert(Collider::cuboid(0.07, 0.07, 0.001))
        .insert(TransformBundle::from(Transform {
            translation: Vec3::new(
                collector_collider_position.x,
                collector_collider_position.y,
                collector_collider_position.z + 0.03,
            ),
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
            ..default()
        }))
        .insert(CollectorSensor)
        .insert(common::DespawnInEndGame)
        .id();

    //Starramp
    let starramp_height = 0.06;
    let starramp_length = 0.16;
    let starramp_width = 0.1;
    let starramp_position = Vec3::new(-0.1, 0.135, 0.02);
    let starramp_mesh_handle: Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(
        starramp_length,
        starramp_width,
        starramp_height,
    )));
    let starramp_material = materials.add(Color::rgba(1.0, 1.0, 0.0, 0.8).into());

    let starramp = commands
        .spawn(PbrBundle {
            mesh: starramp_mesh_handle.clone(),
            material: starramp_material.clone(),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            starramp_length / 2.0,
            starramp_width / 2.0,
            starramp_height / 2.0,
        ))
        .insert(CollisionGroups {
            memberships: Group::GROUP_1,
            filters: Group::GROUP_3,
        })
        .insert(TransformBundle::from(Transform {
            translation: Vec3::new(
                starramp_position.x,
                starramp_position.y,
                starramp_position.z,
            ),
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0)
                * Quat::from_rotation_y(-std::f32::consts::PI / 6.0),
            ..default()
        }))
        .id();

    let mut floor = None;
    for (entity, _half_height) in query_floors.iter() {
        floor = Some(entity);
    }

    commands.entity(floor.unwrap()).push_children(&[
        collector_collider,
        oneway_collector_lid,
        collector_sensor,
        starramp,
    ]);
}

fn handle_star_ball_sensor_events(
    query_collector_sensors: Query<Entity, With<CollectorSensor>>,
    mut query_balls: Query<(Entity, &mut CollisionGroups), With<Ball>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut end_game: ResMut<common::EndGame>,
) {
    for contact_event in contact_events.iter() {
        for sensor_entity in query_collector_sensors.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &sensor_entity || h2 == &sensor_entity {
                    let mut group5_added = false;
                    for (entity_ball, mut collision_group) in query_balls.iter_mut() {
                        if h1 == &entity_ball || h2 == &entity_ball {
                            //Add GROUP_5 to filters. This will activate collision between the ball and the one way gate collider
                            if (collision_group.filters & Group::GROUP_5) == Group::NONE {
                                collision_group.filters = Group::GROUP_1
                                    | Group::GROUP_2
                                    | Group::GROUP_3
                                    | Group::GROUP_4
                                    | Group::GROUP_5;
                                group5_added = true;
                            }
                        }
                    }
                    if group5_added {
                        //spawn new ball
                        let color_selection: [Color; 5] = [
                            Color::YELLOW,
                            Color::ORANGE,
                            Color::YELLOW_GREEN,
                            Color::GREEN,
                            Color::PINK,
                        ];
                        let mut rng = rand::thread_rng();
                        let chosen_index = rng.gen_range(0..5);
                        spawn_single_ball(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            &ball::INIT_BALL_POSITION,
                            &ball::MaterialColor(color_selection[chosen_index].into()),
                        );

                        //If five balls in collector. Let end_game begin.
                        //  Set endgame resource.
                        let mut balls_group5_counter = 0;
                        for (_entity, collision_group) in query_balls.iter() {
                            //Add GROUP_5 to filters. This will activate collision between the ball and the one way gate collider
                            if (collision_group.filters & Group::GROUP_5) == Group::GROUP_5 {
                                balls_group5_counter += 1;
                            }
                        }

                        if balls_group5_counter > 4 {
                            end_game.0 = true;
                        }
                    }
                }
            }
        }
    }
}

fn despawn_collector_when_endgame(
    mut commands: Commands,
    query_despawn_entities: Query<Entity, With<common::DespawnInEndGame>>,
    mut query_balls: Query<(Entity, &mut CollisionGroups), With<Ball>>,
    end_game: Res<common::EndGame>,
    mut done: Local<bool>,
) {
    if !*done {
        if end_game.0 == true {
            //  Despawn collector, lid, sensor and right-down bumper.
            for entity_to_despawn in query_despawn_entities.iter() {
                commands.entity(entity_to_despawn).despawn_recursive();
            }
            // remove GROUP_5 on all balls
            for (_entity_ball, mut collision_group) in query_balls.iter_mut() {
                //Remove GROUP_5 from filter
                collision_group.filters = collision_group.filters ^ Group::GROUP_5;
            }

            *done = true;
        }
    }
}
