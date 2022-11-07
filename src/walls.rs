use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::Pinball3DSystems;

pub struct WallsPlugin;

#[derive(Component)]
pub struct Floor;

#[derive(Component)]
pub struct BottomWall;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_walls.after(Pinball3DSystems::Main));
    }
}

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //let material = materials.add(Color::rgb(2.0, 0.9, 2.0).into());

    //Floor
    let floor_handle:Handle<Mesh> = asset_server.load("floor.glb#Mesh0/Primitive0");
    let floor_position = Vec3::new(0.0, 0.0, 0.0);
    let material_floor = materials.add(Color::rgb(0.0, 0.0, 1.0).into());

    let floor = commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: floor_handle.clone(),
        material: material_floor.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .with_children(|children| {
        children.spawn()
        .insert(Collider::cuboid(0.4, 0.7, 0.01))
        .insert_bundle(TransformBundle::from(
            Transform{
                translation: Vec3::new(0.0, -0.3, 0.0),
                ..default()
            })
        );
    })
    .insert(CollisionGroups{memberships:Group::GROUP_1, filters:Group::GROUP_3})
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(floor_position.x, floor_position.y, floor_position.z),
            rotation: Quat::from_rotation_x(0.12),
            //rotation: Quat::from_rotation_x(3.9),
            ..default()
        }
    ))
    .insert(Floor)
    .id();

    //Outer wall
    let outer_wall_handle:Handle<Mesh> = asset_server.load("outer_wall.glb#Mesh0/Primitive0");
    let outer_wall_position = Vec3::new(0.0, 0.0, 0.0);
    let material_outer_wall = materials.add(Color::rgb(0.0, 1.0, 0.0).into());

    //Build heights vector for half circle collider
    let mut heights = Vec::new();
    let radius : f32 = 0.36;
    let radius_squared: f32 = radius * radius;
    let num_cols = 21;
    let step_size = (radius * 2.0) / (num_cols as f32 -1.0);
    for step in 0..num_cols{
        let x = -radius + (step as f32 * step_size);
        let y = f32::sqrt(radius_squared - (x*x));
        heights.push(y);
        heights.push(y);
    }

    let outer_wall = commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: outer_wall_handle.clone(),
        material: material_outer_wall.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .with_children(|children| {
        //Collider half circle wall
        children.spawn()
        .insert(Collider::heightfield(heights, 2, num_cols, Vec3::new(0.72, 1.0, 0.1)))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -0.01, 0.05)));
        
        //Collider left wall
        let left_wall_position = Vec3::new(-0.371, -0.51, 0.06);
        children.spawn()
        .insert(Collider::cuboid(0.01,0.5, 0.05))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(left_wall_position.x, left_wall_position.y, left_wall_position.z)));
        
        //Collider right wall
        let right_wall_position = Vec3::new(0.371, -0.51, 0.06);
        children.spawn()
        .insert(Collider::cuboid(0.01,0.5, 0.05))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(right_wall_position.x, right_wall_position.y, right_wall_position.z)));

        //Collider sensor bottom wall
        let bottom_wall_position = Vec3::new(0.0, -1.0, 0.06);
        children.spawn()
        .insert(Collider::cuboid(0.38, 0.01, 0.05))
        .insert(Sensor)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(bottom_wall_position.x, bottom_wall_position.y, bottom_wall_position.z)))
        .insert(BottomWall); 
    })    
    .insert(CollisionGroups{memberships:Group::GROUP_2, filters:Group::GROUP_3})
    .insert_bundle(TransformBundle::from(Transform::from_xyz(outer_wall_position.x, outer_wall_position.y, outer_wall_position.z)))
    .id();

    //Left flipper wall
    let left_flipper_wall_mesh_handle:Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(0.01*2.0,0.14*2.0, 0.05*2.0)));
    let left_flipper_wall_position = Vec3::new(-0.24, -0.72, 0.06);
    let material_flipper_wall = materials.add(Color::CYAN.into());
    
    let left_flipper_wall = commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: left_flipper_wall_mesh_handle.clone(),
        material: material_flipper_wall.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.01,0.14, 0.05))
    .insert(CollisionGroups{memberships:Group::GROUP_2, filters:Group::GROUP_3})
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(left_flipper_wall_position.x, left_flipper_wall_position.y, left_flipper_wall_position.z),
            rotation: Quat::from_rotation_z(1.1),
            ..default()
        }
    ))
    .id();
    
    //Right flipper wall
    let right_flipper_wall_mesh_handle:Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(0.01*2.0,0.16*2.0, 0.05*2.0)));
    let right_flipper_wall_position = Vec3::new(0.24, -0.69, 0.06);
    
    let right_flipper_wall = commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: right_flipper_wall_mesh_handle.clone(),
        material: material_flipper_wall.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.01,0.16, 0.05))
    .insert(CollisionGroups{memberships:Group::GROUP_2, filters:Group::GROUP_3})
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(right_flipper_wall_position.x, right_flipper_wall_position.y, right_flipper_wall_position.z),
            rotation: Quat::from_rotation_z(-0.92),
            ..default()
        }
    ))
    .id();
    
    //Add all walls as children to floor
    commands.entity(floor).push_children(&[outer_wall, left_flipper_wall, right_flipper_wall]);
    //commands.entity(floor).push_children(&[outer_wall]);
    
}