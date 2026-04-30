use avian3d::prelude::*;
use bevy::prelude::*;

use super::common;
use super::common::GameLayer;
use super::Ball;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_launcher_and_gate)
            .add_systems(Update, (launcher_movement, handle_gate_sensor_events));
    }
}

#[derive(Component)]
struct Launcher {
    start_pos: Vec3,
}

#[derive(Component)]
struct GateSensor;

fn spawn_launcher_and_gate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let launcher_pos = Vec3::new(0.34, -0.95, 0.03);

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.02 * 2.0, 0.02 * 2.0, 0.02 * 2.0)))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 0.0))),
        RigidBody::Kinematic,
        SleepingDisabled,
        SweptCcd::default(),
        Collider::cuboid(0.02, 0.02, 0.02),
        CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
        common::board_transform(Transform::from_xyz(
            launcher_pos.x,
            launcher_pos.y,
            launcher_pos.z,
        )),
        Launcher { start_pos: launcher_pos },
    ));

    let gate_anchor_pos = Vec3::new(0.3, -0.42, 0.1);

    let gate_anchor = commands
        .spawn((
            RigidBody::Static,
            common::board_transform(Transform {
                translation: gate_anchor_pos,
                ..default()
            }),
        ))
        .id();

    let launcher_gate = commands
        .spawn((
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.017 * 2.0, 0.003 * 2.0, 0.04 * 2.0)))),
            MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
            RigidBody::Dynamic,
            SleepingDisabled,
            SweptCcd::default(),
            common::board_transform(Transform {
                translation: Vec3::new(gate_anchor_pos.x, gate_anchor_pos.y, gate_anchor_pos.z - 0.04),
                ..default()
            }),
        ))
        .with_children(|children| {
            children.spawn((
                Collider::cuboid(0.017, 0.003, 0.04),
                CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
            ));
        })
        .id();

    commands.spawn(
        RevoluteJoint::new(gate_anchor, launcher_gate)
            .with_hinge_axis(Vec3::X)
            .with_angle_limits(0.0, std::f32::consts::PI / 2.0)
            .with_local_anchor1(Vec3::new(0.015, 0.0, 0.0))
            .with_local_anchor2(Vec3::new(-0.017, 0.0, 0.04)),
    );

    let gate_collider_pos = Vec3::new(0.33, -0.41, 0.05);
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(0.03, 0.003, 0.04),
        CollisionLayers::new(GameLayer::Gate, [GameLayer::Ball]),
        common::board_transform(Transform {
            translation: gate_collider_pos,
            rotation: Quat::from_rotation_z(0.1),
            ..default()
        }),
    ));

    let gate_sensor_position = Vec3::new(0.33, -0.39, 0.05);
    commands.spawn((
        Collider::cuboid(0.03, 0.003, 0.04),
        Sensor,
        CollisionEventsEnabled,
        common::board_transform(Transform::from_xyz(
            gate_sensor_position.x,
            gate_sensor_position.y,
            gate_sensor_position.z,
        )),
        GateSensor,
    ));
}

fn launcher_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut launchers: Query<(&mut Launcher, &mut Transform)>,
) {
    for (launcher, mut launcher_transform) in launchers.iter_mut() {
        let mut next_ypos = launcher_transform.translation.y;

        if keyboard_input.pressed(KeyCode::Space) {
            next_ypos += 0.03;
        } else {
            next_ypos -= 0.02;
        }
        launcher_transform.translation.y =
            next_ypos.clamp(launcher.start_pos.y, launcher.start_pos.y + 0.06);
    }
}

fn handle_gate_sensor_events(
    query_gate_sensors: Query<Entity, With<GateSensor>>,
    query_balls: Query<(Entity, &CollisionLayers), With<Ball>>,
    mut contact_events: MessageReader<CollisionStart>,
    mut commands: Commands,
) {
    for event in contact_events.read() {
        for sensor_entity in query_gate_sensors.iter() {
            if event.collider1 == sensor_entity || event.collider2 == sensor_entity {
                let ball_entity = if event.collider1 == sensor_entity {
                    event.collider2
                } else {
                    event.collider1
                };
                if let Ok((_, layers)) = query_balls.get(ball_entity) {
                    commands.entity(ball_entity).insert(CollisionLayers {
                        memberships: layers.memberships,
                        filters: layers.filters | LayerMask::from(GameLayer::Gate),
                    });
                }
            }
        }
    }
}
