use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use std::ops::Add;

use super::Floor;
use super::Ball;
pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_launcher_and_gate)
            .add_system(launcher_movement)
            .add_system(handle_gate_sensor_events);
    }
}

#[derive(Component)]
struct Launcher{
    start_pos : Vec3,
 }

 #[derive(Component)]
 struct GateSensor;

fn spawn_launcher_and_gate(
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
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    .insert(Collider::cuboid(0.02, 0.02, 0.02))
    .insert(CollisionGroups{memberships:Group::GROUP_2, filters:Group::GROUP_3})
    .insert_bundle(TransformBundle::from(Transform::from_xyz(launcher_pos.x, launcher_pos.y, launcher_pos.z)))
    .insert(Launcher{start_pos: launcher_pos})
    .id();

    //Launcher gate
    //Add launcher gate, connected with joints between outer_wall and launcher_wall
    //OneWayGate
    let gate_anchor_pos = Vec3::new(0.3, -0.42, 0.1);
    let launcher_gate_mesh_handle:Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(0.017*2.0,0.003*2.0, 0.04*2.0)));
    let material_launcher_gate = materials.add(Color::RED.into());

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
    
    //let pivot_rotation = Quat::from_rotation_z(0.1);
    //left_flipper_transform.rotate_around(left_flipper.point_of_rotation, pivot_rotation);

    //Litt rart å legge transformasjon for nedtrillingscollider her, men men...
    //let mut transform = Transform::identity();
    //transform.rotate_around(Vec3::new(-0.017, 0.0, 0.04), Quat::from_rotation_z(0.1));


    let launcher_gate = 
    commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: launcher_gate_mesh_handle.clone(),
        material: material_launcher_gate.clone(),
        ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    .with_children(|children| {
        children.spawn()
        .insert(Collider::cuboid(0.017,0.003, 0.04))
        .insert(CollisionGroups{memberships:Group::GROUP_2, filters:Group::GROUP_3});
        //children.spawn()
        //.insert(Collider::cuboid(0.017,0.003, 0.04))
        //.insert_bundle(TransformBundle::from(
        //    transform
        //));
        children.spawn()
        .insert(ImpulseJoint::new(gate_anchor, joint));
    })
    //.insert(CollisionGroups{memberships:Group::GROUP_6, filters:Group::GROUP_8})
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(gate_anchor_pos.x, gate_anchor_pos.y, gate_anchor_pos.z - 0.04),
            //rotation: Quat::from_rotation_z(-0.92),
            ..default()
        }
    ))
    .id();

    //one way gate collider, used to prevent stuck ball.
    let gate_collider_pos = Vec3::new(0.33, -0.4, 0.05);
    let gate_collider = commands.spawn()
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.03,0.003, 0.04))
    .insert(CollisionGroups{memberships:Group::GROUP_4, filters:Group::GROUP_3})
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(gate_collider_pos.x, gate_collider_pos.y, gate_collider_pos.z ),
            rotation: Quat::from_rotation_z(0.1),
            ..default()
        }
    ))
    .id();
    
    //Sensor above gate. Used to change collider group of ball
    let gate_sensor_position = Vec3::new(0.33, -0.38, 0.05);
    let gate_sensor = commands.spawn()
    .insert(Collider::cuboid(0.03, 0.003, 0.04))
    .insert(Sensor)
    .insert_bundle(TransformBundle::from(Transform::from_xyz(gate_sensor_position.x, gate_sensor_position.y, gate_sensor_position.z)))
    .insert(GateSensor)
    .id(); 

    commands.entity(floor.unwrap())
    //.push_children(&[launcher]);
    //.push_children(&[launcher, gate_anchor, launcher_gate, gate_sensor, gate_collider]);
    .push_children(&[launcher, gate_anchor, launcher_gate, gate_sensor, gate_collider]);
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
        let clamped_ypos = next_ypos.clamp(launcher.start_pos.y, launcher.start_pos.y +  0.06);
        launcher_transform.translation.y = clamped_ypos;    
    }
}

fn handle_gate_sensor_events(
    query_gate_sensors: Query<Entity, With<GateSensor>>,
    mut query_balls: Query<(Entity, &mut CollisionGroups), With<Ball>>,
    mut contact_events: EventReader<CollisionEvent>,
    //mut commands: Commands,
) {
    for contact_event in contact_events.iter() {
        for sensor_entity in query_gate_sensors.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &sensor_entity || h2 == &sensor_entity {
                    //If absolutvalue of velocity in x is close to zero, ball can get stuck so apply force in x-direction.
                    //Find right ball
                    for (entity_ball, mut collision_group) in query_balls.iter_mut() {
                        if h1 == &entity_ball || h2 == &entity_ball {
                            //if velocity.linvel.x.abs() < 0.1{
                                //info!("handle_gate_sensor_events 1");
                                //let force_to_add = Vec3::new(-0.0000008, 0.0, 0.0);
                                //external_impulse.impulse = external_impulse.impulse.add(force_to_add);
                            //} 
                            //Add GROUP_4 to filters. This will activate collision between the ball and the one way gate collider
                            collision_group.filters = Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3 | Group::GROUP_4;
                        }
                    }
                }
            }
        }
    }
}