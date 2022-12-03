use bevy::prelude::*;
//use bevy_rapier3d::prelude::*;

#[derive(Default, Component)]
pub struct Position(pub Vec3);

#[derive(Default, Component)]
pub struct Rotation(pub Quat);

#[derive(Default, Component)]
pub struct DespawnInEndGame;

// This resource tracks when game is in it's last phase. All collected balls are released. And spawning of new balls is stopped.
#[derive(Resource)]
pub struct EndGame(pub bool);
