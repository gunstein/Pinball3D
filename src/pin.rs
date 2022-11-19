use std::ops::Add;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::Floor;
use super::Ball;

pub struct PinPlugin;

impl Plugin for PinPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_pins)
            .add_system(handle_pin_events)
            .add_system(respawn_pin_to_toggle_color);
    }
}

#[derive(Component)]
struct Pin{
    timestamp_last_hit : f64,
    position : Vec3,
}

fn spawn_pins(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<Entity, With<Floor>>
)
{
    let pins_pos : [Vec3;6] = [
        Vec3::new(-0.1, 0.0, 0.05),
        Vec3::new(0.1, 0.0, 0.05),
        Vec3::new(0.0,  -0.1, 0.05),
        //test
        Vec3::new(0.0,  0.3, 0.05),
        Vec3::new(-0.25,  0.0, 0.05),
        Vec3::new(0.2,  0.1, 0.05),
    ];

    for i in 0..pins_pos.len() {
        let pin_pos = pins_pos[i];

        spawn_single_pin(&mut commands, pin_pos, None, &mut meshes, &mut materials, &query_floors);
    }
}


fn spawn_single_pin(    
    commands: &mut Commands,
    position: Vec3,
    timestamp_last_hit: Option<f64>,
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

    let temp_timestamp_last_hit = timestamp_last_hit.unwrap_or(0.0);

    let mut color = Color::GREEN;
    if temp_timestamp_last_hit == 0.0{
        color = Color::TEAL;
    }

    let material_pin = materials.add(color.into());

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
    .insert(Pin{timestamp_last_hit: temp_timestamp_last_hit, position: position })
    .id();

    commands.entity(floor.unwrap()).add_child(pin);
}

fn respawn_pin_to_toggle_color(mut query: Query<(Entity, &Pin), With<Pin>>, 
        time: Res<Time>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        query_floors: Query<Entity, With<Floor>>,
    ) {
    for (entity, pin) in query.iter_mut() {
        let diff = time.seconds_since_startup() - pin.timestamp_last_hit;
        if pin.timestamp_last_hit > 0.0 && diff > 1.0{
            //Color have been toggled for more than a second so respawn
            let pos = pin.position;
            commands.entity(entity).despawn();
            spawn_single_pin(&mut commands, pos, None, &mut meshes, &mut materials, &query_floors);
        }
    }
}

/* 
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    query_ball: Query<Entity, With<Ball>>,
    query_pins: Query<Entity, With<Pin>>,
) {
    for collision in collision_events.iter() {
        match *collision {
            CollisionEvent::Started(e1, e2, _) => {
                let mut ball = None;
                if let Ok(ball_e1) = query_ball.get(e1){
                    ball = Some(ball_e1);
                }
                else if let Ok(ball_e2) = query_ball.get(e2){
                    ball = query_ball.get(e2);
                }

                if Some(ball){

                }
                if let Ok([c1, c2]) = [e1, e2] {
                    // stack cards here
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if let Ok([c1, c2]) = cards.get_many_mut([e1, e2]) {
                    // unstack cards here
                }
            }
        }
    }

}
*/

fn handle_pin_events(
    query_pins: Query<(Entity, &Pin), With<Pin>>,
    mut query_balls: Query<(Entity, &mut ExternalImpulse, &Velocity), With<Ball>>,
    time: Res<Time>,
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
                    let pos = pin.position;
                    let timestamp_last_hit = time.seconds_since_startup();
                    commands.entity(entity_pin).despawn();
                    spawn_single_pin(&mut commands, pos, Some(timestamp_last_hit), &mut meshes, &mut materials, &query_floors);
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