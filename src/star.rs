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
        app.add_systems(PostStartup, spawn_star).add_systems(
            Update,
            (
                handle_star_ball_sensor_events,
                despawn_collector_when_endgame,
            ),
        );
    }
}

#[derive(Component)]
struct CollectorSensor;

fn spawn_star(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<&HalfHeight, With<Floor>>,
) {
    let init_star_bumpers: [bumper::BumperConfig; 4] = [
        bumper::BumperConfig {
            position: Vec3::new(-0.06, 0.3, 0.0),
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
            dark_color: bumper::DarkColor(Color::srgb(1.0, 1.0, 0.0)),
            light_color: bumper::LightColor(Color::srgb(0.98, 0.922, 0.843)),
            despawn_in_endgame: false,
        },
        bumper::BumperConfig {
            position: Vec3::new(0.06, 0.3, 0.0),
            rotation: Quat::from_rotation_z(-std::f32::consts::PI / 4.0),
            dark_color: bumper::DarkColor(Color::srgb(1.0, 1.0, 0.0)),
            light_color: bumper::LightColor(Color::srgb(0.98, 0.922, 0.843)),
            despawn_in_endgame: false,
        },
        bumper::BumperConfig {
            position: Vec3::new(0.06, 0.19, 0.0),
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
            dark_color: bumper::DarkColor(Color::srgb(1.0, 1.0, 0.0)),
            light_color: bumper::LightColor(Color::srgb(0.98, 0.922, 0.843)),
            despawn_in_endgame: true,
        },
        bumper::BumperConfig {
            position: Vec3::new(-0.06, 0.19, -0.025),
            rotation: Quat::from_rotation_z(-std::f32::consts::PI / 4.0),
            dark_color: bumper::DarkColor(Color::srgb(1.0, 1.0, 0.0)),
            light_color: bumper::LightColor(Color::srgb(0.98, 0.922, 0.843)),
            despawn_in_endgame: false,
        },
    ];

    for config in &init_star_bumpers {
        bumper::spawn_single_bumper(&mut commands, config, &mut meshes, &mut materials, &query_floors);
    }

    //spawn ball_collector_collider_box
    let collector_collider_position = Vec3::new(0.0, 0.235, 0.01);
    let collector_collider_element = Collider::cuboid(0.06, 0.003, 0.07);

    commands
        .spawn((
            RigidBody::Fixed,
            CollisionGroups {
                memberships: Group::GROUP_2,
                filters: Group::GROUP_3,
            },
            common::board_transform(Transform {
                translation: collector_collider_position,
                ..default()
            }),
            common::DespawnInEndGame,
        ))
        .with_children(|children| {
            children.spawn((
                collector_collider_element.clone(),
                Transform {
                    translation: Vec3::new(-0.04, 0.06, 0.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
                    ..default()
                },
            ));
            children.spawn((
                collector_collider_element.clone(),
                Transform {
                    translation: Vec3::new(0.04, 0.06, 0.0),
                    rotation: Quat::from_rotation_z(-std::f32::consts::PI / 4.0),
                    ..default()
                },
            ));
            children.spawn((
                collector_collider_element.clone(),
                Transform {
                    translation: Vec3::new(-0.04, -0.035, 0.0),
                    rotation: Quat::from_rotation_z(-std::f32::consts::PI / 4.0),
                    ..default()
                },
            ));
            children.spawn((
                collector_collider_element.clone(),
                Transform {
                    translation: Vec3::new(0.04, -0.035, 0.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
                    ..default()
                },
            ));
        });

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(0.07, 0.07, 0.001),
        CollisionGroups {
            memberships: Group::GROUP_5,
            filters: Group::GROUP_3,
        },
        common::board_transform(Transform {
            translation: Vec3::new(
                collector_collider_position.x,
                collector_collider_position.y,
                collector_collider_position.z + 0.06,
            ),
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
            ..default()
        }),
        common::DespawnInEndGame,
    ));

    commands.spawn((
        Sensor,
        Collider::cuboid(0.07, 0.07, 0.001),
        common::board_transform(Transform {
            translation: Vec3::new(
                collector_collider_position.x,
                collector_collider_position.y,
                collector_collider_position.z + 0.03,
            ),
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
            ..default()
        }),
        CollectorSensor,
        common::DespawnInEndGame,
    ));

    let starramp_height = 0.06;
    let starramp_length = 0.16;
    let starramp_width = 0.1;
    let starramp_position = Vec3::new(-0.1, 0.135, 0.02);

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(starramp_length, starramp_width, starramp_height)))),
        MeshMaterial3d(materials.add(Color::srgba(1.0, 1.0, 0.0, 0.8))),
        RigidBody::Fixed,
        Collider::cuboid(starramp_length / 2.0, starramp_width / 2.0, starramp_height / 2.0),
        CollisionGroups {
            memberships: Group::GROUP_1,
            filters: Group::GROUP_3,
        },
        common::board_transform(Transform {
            translation: starramp_position,
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0)
                * Quat::from_rotation_y(-std::f32::consts::PI / 6.0),
            ..default()
        }),
    ));
}

fn handle_star_ball_sensor_events(
    query_collector_sensors: Query<Entity, With<CollectorSensor>>,
    mut query_balls: Query<(Entity, &mut CollisionGroups), With<Ball>>,
    mut contact_events: MessageReader<CollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut end_game: ResMut<common::EndGame>,
) {
    for contact_event in contact_events.read() {
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
                            Color::srgb(1.0, 1.0, 0.0),
                            Color::srgb(1.0, 0.647, 0.0),
                            Color::srgb(0.6, 0.8, 0.2),
                            Color::srgb(0.0, 0.5, 0.0),
                            Color::srgb(1.0, 0.753, 0.796),
                        ];
                        let mut rng = rand::thread_rng();
                        let chosen_index = rng.gen_range(0..5);
                        spawn_single_ball(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            ball::INIT_BALL_POSITION,
                            ball::MaterialColor(color_selection[chosen_index]),
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
    if !*done && end_game.0 {
        for entity_to_despawn in query_despawn_entities.iter() {
            commands.entity(entity_to_despawn).despawn();
        }
        for (_entity_ball, mut collision_group) in query_balls.iter_mut() {
            collision_group.filters ^= Group::GROUP_5;
        }
        *done = true;
    }
}
