use avian3d::prelude::*;
use bevy::prelude::*;

use super::common;
use super::common::GameLayer;
use super::Floor;
use super::HalfHeight;

pub struct FlipperPlugin;

const FLIPPER_FORWARD_SPEED: f32 = 54.0;
const FLIPPER_RETURN_SPEED: f32 = 4.2;
const FLIPPER_MIN_ANGLE: f32 = -0.3;
const FLIPPER_MAX_ANGLE: f32 = 0.3;
const LEFT_FLIPPER_POSITION: Vec3 = Vec3::new(-0.1, -0.8, 0.01);
const RIGHT_FLIPPER_POSITION: Vec3 = Vec3::new(0.1, -0.8, 0.0);
const RIGHT_COLLIDER_OFFSET: Vec3 = Vec3::new(0.0, 0.006, 0.0);
const COLLIDER_SCALE: f32 = 0.81;
const COLLIDER_BASE_CENTER: Vec3 = Vec3::new(-0.004, -0.004, 0.0);
const COLLIDER_TIP_CENTER: Vec3 = Vec3::new(0.081, -0.004, 0.0);
const COLLIDER_BASE_RADIUS: f32 = 0.0175;
const COLLIDER_TIP_RADIUS: f32 = 0.0115;
const COLLIDER_SIDE_THICKNESS: f32 = 0.008;
const COLLIDER_SIDE_HEIGHT: f32 = 0.04;
const COLLIDER_SIDE_LENGTH_EXTRA: f32 = 0.006;

#[derive(Component)]
struct Flipper {
    side: FlipperSide,
    curr_angle: f32,
    base_rotation: Quat,
    pivot_position: Vec3,
    last_position: Vec3,
}

#[derive(Clone, Copy)]
enum FlipperSide {
    Left,
    Right,
}

impl FlipperSide {
    fn input_key(self) -> KeyCode {
        match self {
            Self::Left => KeyCode::ArrowLeft,
            Self::Right => KeyCode::ArrowRight,
        }
    }

    fn active_direction(self) -> f32 {
        match self {
            Self::Left => 1.0,
            Self::Right => -1.0,
        }
    }
}

impl Plugin for FlipperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_flippers)
            .add_systems(FixedUpdate, flipper_movement);
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

    let left_transform =
        common::board_transform(Transform::from_translation(LEFT_FLIPPER_POSITION));
    let right_transform = common::board_transform(Transform {
        translation: RIGHT_FLIPPER_POSITION.with_z(floor_half_height),
        rotation: Quat::from_rotation_z(-std::f32::consts::PI),
        ..default()
    });

    spawn_flipper(
        &mut commands,
        left_flipper_mesh_handle.clone(),
        material.clone(),
        left_transform,
        FlipperSide::Left,
        Vec3::ZERO,
    );
    spawn_flipper(
        &mut commands,
        left_flipper_mesh_handle,
        material,
        right_transform,
        FlipperSide::Right,
        RIGHT_COLLIDER_OFFSET,
    );
}

fn spawn_flipper(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
    side: FlipperSide,
    collider_offset: Vec3,
) {
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            RigidBody::Kinematic,
            SleepingDisabled,
            SweptCcd::NON_LINEAR,
            Friction::new(2.0).with_combine_rule(CoefficientCombine::Max),
            Restitution::new(0.25).with_combine_rule(CoefficientCombine::Average),
            flipper_collider(collider_offset),
            CollisionMargin(0.002),
            SpeculativeMargin(0.12),
            transform,
        ))
        .insert((
            LinearVelocity::ZERO,
            AngularVelocity::ZERO,
            CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
        ))
        .insert(Flipper {
            side,
            curr_angle: 0.0,
            base_rotation: transform.rotation,
            pivot_position: transform.translation,
            last_position: transform.translation,
        })
        .insert(DebugRender::collider(Color::srgb(1.0, 0.45, 0.0)));
}

fn flipper_collider(offset: Vec3) -> Collider {
    let vertical_cylinder = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let base_center = COLLIDER_BASE_CENTER * COLLIDER_SCALE + offset;
    let tip_center = COLLIDER_TIP_CENTER * COLLIDER_SCALE + offset;
    let base_radius = COLLIDER_BASE_RADIUS * COLLIDER_SCALE;
    let tip_radius = COLLIDER_TIP_RADIUS * COLLIDER_SCALE;
    let side_thickness = COLLIDER_SIDE_THICKNESS * COLLIDER_SCALE;
    let side_height = COLLIDER_SIDE_HEIGHT * COLLIDER_SCALE;
    let top_start = base_center.xy() + Vec2::Y * base_radius;
    let top_end = tip_center.xy() + Vec2::Y * tip_radius;
    let bottom_start = base_center.xy() - Vec2::Y * base_radius;
    let bottom_end = tip_center.xy() - Vec2::Y * tip_radius;
    let top_delta = top_end - top_start;
    let bottom_delta = bottom_end - bottom_start;
    let side_length = top_delta.length() + COLLIDER_SIDE_LENGTH_EXTRA * COLLIDER_SCALE;

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

fn flipper_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut flippers: Query<(
        &mut Flipper,
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

    for (mut flipper, mut position, mut rotation, mut linear_velocity, mut angular_velocity) in
        flippers.iter_mut()
    {
        let active_direction = flipper.side.active_direction();
        let speed = if keyboard_input.pressed(flipper.side.input_key()) {
            FLIPPER_FORWARD_SPEED * active_direction
        } else {
            -FLIPPER_RETURN_SPEED * active_direction
        };
        let previous_angle = flipper.curr_angle;
        let new_angle =
            (previous_angle + speed * delta_secs).clamp(FLIPPER_MIN_ANGLE, FLIPPER_MAX_ANGLE);
        let angular_speed = (new_angle - previous_angle) / delta_secs;

        position.0 = flipper.pivot_position;
        rotation.0 = flipper.base_rotation * Quat::from_rotation_z(new_angle);
        linear_velocity.0 = (position.0 - flipper.last_position) / delta_secs;
        angular_velocity.0 = rotation.0 * Vec3::Z * angular_speed;
        flipper.curr_angle = new_angle;
        flipper.last_position = position.0;
    }
}
