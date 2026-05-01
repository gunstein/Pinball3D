use avian3d::prelude::*;
use bevy::prelude::*;

use super::common;
use super::common::GameLayer;
use super::BottomWall;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_balls).add_systems(
            Update,
            (push_ball_to_floor, handle_ball_collisions_with_bottom_wall),
        );
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Default, Component)]
pub struct MaterialColor(pub Color);

pub const INIT_BALL_POSITION: Vec3 = Vec3::new(0.342, -0.83, 0.02);

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
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
    material_color: MaterialColor,
) {
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Sphere::new(0.015)))),
        MeshMaterial3d(materials.add(material_color.0)),
        RigidBody::Dynamic,
        SleepingDisabled,
        SweptCcd::default(),
        Friction::new(0.1).with_combine_rule(CoefficientCombine::Min),
        Collider::sphere(0.015),
        common::board_transform(Transform::from_translation(position)),
        LinearVelocity::default(),
        CollisionEventsEnabled,
        Restitution::new(0.6),
        CollisionLayers::new(
            GameLayer::Ball,
            [GameLayer::Floor, GameLayer::Obstacles, GameLayer::Ball],
        ),
        (material_color, Ball),
    ));
}

fn push_ball_to_floor(
    mut query_balls: Query<(Forces, &GlobalTransform, &Collider), With<Ball>>,
    spatial_query: SpatialQuery,
) {
    let filter = SpatialQueryFilter::from_mask([GameLayer::Floor]);

    for (mut forces, transform, collider) in &mut query_balls {
        let (_, rotation, translation) = transform.to_scale_rotation_translation();

        if let Some(hit) = spatial_query.cast_shape(
            collider,
            translation,
            rotation,
            Dir3::NEG_Z,
            &ShapeCastConfig::from_max_distance(100.0),
            &filter,
        ) {
            if hit.distance > 0.0 {
                forces.apply_force(Vec3::new(0.0, 0.0, -0.0001));
            }
        }
    }
}

fn handle_ball_collisions_with_bottom_wall(
    mut contact_events: MessageReader<CollisionStart>,
    query_ball: Query<(Entity, &MaterialColor, &Position), With<Ball>>,
    query_bottom_wall: Query<Entity, With<BottomWall>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    end_game: Res<common::EndGame>,
) {
    for event in contact_events.read() {
        let ball_entity = if query_ball.contains(event.collider1)
            && query_bottom_wall.contains(event.collider2)
        {
            event.collider1
        } else if query_ball.contains(event.collider2)
            && query_bottom_wall.contains(event.collider1)
        {
            event.collider2
        } else {
            continue;
        };

        if let Ok((_, material_color, position)) = query_ball.get(ball_entity) {
            let board_rotation = Quat::from_rotation_x(0.12);
            let ball_local = board_rotation.inverse() * position.0;
            if ball_local.x > 0.29 {
                continue;
            }

            let color = material_color.0;
            commands.entity(ball_entity).despawn();
            if !end_game.0 {
                spawn_single_ball(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    INIT_BALL_POSITION,
                    MaterialColor(color),
                );
            }
        }
    }
}
