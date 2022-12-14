use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
        app.add_startup_system(spawn_walls.after(Pinball3DSystems::Main));
    }
}

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //Floor
    //Tree in floor background
    let tree_texture_handle = asset_server.load("xmas_tree.png");
    let tree_aspect = 1.8;

    let tree_quad_width = 0.5;
    let tree_quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        tree_quad_width,
        tree_quad_width * tree_aspect,
    ))));

    let tree_texture_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(tree_texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    //Merry christmas in floor background
    let mxmas_texture_handle = asset_server.load("merry_xmas.png");
    let mxmas_aspect = 1.0;

    let mxmas_quad_width = 0.15;
    let mxmas_quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        mxmas_quad_width,
        mxmas_quad_width * mxmas_aspect,
    ))));

    let mxmas_texture_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(mxmas_texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let floor_handle: Handle<Mesh> = asset_server.load("floor.glb#Mesh0/Primitive0");
    let floor_position = Vec3::new(0.0, -0.0, 0.0);
    let material_floor = materials.add(Color::rgb(0.0, 0.0, 1.0).into());
    let floor_half_height = 0.01;

    let floor = commands
        .spawn(PbrBundle {
            mesh: floor_handle.clone(),
            material: material_floor.clone(),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(0.4, 0.7, floor_half_height))
                .insert(TransformBundle::from(Transform {
                    translation: Vec3::new(0.0, -0.3, 0.0),
                    ..default()
                }));
            children.spawn(PbrBundle {
                mesh: tree_quad_handle.clone(),
                material: tree_texture_material_handle.clone(),
                transform: Transform::from_xyz(0.0, -0.3, 0.01),
                ..default()
            });
            children.spawn(PbrBundle {
                mesh: mxmas_quad_handle.clone(),
                material: mxmas_texture_material_handle.clone(),
                transform: Transform::from_xyz(0.0, -0.9, 0.01),
                ..default()
            });
        })
        .insert(CollisionGroups {
            memberships: Group::GROUP_1,
            filters: Group::GROUP_3,
        })
        .insert(TransformBundle::from(Transform {
            translation: Vec3::new(floor_position.x, floor_position.y, floor_position.z),
            rotation: Quat::from_rotation_x(0.12),
            ..default()
        }))
        .insert(Floor)
        .insert(HalfHeight(floor_half_height))
        .id();

    //Outer wall
    let outer_wall_handle: Handle<Mesh> = asset_server.load("outer_wall.glb#Mesh0/Primitive0");
    let outer_wall_position = Vec3::new(0.0, 0.0, 0.0);
    let material_outer_wall = materials.add(Color::rgb(0.0, 1.0, 0.0).into());

    //Build heights vector for half circle collider
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

    let outer_wall = commands
        .spawn(PbrBundle {
            mesh: outer_wall_handle.clone(),
            material: material_outer_wall.clone(),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .with_children(|children| {
            //Collider half circle wall
            children
                .spawn(Collider::heightfield(
                    heights,
                    2,
                    num_cols,
                    Vec3::new(0.72, 1.0, 0.1),
                ))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.01, 0.05)));

            //Collider left wall
            let left_wall_position = Vec3::new(-0.37, -0.51, 0.06);
            children
                .spawn(Collider::cuboid(0.01, 0.5, 0.05))
                .insert(TransformBundle::from(Transform::from_xyz(
                    left_wall_position.x,
                    left_wall_position.y,
                    left_wall_position.z,
                )));

            //Collider right wall
            let right_wall_position = Vec3::new(0.37, -0.51, 0.06);
            children
                .spawn(Collider::cuboid(0.01, 0.5, 0.05))
                .insert(TransformBundle::from(Transform::from_xyz(
                    right_wall_position.x,
                    right_wall_position.y,
                    right_wall_position.z,
                )));

            //Collider sensor bottom wall
            let bottom_wall_position = Vec3::new(0.0, -1.0, 0.06);
            children
                .spawn(Collider::cuboid(0.38, 0.01, 0.05))
                .insert(Sensor)
                .insert(TransformBundle::from(Transform::from_xyz(
                    bottom_wall_position.x,
                    bottom_wall_position.y,
                    bottom_wall_position.z,
                )))
                .insert(BottomWall);
        })
        .insert(CollisionGroups {
            memberships: Group::GROUP_2,
            filters: Group::GROUP_3,
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            outer_wall_position.x,
            outer_wall_position.y,
            outer_wall_position.z,
        )))
        .id();

    //Left flipper wall
    let left_flipper_wall_mesh_handle: Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(
        0.01 * 2.0,
        0.14 * 2.0,
        0.05 * 2.0,
    )));
    let left_flipper_wall_position = Vec3::new(-0.24, -0.72, 0.06);
    let material_flipper_wall = materials.add(Color::CYAN.into());

    let left_flipper_wall = commands
        .spawn(PbrBundle {
            mesh: left_flipper_wall_mesh_handle.clone(),
            material: material_flipper_wall.clone(),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.01, 0.14, 0.05))
        .insert(CollisionGroups {
            memberships: Group::GROUP_2,
            filters: Group::GROUP_3,
        })
        .insert(TransformBundle::from(Transform {
            translation: Vec3::new(
                left_flipper_wall_position.x,
                left_flipper_wall_position.y,
                left_flipper_wall_position.z,
            ),
            rotation: Quat::from_rotation_z(1.1),
            ..default()
        }))
        .id();

    //Right flipper wall
    let right_flipper_wall_mesh_handle: Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(
        0.01 * 2.0,
        0.1 * 2.0,
        0.05 * 2.0,
    )));
    let right_flipper_wall_position = Vec3::new(0.2, -0.74, 0.06);

    let right_flipper_wall = commands
        .spawn(PbrBundle {
            mesh: right_flipper_wall_mesh_handle.clone(),
            material: material_flipper_wall.clone(),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.01, 0.1, 0.05))
        .insert(CollisionGroups {
            memberships: Group::GROUP_2,
            filters: Group::GROUP_3,
        })
        .insert(TransformBundle::from(Transform {
            translation: Vec3::new(
                right_flipper_wall_position.x,
                right_flipper_wall_position.y,
                right_flipper_wall_position.z,
            ),
            rotation: Quat::from_rotation_z(-1.1),
            ..default()
        }))
        .id();

    //Launcher wall
    let launcher_wall_mesh_handle: Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(
        0.01 * 2.0,
        0.28 * 2.0,
        0.05 * 2.0,
    )));
    let launcher_wall_position = Vec3::new(0.3, -0.71, 0.06);
    let material_launcher_wall = materials.add(Color::rgba(0.0, 1.0, 1.0, 0.5).into()); //Cyan

    let launcher_wall = commands
        .spawn(PbrBundle {
            mesh: launcher_wall_mesh_handle.clone(),
            material: material_launcher_wall.clone(),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .with_children(|children| {
            children.spawn(Collider::cuboid(0.01, 0.28, 0.05));

            //small cylinder on top of wall to avoid ball getting stuck.
            children
                .spawn(Collider::cylinder(0.05, 0.01))
                .insert(TransformBundle::from(Transform {
                    translation: Vec3::new(0.0, 0.28, 0.0),
                    rotation: Quat::from_rotation_x(std::f32::consts::PI / 2.0),
                    ..default()
                }));
        })
        .insert(CollisionGroups {
            memberships: Group::GROUP_2,
            filters: Group::GROUP_3,
        })
        .insert(TransformBundle::from(Transform {
            translation: Vec3::new(
                launcher_wall_position.x,
                launcher_wall_position.y,
                launcher_wall_position.z,
            ),
            ..default()
        }))
        .id();

    //Add all walls as children to floor
    commands.entity(floor).push_children(&[
        outer_wall,
        left_flipper_wall,
        right_flipper_wall,
        launcher_wall,
    ]);
}
