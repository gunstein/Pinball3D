use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Default, Component)]
pub struct DespawnInEndGame;

#[derive(Resource)]
pub struct EndGame(pub bool);

pub fn board_transform(transform: Transform) -> Transform {
    let board_rotation = Quat::from_rotation_x(0.12);
    Transform {
        translation: board_rotation * transform.translation,
        rotation: board_rotation * transform.rotation,
        scale: transform.scale,
    }
}

#[derive(PhysicsLayer, Default, Clone, Copy, Debug)]
pub enum GameLayer {
    #[default]
    Default,
    Floor,
    Obstacles,
    Ball,
    Lid,
}
