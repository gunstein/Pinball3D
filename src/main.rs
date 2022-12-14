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

//This is labels for startup systems. Makes it possible to influence startup system sequence.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum Pinball3DSystems {
    Main,
    Walls,
    Flippers,
    Ball,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Pinball3d".to_string(),
                width: 360.0,
                height: 640.0,
                ..default()
            },
            ..default()
        }))
        .insert_resource(Msaa::default())
        .insert_resource(common::EndGame(false))
        .add_plugin(WallPlugin)
        .add_plugin(FlipperPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(LauncherPlugin)
        .add_plugin(PinPlugin)
        .add_plugin(BumperPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup.label(Pinball3DSystems::Main))
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec3::new(0.0, -0.3, -1.0);
    rapier_config.timestep_mode = TimestepMode::Variable {
        max_dt: 1.0 / 60.0,
        time_scale: 1.0,
        substeps: 2,
    };

    // camera and light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(-2.0, 0.0, 5.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 0.0, 5.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, -0.8, 1.8)
            .looking_at(Vec3::new(0.0, -0.35, 0.0), Vec3::Z), //ok
        //transform: Transform::from_xyz(0.32, -0.8, 0.1).looking_at(Vec3::new(0.32, -0.3, 0.0), Vec3::Z),
        ..default()
    });
}
