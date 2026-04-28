use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod wall;
use wall::*;

mod flipper;
use flipper::*;

mod ball;
use ball::*;

mod launcher;
use launcher::*;

mod pin;
use pin::*;

mod bumper;
use bumper::*;

mod star;
use star::*;

mod target;
use target::*;

mod common;

//This is sets for startup systems. Makes it possible to influence startup system sequence.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Pinball3DSystems {
    Main,
    Walls,
    Flippers,
    Ball,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pinball3d".to_string(),
                resolution: (360.0, 640.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(TimestepMode::Variable {
            max_dt: 1.0 / 60.0,
            time_scale: 1.0,
            substeps: 2,
        })
        .insert_resource(common::EndGame(false))
        .add_plugins((
            WallPlugin,
            FlipperPlugin,
            BallPlugin,
            LauncherPlugin,
            PinPlugin,
            BumperPlugin,
            StarPlugin,
            TargetPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup.in_set(Pinball3DSystems::Main))
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
) {
    let mut rapier_config = rapier_config.single_mut();
    rapier_config.gravity = Vec3::new(0.0, -0.3, -1.0);

    // camera and light
    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(-2.0, 0.0, 5.0),
    ));

    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(2.0, 0.0, 5.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, -0.8, 1.8).looking_at(Vec3::new(0.0, -0.35, 0.0), Vec3::Z), //ok
        Tonemapping::None,
        //transform: Transform::from_xyz(0.32, -0.8, 0.1).looking_at(Vec3::new(0.32, -0.3, 0.0), Vec3::Z),
    ));
}
