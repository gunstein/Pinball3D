use avian3d::prelude::*;
use bevy::prelude::*;

use super::common;
use super::common::GameLayer;
use super::Ball;
use super::Floor;
use super::HalfHeight;

pub struct BumperPlugin;

impl Plugin for BumperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_bumpers)
            .add_systems(Update, (handle_bumper_events, change_bumper_to_dark_color));
    }
}

#[derive(Component)]
struct Bumper;

#[derive(Default, Component)]
pub struct TimestampLastHit(f64);

#[derive(Default, Component)]
pub struct DarkColor(pub Color);

#[derive(Default, Component)]
pub struct LightColor(pub Color);

pub struct BumperConfig {
    pub position: Vec3,
    pub rotation: Quat,
    pub dark_color: DarkColor,
    pub light_color: LightColor,
    pub despawn_in_endgame: bool,
}

fn spawn_bumpers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<&HalfHeight, With<Floor>>,
) {
    let init_bumpers: [BumperConfig; 2] = [
        BumperConfig {
            position: Vec3::new(-0.2, -0.66, 0.0),
            rotation: Quat::from_rotation_z(-0.6),
            dark_color: DarkColor(Color::srgb(1.0, 0.0, 0.0)),
            light_color: LightColor(Color::srgb(1.0, 0.843, 0.0)),
            despawn_in_endgame: false,
        },
        BumperConfig {
            position: Vec3::new(-0.28, -0.53, 0.0),
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.0 + 0.12),
            dark_color: DarkColor(Color::srgb(1.0, 0.0, 0.0)),
            light_color: LightColor(Color::srgb(1.0, 0.843, 0.0)),
            despawn_in_endgame: false,
        },
    ];

    for config in &init_bumpers {
        spawn_single_bumper(
            &mut commands,
            config,
            &mut meshes,
            &mut materials,
            &query_floors,
        );
    }
}

pub fn spawn_single_bumper(
    commands: &mut Commands,
    config: &BumperConfig,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    query_floors: &Query<&HalfHeight, With<Floor>>,
) {
    let mut floor_half_height = 0.0;
    for half_height in query_floors.iter() {
        floor_half_height = half_height.0;
    }

    let bumper_height = 0.1;
    let bumper_length = 0.17;
    let bumper_width = 0.02;

    let bumper = commands
        .spawn((
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(
                bumper_length,
                bumper_width,
                bumper_height,
            )))),
            MeshMaterial3d(materials.add(config.dark_color.0)),
            common::board_transform(Transform {
                translation: Vec3::new(
                    config.position.x,
                    config.position.y,
                    config.position.z + bumper_height / 2.0 + floor_half_height,
                ),
                rotation: config.rotation,
                ..default()
            }),
            RigidBody::Static,
            Collider::cuboid(bumper_length, bumper_width, bumper_height),
            CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
            Restitution::new(0.7),
            CollisionEventsEnabled,
            Bumper,
            TimestampLastHit::default(),
            DarkColor(config.dark_color.0),
            LightColor(config.light_color.0),
        ))
        .id();

    if config.despawn_in_endgame {
        commands.entity(bumper).insert(common::DespawnInEndGame);
    }
}

fn change_bumper_to_dark_color(
    mut query_bumpers: Query<
        (
            &mut TimestampLastHit,
            &DarkColor,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        With<Bumper>,
    >,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut timestamp_last_hit, dark_color, mut material) in &mut query_bumpers {
        let diff = time.elapsed_secs_f64() - timestamp_last_hit.0;
        if timestamp_last_hit.0 > 0.0 && diff > 1.0 {
            *material = MeshMaterial3d(materials.add(dark_color.0));
            timestamp_last_hit.0 = 0.0;
        }
    }
}

fn handle_bumper_events(
    mut query_bumpers: Query<
        (
            Entity,
            &mut TimestampLastHit,
            &LightColor,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        With<Bumper>,
    >,
    mut query_balls: Query<(Entity, Forces), With<Ball>>,
    time: Res<Time>,
    mut start_events: MessageReader<CollisionStart>,
    mut end_events: MessageReader<CollisionEnd>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in start_events.read() {
        for (entity, mut timestamp_last_hit, light_color, mut material) in query_bumpers.iter_mut()
        {
            if event.collider1 == entity || event.collider2 == entity {
                *timestamp_last_hit = TimestampLastHit(time.elapsed_secs_f64());
                *material = MeshMaterial3d(materials.add(light_color.0));
            }
        }
    }

    for event in end_events.read() {
        for (entity, ..) in query_bumpers.iter() {
            if event.collider1 == entity || event.collider2 == entity {
                let ball_entity = if event.collider1 == entity {
                    event.collider2
                } else {
                    event.collider1
                };
                if let Ok((_, mut forces)) = query_balls.get_mut(ball_entity) {
                    let velocity = forces.linear_velocity();
                    forces.apply_linear_impulse(velocity.normalize_or_zero() * 0.000003);
                }
            }
        }
    }
}
