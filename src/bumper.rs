use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::Ball;
use super::Floor;
use super::HalfHeight;

use super::common;

pub struct BumperPlugin;

impl Plugin for BumperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_bumpers)
            .add_systems(Update, (handle_bumper_events, change_bumper_to_dark_color));
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

    for init_bumper in &init_bumpers {
        spawn_single_bumper(
            &mut commands,
            init_bumper.position,
            init_bumper.rotation,
            None,
            &init_bumper.dark_color,
            &init_bumper.light_color,
            &mut meshes,
            &mut materials,
            &query_floors,
            init_bumper.despawn_in_endgame,
        );
    }
}

pub fn spawn_single_bumper(
    commands: &mut Commands,
    position: Vec3,
    rotation: Quat,
    timestamp_last_hit: Option<f64>,
    dark_color: &DarkColor,
    light_color: &LightColor,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    query_floors: &Query<&HalfHeight, With<Floor>>,
    add_despawn_in_endgame: bool,
) {
    let mut floor_half_height = 0.0;
    for half_height in query_floors.iter() {
        floor_half_height = half_height.0;
    }

    let bumper_height = 0.1;
    let bumper_length = 0.17;
    let bumper_width = 0.02;
    let bumper_mesh_handle: Handle<Mesh> = meshes.add(Mesh::from(Cuboid::new(
        bumper_length,
        bumper_width,
        bumper_height,
    )));

    let temp_timestamp_last_hit = timestamp_last_hit.unwrap_or(0.0);

    let mut color = light_color.0;
    if temp_timestamp_last_hit == 0.0 {
        color = dark_color.0;
    }

    let material_bumper = materials.add(color);

    let bumper = commands
        .spawn((
            Mesh3d(bumper_mesh_handle),
            MeshMaterial3d(material_bumper),
            common::board_transform(Transform {
                translation: Vec3::new(
                    position.x,
                    position.y,
                    position.z + bumper_height / 2.0 + floor_half_height,
                ),
                rotation,
                ..default()
            }),
            RigidBody::Fixed,
            Collider::cuboid(bumper_length / 2.0, bumper_width / 2.0, bumper_height / 2.0),
            Restitution::coefficient(0.7),
            Bumper,
            TimestampLastHit(temp_timestamp_last_hit),
            DarkColor(dark_color.0),
            LightColor(light_color.0),
        ))
        .id();

    if add_despawn_in_endgame {
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
    mut query_balls: Query<(Entity, &mut ExternalImpulse, &Velocity), With<Ball>>,
    time: Res<Time>,
    mut contact_events: MessageReader<CollisionEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for contact_event in contact_events.read() {
        for (entity, mut timestamp_last_hit, light_color, mut material) in query_bumpers.iter_mut()
        {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity || h2 == &entity {
                    *timestamp_last_hit = TimestampLastHit(time.elapsed_secs_f64());
                    *material = MeshMaterial3d(materials.add(light_color.0));
                }
            }
            if let CollisionEvent::Stopped(h1, h2, _event_flag) = contact_event {
                if h1 == &entity || h2 == &entity {
                    for (entity_ball, mut external_impulse, velocity) in query_balls.iter_mut() {
                        if h1 == &entity_ball || h2 == &entity_ball {
                            external_impulse.impulse += velocity.linvel.normalize() * 0.000003;
                        }
                    }
                }
            }
        }
    }
}
