use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::BottomWall;
use super::common;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_balls).add_systems(
            Update,
            (
                push_ball_to_floor,
                handle_ball_intersections_with_bottom_wall,
            ),
        );
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Default, Component)]
pub struct MaterialColor(pub Color);

pub const INIT_BALL_POSITION: Vec3 = Vec3::new(0.32, -0.83, 0.02);

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_single_ball(
        &mut commands,
        &mut meshes,
        &mut materials,
        INIT_BALL_POSITION,
        MaterialColor(Color::srgb(1.0, 0.27, 0.0)),
    );
}

pub fn spawn_single_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    material_color: MaterialColor,
) {
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Sphere::new(0.015)))),
        MeshMaterial3d(materials.add(material_color.0)),
        RigidBody::Dynamic,
        Sleeping::disabled(),
        Ccd::enabled(),
        Friction { coefficient: 0.1, combine_rule: CoefficientCombineRule::Min },
        Collider::ball(0.015),
        common::board_transform(Transform::from_translation(position)),
        ExternalForce::default(),
        ExternalImpulse::default(),
        Velocity::default(),
        ActiveEvents::COLLISION_EVENTS,
        Restitution::coefficient(0.6),
        CollisionGroups {
            memberships: Group::GROUP_3,
            filters: Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3,
        },
        (material_color, Ball),
    ));
}

fn push_ball_to_floor(
    mut query_balls: Query<(&mut ExternalForce, &mut Velocity, &Transform, &Collider), With<Ball>>,
    rapier_context: ReadRapierContext,
) {
    let Ok(rapier_context) = rapier_context.single() else {
        return;
    };

    for (mut ball_force, _ball_velocity, ball_transform, ball_collider) in query_balls.iter_mut() {
        let filter = QueryFilter {
            groups: Some(
                CollisionGroups {
                    memberships: Group::GROUP_3,
                    filters: Group::GROUP_1,
                }
                .into(),
            ),
            ..default()
        };

        if let Some((_entity, hit)) = rapier_context.cast_shape(
            ball_transform.translation,
            ball_transform.rotation,
            Vec3::NEG_Z,
            ball_collider.into(),
            ShapeCastOptions::with_max_time_of_impact(100.0),
            filter,
        ) {
            ball_force.force = if hit.time_of_impact > 0.0 {
                Vec3::new(0.0, 0.0, -0.0001)
            } else {
                Vec3::ZERO
            };
        }
    }
}

fn handle_ball_intersections_with_bottom_wall(
    rapier_context: ReadRapierContext,
    query_ball: Query<(Entity, &MaterialColor), With<Ball>>,
    query_bottom_wall: Query<Entity, With<BottomWall>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    end_game: Res<common::EndGame>,
) {
    let Ok(rapier_context) = rapier_context.single() else {
        return;
    };

    for entity_bottom_wall in query_bottom_wall.iter() {
        for (entity_ball, material_color) in query_ball.iter() {
            if rapier_context.intersection_pair(entity_bottom_wall, entity_ball) == Some(true) {
                commands.entity(entity_ball).despawn();
                if !end_game.0 {
                    spawn_single_ball(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        INIT_BALL_POSITION,
                        MaterialColor(material_color.0),
                    );
                }
            }
        }
    }
}
