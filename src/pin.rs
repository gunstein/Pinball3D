use std::ops::Add;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use rand::Rng;

use super::Floor;
use super::Ball;

pub struct PinPlugin;

impl Plugin for PinPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_pins)
            .add_system(handle_pin_events);
    }
}

#[derive(Component)]
struct Pin{
    position : Vec3,
}


fn spawn_pins(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<Entity, With<Floor>>
)
{
    let pins_pos : [Vec3;10] = [
        Vec3::new(0.0, 0.07, 0.05),

        Vec3::new(-0.1, 0.0, 0.05),
        Vec3::new(0.1, 0.0, 0.05),

        Vec3::new(0.0,  -0.1, 0.05),

        Vec3::new(-0.1,  -0.2, 0.05),
        Vec3::new(0.1,  -0.2, 0.05), 

        Vec3::new(0.0,  -0.3, 0.05),
        
        Vec3::new(-0.1,  -0.36, 0.05),
        Vec3::new(0.1,  -0.36, 0.05),

        Vec3::new(0.0,  -0.44, 0.05),
    ];

    for i in 0..pins_pos.len() {
        let pin_pos = pins_pos[i];

        spawn_single_pin(&mut commands, pin_pos, None, &mut meshes, &mut materials, &query_floors);
    }
}


fn spawn_single_pin(    
    commands: &mut Commands,
    position: Vec3,
    color: Option<Color>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    query_floors: & Query<Entity, With<Floor>>,
)
{
    let mut floor = None;
    for entity in query_floors.iter(){
        floor = Some(entity);
    }

    let pin_radius = 0.035;
    let pin_depth = 0.05;
    let pin_mesh_handle:Handle<Mesh> = meshes.add(Mesh::from(shape::Capsule {
        depth: pin_depth,
        radius: pin_radius,
        ..default()
    }));

 
    let mut chosen_color = Color::TEAL;
    if color.is_some(){
        chosen_color = color.unwrap();
    } 

    let material_pin = materials.add(chosen_color.into());

    let pin = commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: pin_mesh_handle.clone(),
        material: material_pin.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    //.insert(Collider::ball(pin_radius))
    .insert(Collider::round_cylinder(pin_depth, pin_radius, 0.001))
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(position.x, position.y, position.z),
            rotation: Quat::from_rotation_x(std::f32::consts::PI / 2.0),
            ..default()
        }
    ))
    .insert(Restitution::coefficient(0.7))
    .insert(Pin{position: position })
    .id();

    commands.entity(floor.unwrap()).add_child(pin);
}


fn handle_pin_events(
    query_pins: Query<(Entity, &Pin), With<Pin>>,
    mut query_balls: Query<(Entity, &mut ExternalImpulse, &Velocity), With<Ball>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<Entity, With<Floor>>,
) {
    for contact_event in contact_events.iter() {
        for (entity_pin, pin) in query_pins.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity_pin || h2 == &entity_pin {
                    //Respawn to change color
                    let color_selection : [Color; 5]= [
                        Color::YELLOW,
                        Color::RED,
                        Color::BLUE,
                        Color::GREEN,
                        Color::PINK
                    ];
                    let mut rng = rand::thread_rng();
                    let chosen_index = rng.gen_range(0..5);
                    let pos = pin.position;
                    commands.entity(entity_pin).despawn();
                    spawn_single_pin(&mut commands, pos, Some(color_selection[chosen_index]), &mut meshes, &mut materials, &query_floors);
                }
            }
            if let CollisionEvent::Stopped(h1, h2, _event_flag) = contact_event {
                if h1 == &entity_pin || h2 == &entity_pin {
                    //Give ball a push in velocity direction
                    for (entity_ball, mut external_impulse, velocity) in query_balls.iter_mut() {
                        if h1 == &entity_ball || h2 == &entity_ball {
                            let normalized_velocity = velocity.linvel.normalize();
                            external_impulse.impulse = external_impulse.impulse.add(normalized_velocity * 0.000003);
                        }
                    }
                }
            }
        }
    }
}