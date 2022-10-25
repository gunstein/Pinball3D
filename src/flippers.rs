use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct FlippersPlugin;

/* 
#[derive(Default)]
struct LeftFlipperInstance(Option<Handle<Mesh>>);

#[derive(Default)]
struct RightFlipperInstance(Option<Handle<Mesh>>);
*/

#[derive(Component)]
struct LeftFlipper{
    //position: Vec3,
    curr_angle : f32,
 }

 #[derive(Component)]
 struct RightFlipper{
     //position: Vec3,
     curr_angle : f32,
  } 

impl Plugin for FlippersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_flippers.after("main_setup").label("left_flipper"))
            .add_system(left_flipper_movement)
            .add_system(right_flipper_movement);
    }
}

fn spawn_flippers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let left_flipper_mesh_handle:Handle<Mesh> = asset_server.load("left_flipper.glb#Mesh0/Primitive0");
  
    let material = materials.add(Color::rgb(2.0, 0.9, 2.0).into());

    let left_flipper_position = Vec3::new(-0.1, -0.5, 0.0);
    let right_flipper_position = Vec3::new(0.1, -0.5, 0.0);


    let collider_big_cylinder = Collider::round_cylinder(0.02, 0.016, 0.001);
    let position_big_cylinder = Vec3::new(0.0, 0.0, 0.02);
    let rotation_big_cylinder = Quat::from_rotation_x(std::f32::consts::PI/2.0);

    let collider_small_cylinder = Collider::round_cylinder(0.02, 0.007, 0.001);
    let position_small_cylinder = Vec3::new(0.07, 0.0, 0.02);
    let rotation_small_cylinder = Quat::from_rotation_x(std::f32::consts::PI/2.0);

    let collider_upper_box = Collider::cuboid(0.035, 0.002, 0.02);
    let position_upper_box = Vec3::new(0.035, 0.01, 0.02);
    let rotation_upper_box = Quat::from_rotation_z(-0.12);
    
    let collider_lower_box = collider_upper_box.clone();
    let position_lower_box = Vec3::new(0.035, -0.01, 0.02);
    let rotation_lower_box = Quat::from_rotation_z(0.12); 

    commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: left_flipper_mesh_handle.clone(),
        material: material.clone(),
        ..default()
    }) 
    .insert(RigidBody::KinematicPositionBased)
    .insert(Ccd::enabled())
    .insert(Collider::compound(vec![
        (position_big_cylinder, rotation_big_cylinder, collider_big_cylinder.clone()),
        (position_small_cylinder, rotation_small_cylinder, collider_small_cylinder.clone()),
        (position_upper_box, rotation_upper_box, collider_upper_box.clone()),
        (position_lower_box, rotation_lower_box, collider_lower_box.clone())
    ]))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(left_flipper_position.x, left_flipper_position.y, left_flipper_position.z)))
    .insert(LeftFlipper{curr_angle:0.0}); 

    commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: left_flipper_mesh_handle.clone(),
        material: material.clone(),
        ..default()
    }) 
    .insert(RigidBody::KinematicPositionBased)
    .insert(Ccd::enabled())
    .insert(Collider::compound(vec![
        (position_big_cylinder, rotation_big_cylinder, collider_big_cylinder.clone()),
        (position_small_cylinder, rotation_small_cylinder, collider_small_cylinder.clone()),
        (position_upper_box, rotation_upper_box, collider_upper_box.clone()),
        (position_lower_box, rotation_lower_box, collider_lower_box.clone())
    ]))
    //.insert_bundle(TransformBundle::from(Transform::from_xyz(left_flipper_position.x, left_flipper_position.y, left_flipper_position.z)))
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(right_flipper_position.x, right_flipper_position.y, right_flipper_position.z),
            rotation: Quat::from_rotation_z(-std::f32::consts::PI),
            ..default()
        }
    ))
    .insert(RightFlipper{curr_angle:0.0});     
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
            change_angle = 0.09;
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
            change_angle = -0.09;
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
