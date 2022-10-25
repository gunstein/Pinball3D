use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WallsPlugin;
use bevy_rapier3d::rapier::na::Vector3;

//#[derive(Default)]
//struct HalfCircleWallInstance(Option<Handle<Mesh>>);

//#[derive(Default)]
//struct FloorInstance(Option<Handle<Mesh>>);

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app
            //.init_resource::<HalfCircleWallInstance>()
            //.init_resource::<FloorInstance>()
            //.add_startup_system(load_wall_meshes.after("main_setup").label("walls"));
            .add_startup_system(spawn_walls.after("main_setup").label("walls"));
            //.add_system(scene_update_add_walls);
    }
}

/* 
fn load_wall_meshes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut half_circle_wall_instance: ResMut<HalfCircleWallInstance>,
    mut floor_instance: ResMut<FloorInstance>,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //walls_instance.0 = Some(asset_server.load("pinball3d_test.glb#Mesh0/Primitive0"));
    //walls_instance.0 = Some(asset_server.load("gvtest1.glb#Mesh0/Primitive0"));
    half_circle_wall_instance.0 = Some(asset_server.load("half_circle_wall.glb#Mesh0/Primitive0"));
    floor_instance.0 = Some(meshes.add(Mesh::from(shape::Cube { size: 1.0 })));

}
*/

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let half_circle_wall_mesh_handle:Handle<Mesh> = asset_server.load("half_circle_wall.glb#Mesh0/Primitive0");
    //let floor_instance_mesh_handle:Handle<Mesh> = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let floor_instance_mesh_handle:Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(0.4*2.0,1.5*2.0, 0.01*2.0)));

    let material = materials.add(Color::rgb(2.0, 0.9, 2.0).into());

    let wall_position = Vec3::new(0.0, 0.0, 0.0);
                
    commands.spawn()
    /*.insert_bundle(PbrBundle {
        mesh: floor_instance_mesh_handle.clone(),
        material: material.clone(),
        ..default()
    })*/
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: half_circle_wall_mesh_handle.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        });
    })
    /* .insert_bundle(PbrBundle {
        mesh: floor_instance_mesh_handle.clone(),
        material: material.clone(),
        ..default()
    })*/
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
    .insert_bundle(TransformBundle::from(Transform::from_xyz(wall_position.x, wall_position.y, wall_position.z)));

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