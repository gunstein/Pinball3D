use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

//

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_ball.after("main_setup").label("ball"))
            .add_system(push_ball_to_floor);
    }
}

#[derive(Component)]
struct Ball;

fn spawn_ball(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{
    let ball_pos = Vec3::new(0.1, -1.0, 0.5);

    /*let shape_ball = bevy::prelude::shape::Icosphere {
        radius: 0.01,
        subdivisions: 3,
    };*/

    let entity_id = commands.spawn()
    /* .insert_bundle(PbrBundle {
        //mesh: meshes.add(Mesh::from(shape_ball)),
        mesh: meshes.add(Mesh::from(shape::UVSphere{
            radius: 0.03,
            ..default()
        })),
        material: materials.add(Color::rgb(0.1, 0.2, 0.8).into()),
        //..Default::default()
        ..default()
    })*/
    .insert(RigidBody::Dynamic)
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    //.insert(Collider::ball(0.01))
    .insert(Collider::ball(0.03))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(ball_pos.x, ball_pos.y, ball_pos.z)))
    //.insert(Transform::from_xyz(ball_pos.x, ball_pos.y, ball_pos.z))
    //.insert(Transform::from_xyz(-0.1, 0.2, 0.3));
    .insert(ExternalForce {
        force: Vec3::new(0.0, 0.0, 0.0),
        torque: Vec3::new(0.0, 0.0, 0.0),
    })
    .insert(Velocity {
        linvel: Vec3::new(0.0, 0.0, 0.0),
        angvel: Vec3::new(0.0, 0.0, 0.0),
    })
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(Restitution::coefficient(0.0))
    //.insert(CollisionGroups::new(0b0010, 0b0001))
    .insert(Ball);
}


fn push_ball_to_floor(mut query_balls: Query<(&mut ExternalForce, &mut Velocity, &Transform, &Collider), With<Ball>>, rapier_context: Res<RapierContext>) {
    //info!("push_ball_to_floor 0");
    for (mut ball_force, mut ball_velocity, ball_transform, ball_collider) in query_balls.iter_mut(){
        //info!("push_ball_to_floor 1");
        let max_toi = 100.0;
        let cast_velocity = Vec3::new(0.0, 0.0, -1.0);
        //let filter = QueryFilter::default();
        let filter = QueryFilter::only_fixed();

        //println!("ball_transform.translation {:?}", ball_transform.translation);
        //println!("ball_transform.rotation {:?}", ball_transform.rotation);


        if let Some((entity, hit)) = rapier_context.cast_shape(
            ball_transform.translation, ball_transform.rotation, cast_velocity, ball_collider, max_toi, filter
        ) {
            //info!("push_ball_to_floor 2");
            //println!("hit.toi {:?}", hit.toi);
            if hit.toi > 0.0{
                //ball_force.force = Vec3::new(0.0, 0.0, -0.00007).into();
                //ball_force.force = Vec3::new(0.0, -0.00007, -0.00007).into();
                ball_force.force = Vec3::new(0.0, 0.0, -0.00007);
                //ball_force.force = Vec3::new(0.0, -0.00007, 0.0).into();
                ball_force.torque = Vec3::new(0.0, 0.0, 0.0);
                info!("push_ball_to_floor 3");
            }
            else{
                //ball_force.force = Vec3::new(0.0, 0.0, 0.0).into();
                //ball_force.force = Vec3::new(0.0, 0.00007, 0.0).into();
                ball_force.force = Vec3::new(0.0, 0.007, 0.0).into();

                //ball_force.force = Vec3::new(0.0, 0.0, 0.0);
                ball_force.torque = Vec3::new(0.0, 0.0, 0.0);
                ball_velocity.angvel = Vec3::new(0.0, 0.0, 0.0);
                ball_velocity.linvel = Vec3::new(0.0, 0.0, 0.0);
                info!("push_ball_to_floor 4");
            }
        }
        else{
            info!("zero force");
            ball_force.force = Vec3::new(0.0, 0.0, 0.0);
            ball_force.torque = Vec3::new(0.0, 0.0, 0.0);
            ball_velocity.angvel = Vec3::new(0.0, 0.0, 0.0);
            ball_velocity.linvel = Vec3::new(0.0, 0.0, 0.0);
        }
    }
 
}