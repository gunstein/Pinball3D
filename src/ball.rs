use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::BottomWall;

use super::common;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_balls)
            .add_system(push_ball_to_floor)
            .add_system(handle_ball_intersections_with_bottom_wall);
    }
}

#[derive(Component)]
pub struct Ball;

//#[derive(Default, Component)]
//struct Position(Vec3);

#[derive(Default, Component)]
pub struct MaterialColor(pub Color);

struct InitBallBundle{
    position: Vec3,
    material_color: MaterialColor
}

pub const INIT_BALL_POSITION:Vec3 = Vec3::new(0.32, -0.83, 0.02);

fn spawn_balls(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
)
{
    let init_balls : [InitBallBundle;1] = [
        InitBallBundle{
            //position: Position(Vec3::new(0.32, -0.83, 0.02)),
            position: INIT_BALL_POSITION,
            material_color: MaterialColor(Color::ORANGE_RED),
        }
    ];

    for i in 0..init_balls.len() {
        let init_ball = &init_balls[i];

        spawn_single_ball(&mut commands, &mut meshes, &mut materials, &init_ball.position, &init_ball.material_color);
    }
}


pub fn spawn_single_ball(    
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: &Vec3,
    material_color: &MaterialColor,
)
{
    commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere{
            radius: 0.015,
            ..default()
        })),
        material: materials.add(material_color.0.into()
    ),
    ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    .insert(Friction {
        coefficient: 0.1,
        combine_rule: CoefficientCombineRule::Min,
    })
    //.insert(Collider::ball(0.01))
    .insert(Collider::ball(0.015))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(position.x, position.y, position.z)))
    .insert(ExternalForce {
        force: Vec3::new(0.0, 0.0, 0.0),
        //force: Vec3::new(0.0, 0.0000007, 0.0),
        torque: Vec3::new(0.0, 0.0, 0.0),
    })
    .insert(ExternalImpulse{
        impulse: Vec3::new(0.0, 0.0, 0.0),
        //force: Vec3::new(0.0, 0.0000007, 0.0),
        torque_impulse: Vec3::new(0.0, 0.0, 0.0),
    })    
    .insert(Velocity {
        linvel: Vec3::new(0.0, 0.0, 0.0),
        angvel: Vec3::new(0.0, 0.0, 0.0),
    })
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(Restitution::coefficient(0.6))
    .insert(CollisionGroups{memberships:Group::GROUP_3, filters:(Group::GROUP_1 | Group::GROUP_2)})
    .insert(MaterialColor(material_color.0))
    .insert(Ball);
}

fn push_ball_to_floor(mut query_balls: Query<(&mut ExternalForce, &mut Velocity, &Transform, &Collider), With<Ball>>, rapier_context: Res<RapierContext>) {
    //info!("push_ball_to_floor 0");

    //let test1 = CollisionGroups{memberships:Group::GROUP_3, filters:(Group::GROUP_1 | Group::GROUP_2)};
    //let test2 = InteractionGroups{memberships:Group::GROUP_1, filter:Group::GROUP_1};
    for (mut ball_force, mut ball_velocity, ball_transform, ball_collider) in query_balls.iter_mut(){
        //info!("push_ball_to_floor 1");
        let max_toi = 100.0;
        let cast_velocity = Vec3::new(0.0, 0.0, -1.0);
        //let filter = QueryFilter::default();
        //let filter = QueryFilter::only_fixed();
        //let filter = QueryFilter::only_fixed().groups(InteractionGroups::new(0b0100, 0b0011));
        //Only cast to floor.
        //let filter = QueryFilter::only_fixed().groups(InteractionGroups::new(0b0100, 0b0001));
        let filter = QueryFilter{
            //flags: QueryFilterFlags::ONLY_FIXED, 
            //groups:Some(InteractionGroups{memberships:Group::GROUP_3, filter:Group::GROUP_1}),
            groups:Some(CollisionGroups{memberships:Group::GROUP_3, filters:Group::GROUP_1}.into()), 
            //groups:InteractionGroups::new(Group::GROUP_3, Group::GROUP_1)), 
            ..default()
        };


        //println!("ball_transform.translation {:?}", ball_transform.translation);
        //println!("ball_transform.rotation {:?}", ball_transform.rotation);
        //println!("filter {:?}", filter.groups);

        if let Some((entity, hit)) = rapier_context.cast_shape(
            ball_transform.translation, ball_transform.rotation, cast_velocity, ball_collider, max_toi, filter
        ) {
            //info!("push_ball_to_floor 2");
            //println!("hit.toi {:?}", hit.toi);
            if hit.toi > 0.0{
                //ball_force.force = Vec3::new(0.0, 0.0, -0.00007).into();
                //ball_force.force = Vec3::new(0.0, -0.00007, -0.00007).into();
                
                ball_force.force = Vec3::new(0.0, 0.0, -0.0001);

                //ball_force.force = Vec3::new(0.0, -0.00007, 0.0).into();
                //ball_force.torque = Vec3::new(0.0, 0.0, 0.0);
                //info!("push_ball_to_floor 3");
            }
            else{
                //ball_force.force = Vec3::new(0.0, 0.0, 0.0).into();
                //ball_force.force = Vec3::new(0.0, 0.00007, 0.0).into();

                //ball_force.force = Vec3::new(0.0, 0.00007, -0.0007);

                //ball_force.force = Vec3::new(0.0, 0.0000007, 0.0);

                ball_force.force = Vec3::new(0.0, 0.0, 0.0);
                
                //ball_force.torque = Vec3::new(0.0, 0.0, 0.0);
                //ball_velocity.angvel = Vec3::new(0.0, 0.0, 0.0);
                //ball_velocity.linvel = Vec3::new(0.0, 0.0, 0.0);
                //info!("push_ball_to_floor 4");
            }
        }
        /* 
        else{
            info!("zero force");
            ball_force.force = Vec3::new(0.0, 0.0, 0.0);
            ball_force.torque = Vec3::new(0.0, 0.0, 0.0);
            ball_velocity.angvel = Vec3::new(0.0, 0.0, 0.0);
            ball_velocity.linvel = Vec3::new(0.0, 0.0, 0.0);
        }*/
    }
 
}

fn handle_ball_intersections_with_bottom_wall(
    rapier_context: Res<RapierContext>,
    query_ball: Query<(Entity, &MaterialColor), With<Ball>>,
    query_bottom_wall: Query<Entity, With<BottomWall>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //let mut should_spawn_ball = false;

    for entity_bottom_wall in query_bottom_wall.iter() {
        for (entity_ball, material_color) in query_ball.iter() {
            /* Find the intersection pair, if it exists, between two colliders. */
            if rapier_context.intersection_pair(entity_bottom_wall, entity_ball) == Some(true) {
                commands.entity(entity_ball).despawn();
                //spawn_single_ball(commands, meshes, materials);
                //info!("spawn new ball.");
                spawn_single_ball(&mut commands, &mut meshes, &mut materials, &INIT_BALL_POSITION, &material_color);
            }
        }
    }

    //if should_spawn_ball
    //{
    //    spawn_ball(commands, meshes, materials);
    //}
}