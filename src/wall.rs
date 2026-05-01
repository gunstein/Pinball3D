use avian3d::prelude::*;
use bevy::prelude::*;

use super::common;
use super::common::GameLayer;
use super::Pinball3DSystems;

pub struct WallPlugin;

#[derive(Component)]
pub struct Floor;

#[derive(Component)]
pub struct HalfHeight(pub f32);

#[derive(Component)]
pub struct BottomWall;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            spawn_walls
                .in_set(Pinball3DSystems::Walls)
                .after(Pinball3DSystems::Main),
        );
    }
}

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    fn spawn_wall_segment(commands: &mut Commands, start: Vec2, end: Vec2, thickness: f32) {
        let delta = end - start;
        let center = (start + end) * 0.5;
        let length = delta.length();
        let angle = delta.y.atan2(delta.x);

        commands.spawn((
            RigidBody::Static,
            Collider::cuboid(length, thickness, 0.1),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
            common::board_transform(Transform {
                translation: Vec3::new(center.x, center.y, 0.06),
                rotation: Quat::from_rotation_z(angle),
                ..default()
            }),
        ));
    }

    fn spawn_wall_joint(commands: &mut Commands, position: Vec2, thickness: f32) {
        commands.spawn((
            RigidBody::Static,
            Collider::cylinder(thickness * 0.5, 0.1),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
            common::board_transform(Transform {
                translation: Vec3::new(position.x, position.y, 0.06),
                rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                ..default()
            }),
        ));
    }

    let tree_texture_handle = asset_server.load("xmas_tree.png");
    let tree_aspect = 1.8;
    let tree_quad_width = 0.5;
    let tree_quad_handle = meshes.add(Mesh::from(Rectangle::from_size(Vec2::new(
        tree_quad_width,
        tree_quad_width * tree_aspect,
    ))));
    let tree_texture_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(tree_texture_handle),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let mxmas_texture_handle = asset_server.load("merry_xmas.png");
    let mxmas_quad_width = 0.15;
    let mxmas_quad_handle = meshes.add(Mesh::from(Rectangle::from_size(Vec2::new(
        mxmas_quad_width,
        mxmas_quad_width,
    ))));
    let mxmas_texture_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(mxmas_texture_handle),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let floor_half_height = 0.01;

    commands
        .spawn((
            Mesh3d(asset_server.load("floor.glb#Mesh0/Primitive0")),
            MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
            CollisionLayers::new(GameLayer::Floor, [GameLayer::Ball]),
            common::board_transform(Transform::default()),
            Floor,
            HalfHeight(floor_half_height),
        ))
        .with_children(|children| {
            children.spawn((
                Mesh3d(tree_quad_handle),
                MeshMaterial3d(tree_texture_material_handle),
                Transform::from_xyz(0.0, -0.3, 0.01),
            ));
            children.spawn((
                Mesh3d(mxmas_quad_handle),
                MeshMaterial3d(mxmas_texture_material_handle),
                Transform::from_xyz(0.0, -0.9, 0.01),
            ));
        });

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(0.8, 1.4, floor_half_height * 2.0),
        CollisionLayers::new(GameLayer::Floor, [GameLayer::Ball]),
        common::board_transform(Transform {
            translation: Vec3::new(0.0, -0.3, 0.0),
            ..default()
        }),
    ));

    commands.spawn((
        Mesh3d(asset_server.load("outer_wall.glb#Mesh0/Primitive0")),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        common::board_transform(Transform::default()),
    ));

    let wall_thickness = 0.025;
    spawn_wall_segment(
        &mut commands,
        Vec2::new(-0.37, -0.98),
        Vec2::new(-0.37, 0.004),
        wall_thickness,
    );
    spawn_wall_segment(
        &mut commands,
        Vec2::new(0.37, -0.98),
        Vec2::new(0.37, 0.004),
        wall_thickness,
    );

    let arc_center = Vec2::new(0.0, 0.004);
    let arc_radius = 0.37;
    let arc_start = std::f32::consts::PI;
    let arc_end = 0.0;
    let arc_segments = 24;
    let mut previous_arc_point = None;
    for index in 0..=arc_segments {
        let t = index as f32 / arc_segments as f32;
        let angle = arc_start + (arc_end - arc_start) * t;
        let point = arc_center + Vec2::new(angle.cos(), angle.sin()) * arc_radius;
        if index > 0 && index < arc_segments {
            spawn_wall_joint(&mut commands, point, wall_thickness);
        }
        if let Some(previous_point) = previous_arc_point {
            spawn_wall_segment(&mut commands, previous_point, point, wall_thickness);
        }
        previous_arc_point = Some(point);
    }

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(0.76, 0.02, 0.1),
        Sensor,
        Restitution::ZERO,
        Friction::new(0.8),
        CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
        CollisionEventsEnabled,
        common::board_transform(Transform::from_xyz(0.0, -1.0, 0.06)),
        BottomWall,
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(0.045, 0.02, 0.06),
        Restitution::ZERO,
        Friction::new(0.8),
        CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
        common::board_transform(Transform::from_xyz(0.355, -1.0, 0.04)),
    ));

    let material_flipper_wall = materials.add(Color::srgb(0.0, 1.0, 1.0));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.01 * 2.0, 0.14 * 2.0, 0.05 * 2.0)))),
        MeshMaterial3d(material_flipper_wall.clone()),
        RigidBody::Static,
        Collider::cuboid(0.02, 0.28, 0.1),
        CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
        common::board_transform(Transform {
            translation: Vec3::new(-0.24, -0.72, 0.06),
            rotation: Quat::from_rotation_z(1.1),
            ..default()
        }),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.01 * 2.0, 0.1 * 2.0, 0.05 * 2.0)))),
        MeshMaterial3d(material_flipper_wall),
        RigidBody::Static,
        Collider::cuboid(0.02, 0.2, 0.1),
        CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
        common::board_transform(Transform {
            translation: Vec3::new(0.2, -0.74, 0.06),
            rotation: Quat::from_rotation_z(-1.1),
            ..default()
        }),
    ));

    let launcher_wall_height = 0.05;
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.02, 0.56, launcher_wall_height)))),
        MeshMaterial3d(materials.add(Color::srgba(0.0, 1.0, 1.0, 0.5))),
        RigidBody::Static,
        Collider::cuboid(0.02, 0.56, launcher_wall_height),
        Restitution::ZERO,
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
        common::board_transform(Transform {
            translation: Vec3::new(0.315, -0.71, 0.01 + launcher_wall_height * 0.5),
            ..default()
        }),
    ));
}
