use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{common, Pinball3DSystems};

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
            CollisionGroups {
                memberships: Group::GROUP_1,
                filters: Group::GROUP_3,
            },
            Transform {
                rotation: Quat::from_rotation_x(0.12),
                ..default()
            },
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
        RigidBody::Fixed,
        Collider::cuboid(0.4, 0.7, floor_half_height),
        CollisionGroups {
            memberships: Group::GROUP_1,
            filters: Group::GROUP_3,
        },
        common::board_transform(Transform {
            translation: Vec3::new(0.0, -0.3, 0.0),
            ..default()
        }),
    ));

    //Outer wall
    let mut heights = Vec::new();
    let radius: f32 = 0.36;
    let radius_squared: f32 = radius * radius;
    let num_cols = 21;
    let step_size = (radius * 2.0) / (num_cols as f32 - 1.0);
    for step in 0..num_cols {
        let x = -radius + (step as f32 * step_size);
        let y = f32::sqrt(radius_squared - (x * x));
        heights.push(y);
        heights.push(y);
    }

    commands
        .spawn((
            Mesh3d(asset_server.load("outer_wall.glb#Mesh0/Primitive0")),
            MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
            RigidBody::Fixed,
            CollisionGroups {
                memberships: Group::GROUP_2,
                filters: Group::GROUP_3,
            },
            common::board_transform(Transform::default()),
        ))
        .with_children(|children| {
            children.spawn((
                Collider::heightfield(heights, 2, num_cols, Vec3::new(0.72, 1.0, 0.1)),
                Transform::from_xyz(0.0, -0.01, 0.05),
            ));
            children.spawn((
                Collider::cuboid(0.01, 0.5, 0.05),
                Transform::from_xyz(-0.37, -0.51, 0.06),
            ));
            children.spawn((
                Collider::cuboid(0.01, 0.5, 0.05),
                Transform::from_xyz(0.37, -0.51, 0.06),
            ));
            children.spawn((
                Collider::cuboid(0.38, 0.01, 0.05),
                Sensor,
                Transform::from_xyz(0.0, -1.0, 0.06),
                BottomWall,
            ));
        });

    //Left flipper wall
    let material_flipper_wall = materials.add(Color::srgb(0.0, 1.0, 1.0));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.01 * 2.0, 0.14 * 2.0, 0.05 * 2.0)))),
        MeshMaterial3d(material_flipper_wall.clone()),
        RigidBody::Fixed,
        Collider::cuboid(0.01, 0.14, 0.05),
        CollisionGroups {
            memberships: Group::GROUP_2,
            filters: Group::GROUP_3,
        },
        common::board_transform(Transform {
            translation: Vec3::new(-0.24, -0.72, 0.06),
            rotation: Quat::from_rotation_z(1.1),
            ..default()
        }),
    ));

    //Right flipper wall
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.01 * 2.0, 0.1 * 2.0, 0.05 * 2.0)))),
        MeshMaterial3d(material_flipper_wall),
        RigidBody::Fixed,
        Collider::cuboid(0.01, 0.1, 0.05),
        CollisionGroups {
            memberships: Group::GROUP_2,
            filters: Group::GROUP_3,
        },
        common::board_transform(Transform {
            translation: Vec3::new(0.2, -0.74, 0.06),
            rotation: Quat::from_rotation_z(-1.1),
            ..default()
        }),
    ));

    //Launcher wall
    commands
        .spawn((
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.01 * 2.0, 0.28 * 2.0, 0.05 * 2.0)))),
            MeshMaterial3d(materials.add(Color::srgba(0.0, 1.0, 1.0, 0.5))),
            RigidBody::Fixed,
            CollisionGroups {
                memberships: Group::GROUP_2,
                filters: Group::GROUP_3,
            },
            common::board_transform(Transform {
                translation: Vec3::new(0.3, -0.71, 0.06),
                ..default()
            }),
        ))
        .with_children(|children| {
            children.spawn(Collider::cuboid(0.01, 0.28, 0.05));
            children.spawn((
                Collider::cylinder(0.05, 0.01),
                Transform {
                    translation: Vec3::new(0.0, 0.28, 0.0),
                    rotation: Quat::from_rotation_x(std::f32::consts::PI / 2.0),
                    ..default()
                },
            ));
        });
}
