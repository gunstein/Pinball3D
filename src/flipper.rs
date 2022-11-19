use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

//use super::Pinball3DSystems;
use super::Floor;
use super::HalfHeight;

pub struct FlipperPlugin;

#[derive(Component)]
struct LeftFlipper{
    curr_angle : f32,
 }

 #[derive(Component)]
 struct RightFlipper{
     curr_angle : f32,
  } 

impl Plugin for FlipperPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_startup_system(spawn_flippers.after(Pinball3DSystems::Walls).label(Pinball3DSystems::Flippers))
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_flippers)
            .add_system(left_flipper_movement)
            .add_system(right_flipper_movement);
    }
}

fn spawn_flippers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<(Entity, &HalfHeight), With<Floor>>
) {
    //info!("Spawn flippers start");
    let mut floor = None;
    let mut floor_half_height = 0.0;
    //for floor in query_floors.iter(){
    for (entity, half_height) in query_floors.iter(){
        //info!("Spawn flippers, floor is found.");
        //floor_entity = Some(floor);
        floor = Some(entity);
        floor_half_height = half_height.0;
    }
    let left_flipper_mesh_handle:Handle<Mesh> = asset_server.load("left_flipper.glb#Mesh0/Primitive0");
  
    let material = materials.add(Color::YELLOW.into());

    let left_flipper_position = Vec3::new(-0.1, -0.8, 0.01);
    let right_flipper_position = Vec3::new(0.1, -0.8, floor_half_height);
    let flipper_half_height = 0.05;

    //let collider_big_cylinder = Collider::round_cylinder(flipper_half_height, 0.016, 0.001);
    //let position_big_cylinder = Vec3::new(0.0, 0.0, flipper_half_height);
    //let rotation_big_cylinder = Quat::from_rotation_x(std::f32::consts::PI/2.0);

    let collider_small_cylinder = Collider::round_cylinder(flipper_half_height, 0.007, 0.002);
    let position_small_cylinder = Vec3::new(0.07, 0.0, flipper_half_height + floor_half_height);
    let rotation_small_cylinder = Quat::from_rotation_x(std::f32::consts::PI/2.0);

    //let collider_upper_box = Collider::round_cuboid(0.035, 0.004, flipper_half_height, 0.002);
    let collider_upper_box = Collider::cuboid(0.038, 0.007, flipper_half_height);
    let position_upper_box = Vec3::new(0.033, 0.006, flipper_half_height + floor_half_height);
    let rotation_upper_box = Quat::from_rotation_z(-0.12);
    
    let collider_lower_box = collider_upper_box.clone();
    let position_lower_box = Vec3::new(0.033, -0.006, flipper_half_height + floor_half_height);
    let rotation_lower_box = Quat::from_rotation_z(0.12); 

    let left_flipper = commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: left_flipper_mesh_handle.clone(),
        material: material.clone(),
        ..default()
    })
    .insert(RigidBody::KinematicPositionBased)
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    .insert(Collider::compound(vec![
        //(position_big_cylinder, rotation_big_cylinder, collider_big_cylinder.clone()),
        (position_small_cylinder, rotation_small_cylinder, collider_small_cylinder.clone()),
        (position_upper_box, rotation_upper_box, collider_upper_box.clone()),
        //(position_lower_box, rotation_lower_box, collider_lower_box.clone())
    ]))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(left_flipper_position.x, left_flipper_position.y, left_flipper_position.z)))
    .insert(LeftFlipper{curr_angle:0.0})
    .id(); 

    commands.entity(floor.unwrap()).add_child(left_flipper);

    let right_flipper = commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: left_flipper_mesh_handle.clone(),
        material: material.clone(),
        ..default()
    })
    .insert(RigidBody::KinematicPositionBased)
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    //.insert(AsyncCollider{handle: left_flipper_mesh_handle, shape: ComputedColliderShape::TriMesh})
    .insert(Collider::compound(vec![
        //(position_big_cylinder, rotation_big_cylinder, collider_big_cylinder.clone()),
        (position_small_cylinder, rotation_small_cylinder, collider_small_cylinder.clone()),
        //(position_upper_box, rotation_upper_box, collider_upper_box.clone()),
        (position_lower_box, rotation_lower_box, collider_lower_box.clone())
    ]))
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(right_flipper_position.x, right_flipper_position.y, right_flipper_position.z),
            rotation: Quat::from_rotation_z(-std::f32::consts::PI),
            ..default()
        }
    ))
    .insert(RightFlipper{curr_angle:0.0})
    .id();
    
    commands.entity(floor.unwrap()).add_child(right_flipper);
}


fn left_flipper_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut left_flippers: Query<(&mut LeftFlipper, &mut Transform), With<LeftFlipper>>,
) {
    //info!("test1");
    for (mut left_flipper, mut left_flipper_transform) in left_flippers.iter_mut() {
        //info!("test2");
        let mut new_angle = left_flipper.curr_angle;
        let change_angle:f32;

        if keyboard_input.pressed(KeyCode::Left)
        {
            //change_angle = 0.09;
            change_angle = 0.3;
        }
        else
        {
            change_angle = -0.07;
        }

        new_angle += change_angle;
        let new_clamped_angle = new_angle.clamp(-0.3, 0.3);
        let pivot_rotation = Quat::from_rotation_z(new_clamped_angle - left_flipper.curr_angle);
        //left_flipper_transform.rotate_around(left_flipper.point_of_rotation, pivot_rotation);
        //println!("pivot_rotation {:?}", pivot_rotation );
        left_flipper_transform.rotate(pivot_rotation);   
        left_flipper.curr_angle = new_clamped_angle;     
    }
}


fn right_flipper_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut right_flippers: Query<(&mut RightFlipper, &mut Transform), With<RightFlipper>>,
) {
    //info!("test1");
    for (mut right_flipper, mut right_flipper_transform) in right_flippers.iter_mut() {
        //info!("test2");
        let mut new_angle = right_flipper.curr_angle;
        let change_angle:f32;

        if keyboard_input.pressed(KeyCode::Right)
        {
            //change_angle = -0.09;
            change_angle = -0.3;
        }
        else
        {
            change_angle = 0.07;
        }

        new_angle += change_angle;
        let new_clamped_angle = new_angle.clamp(-0.3, 0.3);
        let pivot_rotation = Quat::from_rotation_z(new_clamped_angle - right_flipper.curr_angle);
        //left_flipper_transform.rotate_around(left_flipper.point_of_rotation, pivot_rotation);
        //println!("pivot_rotation {:?}", pivot_rotation );
        right_flipper_transform.rotate(pivot_rotation);   
        right_flipper.curr_angle = new_clamped_angle;     
    }
}
