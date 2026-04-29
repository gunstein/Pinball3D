use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use rand::Rng;

use super::common;
use super::Ball;

pub struct PinPlugin;

impl Plugin for PinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_pins)
            .add_systems(Update, handle_pin_events);
    }
}

#[derive(Component)]
struct Pin;

fn spawn_pins(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pins_pos: [Vec3; 10] = [
        Vec3::new(0.0, 0.07, 0.05),
        Vec3::new(-0.1, 0.0, 0.05),
        Vec3::new(0.1, 0.0, 0.05),
        Vec3::new(0.0, -0.1, 0.05),
        Vec3::new(-0.1, -0.2, 0.05),
        Vec3::new(0.1, -0.2, 0.05),
        Vec3::new(0.0, -0.3, 0.05),
        Vec3::new(-0.1, -0.36, 0.05),
        Vec3::new(0.1, -0.36, 0.05),
        Vec3::new(0.0, -0.44, 0.05),
    ];

    for pin_pos in pins_pos {
        spawn_single_pin(&mut commands, pin_pos, None, &mut meshes, &mut materials);
    }
}

fn spawn_single_pin(
    commands: &mut Commands,
    position: Vec3,
    color: Option<Color>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let pin_radius = 0.035;
    let pin_depth = 0.05;
    let pin_mesh_handle: Handle<Mesh> =
        meshes.add(Mesh::from(Capsule3d::new(pin_radius, pin_depth)));

    let chosen_color = color.unwrap_or(Color::srgb(0.0, 0.5, 0.5));

    let material_pin = materials.add(chosen_color);

    commands
        .spawn((
            Mesh3d(pin_mesh_handle),
            MeshMaterial3d(material_pin),
            RigidBody::Fixed,
            Collider::round_cylinder(pin_depth, pin_radius, 0.001),
            common::board_transform(Transform {
                translation: position,
                rotation: Quat::from_rotation_x(std::f32::consts::PI / 2.0),
                ..default()
            }),
            Restitution::coefficient(0.7),
            Pin,
        ));
}

fn handle_pin_events(
    mut query_pins: Query<(Entity, &mut MeshMaterial3d<StandardMaterial>), With<Pin>>,
    mut query_balls: Query<(Entity, &mut ExternalImpulse, &Velocity), With<Ball>>,
    mut contact_events: MessageReader<CollisionEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for contact_event in contact_events.read() {
        for (entity_pin, mut material) in query_pins.iter_mut() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity_pin || h2 == &entity_pin {
                    //Respawn to change color
                    let color_selection: [Color; 5] = [
                        Color::srgb(1.0, 1.0, 0.0),
                        Color::srgb(1.0, 0.0, 0.0),
                        Color::srgb(0.0, 0.0, 1.0),
                        Color::srgb(0.0, 0.5, 0.0),
                        Color::srgb(1.0, 0.753, 0.796),
                    ];
                    let mut rng = rand::thread_rng();
                    let chosen_index = rng.gen_range(0..5);
                    let material_pin = materials.add(color_selection[chosen_index]);
                    *material = MeshMaterial3d(material_pin.clone());
                }
            }
            if let CollisionEvent::Stopped(h1, h2, _event_flag) = contact_event {
                if h1 == &entity_pin || h2 == &entity_pin {
                    //Give ball a push in velocity direction
                    for (entity_ball, mut external_impulse, velocity) in query_balls.iter_mut() {
                        if h1 == &entity_ball || h2 == &entity_ball {
                            external_impulse.impulse += velocity.linvel.normalize() * 0.000003;
                        }
                    }
                }
            }
        }
    }
}
