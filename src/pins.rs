use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::Floor;

pub struct PinsPlugin;

impl Plugin for PinsPlugin {
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
    let pins_pos : [Vec3;3] = [
        Vec3::new(-0.1, 0.0, 0.05),
        Vec3::new(0.1, 0.0, 0.05),
        Vec3::new(0.0,  -0.1, 0.05),
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

    let pin_radius = 0.03;
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

fn handle_pin_events(
    query: Query<(Entity, &Pin), With<Pin>>,
    time: Res<Time>,
    mut contact_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<Entity, With<Floor>>,
) {
    for contact_event in contact_events.iter() {
        for (entity, pin) in query.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity || h2 == &entity {
                    //Respawn to change color
                    let pos = pin.position;
                    let timestamp_last_hit = time.seconds_since_startup();
                    commands.entity(entity).despawn();
                    spawn_single_pin(&mut commands, pos, Some(timestamp_last_hit), &mut meshes, &mut materials, &query_floors);
                }
            }
        }
    }
}