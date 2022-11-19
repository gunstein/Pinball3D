use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::Floor;
pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_launcher)
            .add_system(launcher_movement);
    }
}

#[derive(Component)]
struct Launcher{
    start_pos : Vec3,
 }

fn spawn_launcher(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<Entity, With<Floor>>
) {
    let mut floor = None;
    for entity in query_floors.iter(){
        floor = Some(entity);
    }

    let launcher_pos = Vec3::new(0.34, -0.95, 0.03);
    let launcher_mesh_handle:Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(0.02*2.0,0.02*2.0, 0.02*2.0)));
    let material_launcher = materials.add(Color::YELLOW.into());
 
    let launcher = commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: launcher_mesh_handle.clone(),
        material: material_launcher.clone(),
        ..default()
    })
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::cuboid(0.02, 0.02, 0.02))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(launcher_pos.x, launcher_pos.y, launcher_pos.z)))
    .insert(Launcher{start_pos: launcher_pos})
    .id();


    //Launcher gate
    //Add launcher gate, connected with joints between outer_wall and launcher_wall
    //OneWayGate
    /*let gate_anchor_pos = Vec3::new(0.3, -0.42, 0.1);

    let gate_anchor = commands.spawn()
    .insert(RigidBody::Fixed)
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(gate_anchor_pos.x, gate_anchor_pos.y, gate_anchor_pos.z),
            ..default()
        }
    ))
    .id();

    let joint_axis = Vec3::new(1.0, 0.0, 0.0);
    let joint = RevoluteJointBuilder::new(joint_axis)
        .limits([0.0, std::f32::consts::PI / 2.0])
        .local_anchor1(Vec3::new(0.015, 0.0, 0.0)) //pos in local coordinates of joint
        .local_anchor2(Vec3::new(-0.017, 0.0, 0.04)); //pos in local coordinates of gate
    
    let pivot_rotation = Quat::from_rotation_z(0.1);
    //left_flipper_transform.rotate_around(left_flipper.point_of_rotation, pivot_rotation);

    //Litt rart å legge transformasjon for nedtrillingscollider her, men men...
    //let mut transform = Transform::identity();
    //transform.rotate_around(Vec3::new(-0.017, 0.0, 0.04), Quat::from_rotation_z(0.1));


    let launcher_gate = 
    commands.spawn()
    .insert(RigidBody::Dynamic)
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    .with_children(|children| {
        children.spawn()
        .insert(Collider::cuboid(0.017,0.003, 0.04));
        /*children.spawn()
        .insert(Collider::cuboid(0.017,0.003, 0.04))
        .insert_bundle(TransformBundle::from(
            transform
        ));*/
        children.spawn()
        .insert(ImpulseJoint::new(gate_anchor, joint));
    })
    .insert(CollisionGroups{memberships:Group::GROUP_2, filters:Group::GROUP_3})
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(gate_anchor_pos.x, gate_anchor_pos.y, gate_anchor_pos.z - 0.04),
            //rotation: Quat::from_rotation_z(-0.92),
            ..default()
        }
    ))
    .id();
    */

    commands.entity(floor.unwrap())
    .push_children(&[launcher]);
    //.push_children(&[launcher, gate_anchor, launcher_gate]);
}

fn launcher_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut launchers: Query<(&mut Launcher, &mut Transform), With<Launcher>>,
) {
    for (launcher, mut launcher_transform) in launchers.iter_mut() {
        let mut next_ypos = launcher_transform.translation.y;
        
        if keyboard_input.pressed(KeyCode::Space)
        {
            next_ypos = next_ypos + 0.03;
        }
        else
        {
            next_ypos = next_ypos - 0.02;
        }   
        let clamped_ypos = next_ypos.clamp(launcher.start_pos.y, launcher.start_pos.y +  0.05);
        launcher_transform.translation.y = clamped_ypos;    
    }
}
