use bevy::core::Zeroable;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod walls;
use walls::*;

mod flippers;
use flippers::*;

mod ball;
use ball::*;

mod utils;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum Pinball3DSystems {
    Main,
    Walls,
    Flippers,
    Ball,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pinball3d".to_string(),
            width: 360.0,
            height: 640.0,
            ..Default::default()
        })
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WallsPlugin)
        .add_plugin(FlippersPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup.label(Pinball3DSystems::Main))
        .run();
}


fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    // Set gravity to x and spawn camera.
    //rapier_config.gravity = Vec3::zeroed();
    rapier_config.gravity = Vec3::new(0.0, 0.0, -1.0);

    // camera
    /* 
    commands.spawn_bundle(Camera3dBundle {
        //transform: Transform::from_xyz(0.0, 0.5, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        transform: Transform::from_xyz(0.0, -2.5, 2.7).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        //transform: Transform::from_xyz(0.0, -0.1, 1.2).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }); 
    */
    commands
    .spawn_bundle(Camera3dBundle{
        //transform: Transform::from_xyz(-1.0, 0.5, 0.1).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Z),
        transform: Transform::from_xyz(0.0, -0.9, 1.8).looking_at(Vec3::new(0.0, -0.35, 0.0), Vec3::Z),//ok
        //transform: Transform::from_xyz(0.0, -0.5, 1.0).looking_at(Vec3::new(0.0, -0.2, 0.0), Vec3::Z),
        //transform: Transform::from_xyz(0.1, -0.5, 0.5).looking_at(Vec3::new(0.1, -0.5, 0.0), Vec3::Y),
        //transform: Transform::from_xyz(-0.5, 1.0, 2.0).looking_at(Vec3::new(-0.5, 1.0, 0.0), Vec3::Y),
        //transform: Transform::from_xyz(0.0, 0.8, 1.0).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
        //transform: Transform::from_xyz(0.0, -0.8, 0.011).looking_at(Vec3::new(0.0, -0.2, 0.011), Vec3::Z),
        ..default()
    });
    /* 
    .insert(UiCameraConfig {
        show_ui: false,
        ..default()
    });*/
}
