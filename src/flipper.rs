use avian3d::prelude::*;
use bevy::prelude::*;

use super::common::GameLayer;
use super::common;
use super::Floor;
use super::HalfHeight;

pub struct FlipperPlugin;

#[derive(Component)]
struct LeftFlipper {
    curr_angle: f32,
    base_rotation: Quat,
}

#[derive(Component)]
struct RightFlipper {
    curr_angle: f32,
    base_rotation: Quat,
}

impl Plugin for FlipperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_flippers)
            .add_systems(Update, (left_flipper_movement, right_flipper_movement));
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
    let flipper_half_height = 0.05;

    let collider_small_cylinder = Collider::cylinder(0.007, flipper_half_height * 2.0);
    let position_small_cylinder = Vec3::new(0.07, 0.0, flipper_half_height + floor_half_height);
    let rotation_small_cylinder = Quat::from_rotation_x(std::f32::consts::PI / 2.0);

    let collider_upper_box = Collider::cuboid(0.038, 0.007, flipper_half_height);
    let position_upper_box = Vec3::new(0.033, 0.006, flipper_half_height + floor_half_height);
    let rotation_upper_box = Quat::from_rotation_z(-0.12);

    let collider_lower_box = collider_upper_box.clone();
    let position_lower_box = Vec3::new(0.033, -0.006, flipper_half_height + floor_half_height);
    let rotation_lower_box = Quat::from_rotation_z(0.12);

    let left_transform = common::board_transform(Transform::from_xyz(
        left_flipper_position.x,
        left_flipper_position.y,
        left_flipper_position.z,
    ));

    commands.spawn((
        Mesh3d(left_flipper_mesh_handle.clone()),
        MeshMaterial3d(material.clone()),
        RigidBody::Kinematic,
        SleepingDisabled,
        SweptCcd::default(),
        Friction::new(0.7).with_combine_rule(CoefficientCombine::Min),
        Collider::compound(vec![
            (position_small_cylinder, rotation_small_cylinder, collider_small_cylinder.clone()),
            (position_upper_box, rotation_upper_box, collider_upper_box.clone()),
        ]),
        CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
        left_transform,
        LeftFlipper {
            curr_angle: 0.0,
            base_rotation: left_transform.rotation,
        },
    ));

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
        SweptCcd::default(),
        Friction::new(0.7).with_combine_rule(CoefficientCombine::Min),
        Collider::compound(vec![
            (position_small_cylinder, rotation_small_cylinder, collider_small_cylinder.clone()),
            (position_lower_box, rotation_lower_box, collider_lower_box.clone()),
        ]),
        CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
        right_transform,
        RightFlipper {
            curr_angle: 0.0,
            base_rotation: right_transform.rotation,
        },
    ));
}

fn left_flipper_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut left_flippers: Query<(&mut LeftFlipper, &mut Transform)>,
) {
    for (mut left_flipper, mut left_flipper_transform) in left_flippers.iter_mut() {
        let mut new_angle = left_flipper.curr_angle;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            new_angle += 0.3;
        } else {
            new_angle -= 0.07;
        }

        let new_clamped_angle = new_angle.clamp(-0.3, 0.3);
        left_flipper_transform.rotation =
            left_flipper.base_rotation * Quat::from_rotation_z(new_clamped_angle);
        left_flipper.curr_angle = new_clamped_angle;
    }
}

fn right_flipper_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut right_flippers: Query<(&mut RightFlipper, &mut Transform)>,
) {
    for (mut right_flipper, mut right_flipper_transform) in right_flippers.iter_mut() {
        let mut new_angle = right_flipper.curr_angle;

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            new_angle -= 0.3;
        } else {
            new_angle += 0.07;
        }

        let new_clamped_angle = new_angle.clamp(-0.3, 0.3);
        right_flipper_transform.rotation =
            right_flipper.base_rotation * Quat::from_rotation_z(new_clamped_angle);
        right_flipper.curr_angle = new_clamped_angle;
    }
}
