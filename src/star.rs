use avian3d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

use super::ball;
use super::common;
use super::common::GameLayer;
use super::spawn_single_ball;
use super::Ball;

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
    _commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
}

fn handle_star_ball_sensor_events(
    query_collector_sensors: Query<Entity, With<CollectorSensor>>,
    query_balls: Query<(Entity, &CollisionLayers), With<Ball>>,
    mut contact_events: MessageReader<CollisionStart>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut end_game: ResMut<common::EndGame>,
) {
    for event in contact_events.read() {
        for sensor_entity in query_collector_sensors.iter() {
            if event.collider1 == sensor_entity || event.collider2 == sensor_entity {
                let ball_entity = if event.collider1 == sensor_entity {
                    event.collider2
                } else {
                    event.collider1
                };

                let mut lid_added = false;
                if let Ok((_, layers)) = query_balls.get(ball_entity) {
                    let lid_mask = LayerMask::from(GameLayer::Lid);
                    if (layers.filters & lid_mask) == LayerMask::NONE {
                        commands.entity(ball_entity).insert(CollisionLayers {
                            memberships: layers.memberships,
                            filters: layers.filters | lid_mask,
                        });
                        lid_added = true;
                    }
                }

                if lid_added {
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

                    let balls_in_collector = query_balls
                        .iter()
                        .filter(|(_, layers)| {
                            (layers.filters & LayerMask::from(GameLayer::Lid)) != LayerMask::NONE
                        })
                        .count();

                    if balls_in_collector > 4 {
                        end_game.0 = true;
                    }
                }
            }
        }
    }
}

fn despawn_collector_when_endgame(
    mut commands: Commands,
    query_despawn_entities: Query<Entity, With<common::DespawnInEndGame>>,
    query_balls: Query<(Entity, &CollisionLayers), With<Ball>>,
    end_game: Res<common::EndGame>,
    mut done: Local<bool>,
) {
    if !*done && end_game.0 {
        for entity_to_despawn in query_despawn_entities.iter() {
            commands.entity(entity_to_despawn).despawn();
        }
        for (entity, layers) in query_balls.iter() {
            let lid_mask = LayerMask::from(GameLayer::Lid);
            commands.entity(entity).insert(CollisionLayers {
                memberships: layers.memberships,
                filters: layers.filters ^ lid_mask,
            });
        }
        *done = true;
    }
}
