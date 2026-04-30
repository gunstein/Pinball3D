use avian3d::prelude::*;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;

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

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Pinball3DSystems {
    Main,
    Walls,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pinball3d".to_string(),
                resolution: (360, 640).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec3::new(0.0, -0.3, -0.5)))
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
        ))
        .add_systems(Startup, setup.in_set(Pinball3DSystems::Main))
        .run();
}

fn setup(mut commands: Commands) {
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
        Transform::from_xyz(0.0, -0.8, 1.8).looking_at(Vec3::new(0.0, -0.35, 0.0), Vec3::Z),
        Tonemapping::None,
    ));
}
