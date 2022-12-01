use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Default, Component)]
pub struct Position(pub Vec3);

#[derive(Default, Component)]
pub struct Rotation(pub Quat);