use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::{
    prelude::{AppLooplessStateExt, ConditionHelpers, IntoConditionalSystem},
    state::NextState,
};

use crate::{
    states::{GameState, PauseState},
    InGameItem,
};

#[derive(Component)]
struct Player {
    jump_start: f32,
}

#[derive(Component)]
struct CCAcceleration(Vec2);

#[derive(Component)]
struct CCVelocity(Vec2);
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            camera_follow_player
                .run_in_state(GameState::InGame)
                .run_in_state(PauseState::Running),
        )
        .add_system(
            player_movement
                .run_in_state(GameState::InGame)
                .run_in_state(PauseState::Running),
        )
        .add_system(
            player_fall_out
                .run_in_state(GameState::InGame)
                .run_in_state(PauseState::Running),
        )
        .add_enter_system(GameState::InGame, spawn_player);
    }
}

const CC_JUMP_ACCEL: f32 = 1.2;
const CC_JUMP_MAX_DURATION: f32 = 1.;
const CC_JUMP_FALLOFF_EXPONENT: f32 = 12.;
const CC_GRAVITY: f32 = 0.3;

const CC_WALK_SPEED: f32 = 2.3;
const CC_WALK_ACCEL: f32 = 0.1;
const CC_FRICTION_COEFFICIENT: f32 = 1.1;

const PLAYER_RADIUS: f32 = 4.;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_size = Some(Vec2::new(PLAYER_RADIUS * 2., PLAYER_RADIUS * 2.));
    // Player
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(PLAYER_RADIUS))
        .insert(KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
            translation: Some(Vec2::ZERO),
            ..default()
        })
        .insert(CCAcceleration(Vec2::new(0., 0.)))
        .insert(CCVelocity(Vec2::new(0., 0.)))
        .insert(Player { jump_start: 0. })
        .insert(InGameItem)
        .insert(SpriteBundle {
            texture: asset_server.load("img/character/outline.png"),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: sprite_size,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.),
            ..default()
        })
        .with_children(|f| {
            f.spawn(SpriteBundle {
                texture: asset_server.load("img/character/body.png"),
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: sprite_size,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });
        })
        .with_children(|f| {
            f.spawn(SpriteBundle {
                texture: asset_server.load("img/character/face_angry.png"),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            });
        });
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(
        &mut KinematicCharacterController,
        &KinematicCharacterControllerOutput,
        &mut CCAcceleration,
        &mut CCVelocity,
        &mut Player,
    )>,
) {
    for (mut controller, output, mut acc, mut vel, mut player) in &mut player_info {
        // friction
        // if output.grounded {
        vel.0.x /= CC_FRICTION_COEFFICIENT;
        // }

        let up_start = keyboard_input.any_just_pressed([KeyCode::W, KeyCode::Up, KeyCode::Space]);
        let up_held = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up, KeyCode::Space]);
        // let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

        let x_axis = (-(left as i8) + right as i8) as f32 * CC_WALK_ACCEL;

        let mut y_axis = if up_start && output.grounded {
            // JUMP
            player.jump_start = time.elapsed_seconds();
            CC_JUMP_ACCEL
        } else if up_held && time.elapsed_seconds() - CC_JUMP_MAX_DURATION < player.jump_start {
            CC_JUMP_ACCEL
                * (1. - (time.elapsed_seconds() - player.jump_start) / CC_JUMP_MAX_DURATION)
                    .powf(CC_JUMP_FALLOFF_EXPONENT)
        } else {
            0.
        };

        if !output.grounded {
            y_axis -= CC_GRAVITY;
        }

        acc.0 = Vec2::new(x_axis, y_axis);

        if output.grounded && vel.0.y < 0. {
            vel.0.y = 0.;
            acc.0.y = 0.;
        }

        vel.0 += acc.0;

        if vel.0.x > CC_WALK_SPEED {
            vel.0.x = CC_WALK_SPEED;
        }

        if vel.0.x < -CC_WALK_SPEED {
            vel.0.x = -CC_WALK_SPEED;
        }

        // println!("V: {:?} | A: {:?}", vel.0, acc.0);

        controller.translation = Some(vel.0);
    }
}

fn camera_follow_player(
    mut camera_transform: Query<&mut Transform, With<Camera2d>>,
    query: Query<&GlobalTransform, With<Player>>,
) {
    // set camera translation to player translation
    for player_transform in &query {
        camera_transform.single_mut().translation = Vec3::new(
            player_transform.translation().x + 50.,
            -20.,
            // player_transform.translation().y + 25.,
            0.,
        )
    }
}

fn player_fall_out(mut commands: Commands, query: Query<&Transform, With<Player>>) {
    for transfrorm in &query {
        if transfrorm.translation.y < -80. {
            commands.insert_resource(NextState(GameState::Dead));
        }
    }
}
