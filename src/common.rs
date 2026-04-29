use bevy::prelude::*;
//use bevy_rapier3d::prelude::*;

#[derive(Default, Component)]
pub struct DespawnInEndGame;

pub fn board_transform(transform: Transform) -> Transform {
    let board_rotation = Quat::from_rotation_x(0.12);
    Transform {
        translation: board_rotation * transform.translation,
        rotation: board_rotation * transform.rotation,
        scale: transform.scale,
    }
}

// This resource tracks when game is in it's last phase. All collected balls are released. And spawning of new balls is stopped.
#[derive(Resource)]
pub struct EndGame(pub bool);
