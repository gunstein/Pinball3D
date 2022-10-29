use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WallsPlugin;
use bevy_rapier3d::rapier::na::Vector3;


impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_walls.after("main_setup").label("walls"));
    }
}

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(Color::rgb(2.0, 0.9, 2.0).into());

    /* 
    commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: floor_instance_mesh_handle.clone(),
        material: material.clone(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: half_circle_wall_mesh_handle.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        });
    })
    .insert(RigidBody::Fixed)
    .with_children(|children| {
        children.spawn()
        //.insert(AsyncCollider{}::from_bevy_mesh(half_circle_wall_mesh_handle, &ComputedColliderShape::TriMesh).unwrap())
        .insert(AsyncCollider{handle: half_circle_wall_mesh_handle, shape: ComputedColliderShape::TriMesh})
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.3, 0.0)));
        //children.spawn()
        //.insert(Collider::heightfield(vec!(0.1, 0.1, 0.1), 1, 3, Vector3::new(1.0, 1.0, 1.0)));
        children.spawn()
        .insert(Collider::cuboid(0.4, 0.7, 0.01));
    })
    //.insert(Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap())
    //.insert(Collider::cuboid(0.1, 0.1, 0.01))
    .insert(CollisionGroups::new(0b0001, 0b0100))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(wall_position.x, wall_position.y, wall_position.z)));
    */

    /* Not working, too complex mesh for decomposition, it seems.
    let outer_wall_and_floor_mesh_handle:Handle<Mesh> = asset_server.load("outer_wall_and_floor.glb#Mesh0/Primitive0");
    commands.spawn() 
    .insert_bundle(PbrBundle {
        mesh: outer_wall_and_floor_mesh_handle.clone(),
        material: material.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(AsyncCollider{handle: outer_wall_and_floor_mesh_handle, shape: ComputedColliderShape::ConvexDecomposition(VHACDParameters::default())})
    .insert(CollisionGroups::new(0b0010, 0b0100))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
    */


    let floor_position = Vec3::new(0.0, 0.0, 0.0);
    let floor_mesh_handle:Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(0.4*2.0,0.7*2.0, 0.01*2.0)));

    commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: floor_mesh_handle.clone(),
        material: material.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.4, 0.7, 0.01))
    .insert(CollisionGroups::new(0b0001, 0b0100))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(floor_position.x, floor_position.y, floor_position.z)));


    let leftright_wall_mesh_handle:Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(0.01*2.0,0.5*2.0, 0.05*2.0)));
    let left_wall_position = Vec3::new(-0.39, -0.2, 0.06);
    
    commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: leftright_wall_mesh_handle.clone(),
        material: material.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.01,0.5, 0.05))
    .insert(CollisionGroups::new(0b0001, 0b0100))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(left_wall_position.x, left_wall_position.y, left_wall_position.z)));    
    
    let right_wall_position = Vec3::new(0.39, -0.2, 0.06);
 
    commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: leftright_wall_mesh_handle.clone(),
        material: material.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.01,0.5, 0.05))
    .insert(CollisionGroups::new(0b0001, 0b0100))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(right_wall_position.x, right_wall_position.y, right_wall_position.z)));    
    
    let half_circle_wall_mesh_handle:Handle<Mesh> = asset_server.load("half_circle_wall.glb#Mesh0/Primitive0");

    let half_circle_wall_position = Vec3::new(0.0, 0.3, 0.01);
    //let half_circle_wall_position = Vec3::new(0.0, 0.0, 0.0);


    //Build heights vector
    let mut heights = Vec::new();
    let radius : f32 = 0.38;
    let radius_squared: f32 = radius * radius;
    let num_cols = 21;
    let step_size = (radius * 2.0) / (num_cols as f32 -1.0);
    for step in 0..num_cols{
        let x = -radius + (step as f32 * step_size);
        let y = f32::sqrt(radius_squared - (x*x));
        heights.push(y);
        heights.push(y);
    }

    println!("heights {:?}", heights);

    commands.spawn() 
    .insert_bundle(PbrBundle {
        mesh: half_circle_wall_mesh_handle.clone(),
        material: material.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    //.insert(AsyncCollider{handle: half_circle_wall_mesh_handle, shape: ComputedColliderShape::TriMesh})
    //.insert(AsyncCollider{handle: half_circle_wall_mesh_handle, shape: ComputedColliderShape::ConvexDecomposition(VHACDParameters::default())})
    //.insert(Collider::heightfield(vec!(0.1, 0.1, 0.2, 0.2, 0.4, 0.4, 0.2, 0.2, 0.1, 0.1), 2, 5, Vector3::new(0.8, 1.0, 0.1)))
    .with_children(|children| {
        children.spawn()
        .insert(Collider::heightfield(heights, 2, num_cols, Vector3::new(0.76, 1.0, 0.1)))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -0.01, 0.05)));
    })
    .insert(CollisionGroups::new(0b0010, 0b0100))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(half_circle_wall_position.x, half_circle_wall_position.y, half_circle_wall_position.z)));    
    
}

/* 
fn scene_update_add_walls(
    mut commands: Commands,
    //half_circle_wall_instance: Res<HalfCircleWallInstance>,
    //floor_instance: Res<FloorInstance>,
    meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Some(half_circle_wall_mesh_handle) = &half_circle_wall_instance.0 {
            if let Some(mesh) = meshes.get(half_circle_wall_mesh_handle){
                let material = materials.add(Color::rgb(2.0, 0.9, 2.0).into());

                //let collider_shape = crate::utils::mesh_to_convexdecomp_collider_shape(mesh);

                let wall_position = Vec3::new(0.0, 0.0, 0.0);
                
                commands.spawn()
                
                .with_children(|children|{
                    children.spawn()
                    .insert_bundle(PbrBundle {
                        mesh: mesh_handle.clone(),
                        material: material.clone(),
                        ..default()
                    });
                    
                    children.spawn()
                    .insert_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                        ..default()
                    });
                                     
                })
                
                
                .insert_bundle(PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material.clone(),
                    ..Default::default()
                })
                .insert_bundle(PbrBundle {
                    //mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    ..default()
                })
                .insert(RigidBody::Fixed)
                .with_children(|children| {
                    children.spawn()
                    .insert(Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap())
                    .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)));
                    //children.spawn()
                    //.insert(Collider::heightfield(vec!(0.1, 0.1, 0.1), 1, 3, Vector3::new(1.0, 1.0, 1.0)));
                    children.spawn()
                    .insert(Collider::cuboid(0.4, 1.5, 0.01));
                })
                //.insert(Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap())
                //.insert(Collider::cuboid(0.1, 0.1, 0.01))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(wall_position.x, wall_position.y, wall_position.z)));
                //.insert(CollisionGroups::new(0b0001, 0b0010));
                //.insert(Transform::from_xyz(wall_position.x, wall_position.y, wall_position.z)); 

                
                //println!("Rail added!!! {:?}", mesh );

                *done = true;
            }
        }
    }
}
*/