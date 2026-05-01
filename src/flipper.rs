use avian3d::prelude::*;
use bevy::prelude::*;

use super::common;
use super::common::GameLayer;
use super::Ball;
use super::Floor;
use super::HalfHeight;

pub struct FlipperPlugin;

#[derive(Component)]
struct LeftFlipper {
    curr_angle: f32,
    angular_speed: f32,
    base_rotation: Quat,
    pivot_position: Vec3,
    last_position: Vec3,
}

#[derive(Component)]
struct RightFlipper {
    curr_angle: f32,
    angular_speed: f32,
    base_rotation: Quat,
    pivot_position: Vec3,
    last_position: Vec3,
}

impl Plugin for FlipperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_flippers)
            .add_systems(
                FixedUpdate,
                (
                    left_flipper_movement,
                    right_flipper_movement,
                    drive_ball_from_active_flippers,
                )
                    .chain(),
            )
            .add_systems(Update, kick_ball_from_active_flipper);
    }
}

fn spawn_flippers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<&HalfHeight, With<Floor>>,
) {
    let mut floor_half_height = 0.0;
    for half_height in query_floors.iter() {
        floor_half_height = half_height.0;
    }

    let left_flipper_mesh_handle: Handle<Mesh> =
        asset_server.load("left_flipper.glb#Mesh0/Primitive0");
    let material = materials.add(Color::srgb(1.0, 1.0, 0.0));

    let left_flipper_position = Vec3::new(-0.1, -0.8, 0.01);
    let right_flipper_position = Vec3::new(0.1, -0.8, floor_half_height);

    let left_transform =
        common::board_transform(Transform::from_translation(left_flipper_position));
    let right_transform = common::board_transform(Transform {
        translation: right_flipper_position,
        rotation: Quat::from_rotation_z(-std::f32::consts::PI),
        ..default()
    });
    commands.spawn((
        Mesh3d(left_flipper_mesh_handle.clone()),
        MeshMaterial3d(material.clone()),
        RigidBody::Kinematic,
        SleepingDisabled,
        SweptCcd::NON_LINEAR,
        Friction::new(0.35).with_combine_rule(CoefficientCombine::Min),
        Restitution::new(0.25).with_combine_rule(CoefficientCombine::Average),
        flipper_collider(Vec3::ZERO),
        CollisionMargin(0.002),
        SpeculativeMargin(0.12),
        LinearVelocity::ZERO,
        AngularVelocity::ZERO,
        CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
        left_transform,
        LeftFlipper {
            curr_angle: 0.0,
            angular_speed: 0.0,
            base_rotation: left_transform.rotation,
            pivot_position: left_transform.translation,
            last_position: left_transform.translation,
        },
    ));

    commands.spawn((
        Mesh3d(left_flipper_mesh_handle.clone()),
        MeshMaterial3d(material.clone()),
        RigidBody::Kinematic,
        SleepingDisabled,
        SweptCcd::NON_LINEAR,
        Friction::new(0.35).with_combine_rule(CoefficientCombine::Min),
        Restitution::new(0.25).with_combine_rule(CoefficientCombine::Average),
        flipper_collider(Vec3::new(0.0, 0.006, 0.0)),
        CollisionMargin(0.002),
        SpeculativeMargin(0.12),
        LinearVelocity::ZERO,
        AngularVelocity::ZERO,
        CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
        right_transform,
        RightFlipper {
            curr_angle: 0.0,
            angular_speed: 0.0,
            base_rotation: right_transform.rotation,
            pivot_position: right_transform.translation,
            last_position: right_transform.translation,
        },
    ));
}

fn flipper_collider(offset: Vec3) -> Collider {
    let vertical_cylinder = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let scale = 0.81;
    let base_center = Vec3::new(-0.004, -0.004, 0.0) * scale + offset;
    let tip_center = Vec3::new(0.081, -0.004, 0.0) * scale + offset;
    let base_radius = 0.0175 * scale;
    let tip_radius = 0.0115 * scale;
    let side_thickness = 0.008 * scale;
    let side_height = 0.04 * scale;
    let top_start = base_center.xy() + Vec2::Y * base_radius;
    let top_end = tip_center.xy() + Vec2::Y * tip_radius;
    let bottom_start = base_center.xy() - Vec2::Y * base_radius;
    let bottom_end = tip_center.xy() - Vec2::Y * tip_radius;
    let top_delta = top_end - top_start;
    let bottom_delta = bottom_end - bottom_start;
    let side_length = top_delta.length() + 0.006 * scale;

    Collider::compound(vec![
        (
            base_center,
            vertical_cylinder,
            Collider::cylinder(base_radius, side_height),
        ),
        (
            Vec3::new(
                (top_start.x + top_end.x) * 0.5,
                (top_start.y + top_end.y) * 0.5,
                0.0,
            ),
            Quat::from_rotation_z(top_delta.y.atan2(top_delta.x)),
            Collider::cuboid(side_length, side_thickness, side_height),
        ),
        (
            Vec3::new(
                (bottom_start.x + bottom_end.x) * 0.5,
                (bottom_start.y + bottom_end.y) * 0.5,
                0.0,
            ),
            Quat::from_rotation_z(bottom_delta.y.atan2(bottom_delta.x)),
            Collider::cuboid(side_length, side_thickness, side_height),
        ),
        (
            tip_center,
            vertical_cylinder,
            Collider::cylinder(tip_radius, side_height),
        ),
    ])
}

fn left_flipper_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut left_flippers: Query<(
        &mut LeftFlipper,
        &mut Position,
        &mut Rotation,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
) {
    let delta_secs = time.delta_secs();
    if delta_secs <= f32::EPSILON {
        return;
    }
    for (mut left_flipper, mut position, mut rotation, mut linear_velocity, mut angular_velocity) in
        left_flippers.iter_mut()
    {
        let speed = if keyboard_input.pressed(KeyCode::ArrowLeft) {
            36.0
        } else {
            -4.2
        };
        let previous_angle = left_flipper.curr_angle;
        let new_angle = (previous_angle + speed * delta_secs).clamp(-0.3, 0.3);
        let angular_speed = (new_angle - previous_angle) / delta_secs;

        position.0 = left_flipper.pivot_position;
        rotation.0 = left_flipper.base_rotation * Quat::from_rotation_z(new_angle);
        linear_velocity.0 = (position.0 - left_flipper.last_position) / delta_secs;
        angular_velocity.0 = rotation.0 * Vec3::Z * angular_speed;
        left_flipper.curr_angle = new_angle;
        left_flipper.angular_speed = angular_speed;
        left_flipper.last_position = position.0;
    }
}

fn right_flipper_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut right_flippers: Query<(
        &mut RightFlipper,
        &mut Position,
        &mut Rotation,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
) {
    let delta_secs = time.delta_secs();
    if delta_secs <= f32::EPSILON {
        return;
    }
    for (
        mut right_flipper,
        mut position,
        mut rotation,
        mut linear_velocity,
        mut angular_velocity,
    ) in right_flippers.iter_mut()
    {
        let speed = if keyboard_input.pressed(KeyCode::ArrowRight) {
            -36.0
        } else {
            4.2
        };
        let previous_angle = right_flipper.curr_angle;
        let new_angle = (previous_angle + speed * delta_secs).clamp(-0.3, 0.3);
        let angular_speed = (new_angle - previous_angle) / delta_secs;

        position.0 = right_flipper.pivot_position;
        rotation.0 = right_flipper.base_rotation * Quat::from_rotation_z(new_angle);
        linear_velocity.0 = (position.0 - right_flipper.last_position) / delta_secs;
        angular_velocity.0 = rotation.0 * Vec3::Z * angular_speed;
        right_flipper.curr_angle = new_angle;
        right_flipper.angular_speed = angular_speed;
        right_flipper.last_position = position.0;
    }
}

fn kick_ball_from_active_flipper(
    mut collision_events: MessageReader<CollisionStart>,
    left_flippers: Query<&LeftFlipper>,
    right_flippers: Query<&RightFlipper>,
    mut balls: Query<(Entity, &mut LinearVelocity), With<Ball>>,
) {
    let board_rotation = Quat::from_rotation_x(0.12);

    for event in collision_events.read() {
        let (ball_entity, flipper_entity) = if balls.get(event.collider1).is_ok() {
            (event.collider1, event.collider2)
        } else if balls.get(event.collider2).is_ok() {
            (event.collider2, event.collider1)
        } else {
            continue;
        };

        let kick = if let Ok(flipper) = left_flippers.get(flipper_entity) {
            if flipper.angular_speed > 8.0 {
                Some(Vec3::new(0.30, 1.25, 0.05))
            } else {
                None
            }
        } else if let Ok(flipper) = right_flippers.get(flipper_entity) {
            if flipper.angular_speed < -8.0 {
                Some(Vec3::new(-0.30, 1.25, 0.05))
            } else {
                None
            }
        } else {
            None
        };

        if let Some(kick) = kick {
            if let Ok((_, mut velocity)) = balls.get_mut(ball_entity) {
                velocity.0 += board_rotation * kick;
            }
        }
    }
}

fn drive_ball_from_active_flippers(
    left_flippers: Query<&LeftFlipper>,
    right_flippers: Query<&RightFlipper>,
    mut balls: Query<(&Position, &mut LinearVelocity), With<Ball>>,
) {
    let board_rotation = Quat::from_rotation_x(0.12);

    for (ball_position, mut velocity) in &mut balls {
        let ball_local = board_rotation.inverse() * ball_position.0;
        let mut velocity_local = board_rotation.inverse() * velocity.0;
        let mut changed = false;

        for flipper in &left_flippers {
            if flipper.angular_speed <= 8.0 {
                continue;
            }

            let pivot_local = board_rotation.inverse() * flipper.pivot_position;
            let flipper_local =
                Quat::from_rotation_z(-flipper.curr_angle) * (ball_local - pivot_local);

            if ball_is_on_flipper(flipper_local) {
                velocity_local.x = velocity_local.x.max(0.55);
                velocity_local.y = velocity_local.y.max(1.25);
                velocity_local.z = velocity_local.z.max(0.04);
                changed = true;
            }
        }

        for flipper in &right_flippers {
            if flipper.angular_speed >= -8.0 {
                continue;
            }

            let pivot_local = board_rotation.inverse() * flipper.pivot_position;
            let flipper_local = Quat::from_rotation_z(-std::f32::consts::PI - flipper.curr_angle)
                * (ball_local - pivot_local);

            if ball_is_on_flipper(flipper_local) {
                velocity_local.x = velocity_local.x.min(-0.55);
                velocity_local.y = velocity_local.y.max(1.25);
                velocity_local.z = velocity_local.z.max(0.04);
                changed = true;
            }
        }

        if changed {
            velocity.0 = board_rotation * velocity_local;
        }
    }
}

fn ball_is_on_flipper(position: Vec3) -> bool {
    position.x > -0.025 && position.x < 0.085 && position.y > -0.035 && position.y < 0.025
}
