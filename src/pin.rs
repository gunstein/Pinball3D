use avian3d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

use super::common;
use super::common::GameLayer;
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
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let pin_radius = 0.035;
    let pin_depth = 0.05;

    let chosen_color = color.unwrap_or(Color::srgb(0.0, 0.5, 0.5));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Capsule3d::new(pin_radius, pin_depth)))),
        MeshMaterial3d(materials.add(chosen_color)),
        RigidBody::Static,
        Collider::capsule(pin_radius, pin_depth),
        common::board_transform(Transform {
            translation: position,
            rotation: Quat::from_rotation_x(std::f32::consts::PI / 2.0),
            ..default()
        }),
        Restitution::new(0.7),
        CollisionEventsEnabled,
        CollisionLayers::new(GameLayer::Obstacles, [GameLayer::Ball]),
        Pin,
    ));
}

fn handle_pin_events(
    mut query_pins: Query<(Entity, &mut MeshMaterial3d<StandardMaterial>), With<Pin>>,
    mut query_balls: Query<(Entity, Forces), With<Ball>>,
    mut start_events: MessageReader<CollisionStart>,
    mut end_events: MessageReader<CollisionEnd>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in start_events.read() {
        for (entity_pin, mut material) in query_pins.iter_mut() {
            if event.collider1 == entity_pin || event.collider2 == entity_pin {
                let color_selection: [Color; 5] = [
                    Color::srgb(1.0, 1.0, 0.0),
                    Color::srgb(1.0, 0.0, 0.0),
                    Color::srgb(0.0, 0.0, 1.0),
                    Color::srgb(0.0, 0.5, 0.0),
                    Color::srgb(1.0, 0.753, 0.796),
                ];
                let mut rng = rand::thread_rng();
                let chosen_index = rng.gen_range(0..5);
                *material = MeshMaterial3d(materials.add(color_selection[chosen_index]));
            }
        }
    }

    for event in end_events.read() {
        for (entity_pin, _) in query_pins.iter() {
            if event.collider1 == entity_pin || event.collider2 == entity_pin {
                let ball_entity = if event.collider1 == entity_pin {
                    event.collider2
                } else {
                    event.collider1
                };
                if let Ok((_, mut forces)) = query_balls.get_mut(ball_entity) {
                    let velocity = forces.linear_velocity();
                    forces.apply_linear_impulse(velocity.normalize_or_zero() * 0.000003);
                }
            }
        }
    }
}
