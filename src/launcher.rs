use avian3d::prelude::*;
use bevy::prelude::*;

use super::Ball;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LauncherState>()
            .add_systems(Update, update_launcher)
            .add_systems(FixedUpdate, guide_ball_in_launcher_lane);
    }
}

#[derive(Default, Resource)]
struct LauncherState {
    pressed_at_secs: Option<f64>,
}

fn update_launcher(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut launcher_state: ResMut<LauncherState>,
    mut balls: Query<(&Position, &mut LinearVelocity), With<Ball>>,
) {
    const CHARGE_TIME_SECS: f32 = 1.5;
    const MIN_LAUNCH_SPEED: f32 = 2.0;
    const MAX_LAUNCH_SPEED: f32 = 11.5;

    if keyboard_input.just_pressed(KeyCode::Space) {
        launcher_state.pressed_at_secs = Some(time.elapsed_secs_f64());
    }

    if keyboard_input.just_released(KeyCode::Space) {
        let held_secs = launcher_state
            .pressed_at_secs
            .map(|pressed_at_secs| time.elapsed_secs_f64() - pressed_at_secs)
            .unwrap_or(0.0) as f32;
        let charge = (held_secs / CHARGE_TIME_SECS).clamp(0.0, 1.0);
        let board_rotation = Quat::from_rotation_x(0.12);
        let launch_strength = charge * charge;
        let launch_speed =
            MIN_LAUNCH_SPEED + (MAX_LAUNCH_SPEED - MIN_LAUNCH_SPEED) * launch_strength;
        for (ball_pos, mut ball_vel) in &mut balls {
            let ball_local = board_rotation.inverse() * ball_pos.0;
            let ball_in_launcher_lane = ball_local.x > 0.23 && ball_local.x < 0.41;

            if ball_in_launcher_lane {
                ball_vel.0 = board_rotation * Vec3::new(0.0, launch_speed, 0.18);
            }
        }
        launcher_state.pressed_at_secs = None;
    }
}

fn guide_ball_in_launcher_lane(mut balls: Query<(&Position, &mut LinearVelocity), With<Ball>>) {
    let board_rotation = Quat::from_rotation_x(0.12);
    let target_x = 0.342;

    for (position, mut velocity) in &mut balls {
        let ball_local = board_rotation.inverse() * position.0;
        let mut velocity_local = board_rotation.inverse() * velocity.0;
        let ball_in_launcher_lane = ball_local.x > 0.30
            && ball_local.x < 0.38
            && ball_local.y < -0.62
            && velocity_local.y > 0.1;

        if ball_in_launcher_lane {
            let correction = (target_x - ball_local.x) * 8.0;
            velocity_local.x = correction.clamp(-0.30, 0.30);
            velocity.0 = board_rotation * velocity_local;
        } else if ball_local.x > 0.285
            && ball_local.x < 0.35
            && ball_local.y > -0.72
            && ball_local.y < -0.48
            && velocity_local.length() < 0.25
        {
            velocity_local.x = -0.65;
            velocity_local.y = -0.25;
            velocity.0 = board_rotation * velocity_local;
        }
    }
}
