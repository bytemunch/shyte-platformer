use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::{
    prelude::{ConditionHelpers, IntoConditionalSystem},
    state::NextState,
};

use crate::{
    kinematic_physics::{CCAcceleration, CCVelocity, KinematicGravity},
    states::{GameState, PauseState},
    Actor, InGameItem, SystemOrderLabel, TextureHandles,
};

#[derive(Component)]
pub struct Player {
    jump_start: f32,
    can_jump: Timer,
}

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
                .run_in_state(PauseState::Running)
                .label(SystemOrderLabel::Input),
        )
        .add_system(
            detect_player_removed
                .run_in_state(GameState::InGame)
                .run_in_state(PauseState::Running),
        );
    }
}

fn detect_player_removed(mut commands: Commands, removals: RemovedComponents<Player>) {
    for _entity in removals.iter() {
        commands.insert_resource(NextState(GameState::Dead));
    }
}

const PLAYER_JUMP_ACCEL: f32 = 0.4;
const PLAYER_JUMP_MAX_DURATION: f32 = 1.;
const PLAYER_JUMP_FALLOFF_EXPONENT: f32 = 12.;
const PLAYER_WALK_ACCEL: f32 = 0.05;

const PLAYER_COYOTE_TIME: f32 = 0.05;

pub const PLAYER_RADIUS: f32 = 0.8;

pub fn spawn_player(commands: &mut Commands, texture_handles: &TextureHandles, position: Vec3) {
    let sprite_size = Some(Vec2::new(PLAYER_RADIUS * 2., PLAYER_RADIUS * 2.));
    // Player
    commands
        .spawn((
            RigidBody::KinematicPositionBased,
            ActiveHooks::FILTER_CONTACT_PAIRS,
        ))
        .insert(Collider::ball(PLAYER_RADIUS))
        .insert(KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
            translation: Some(Vec2::ZERO),
            filter_groups: Some(CollisionGroups::new(Group::GROUP_2, Group::GROUP_2)),
            ..default()
        })
        .insert(CCAcceleration(Vec2::new(0., 0.)))
        .insert(CCVelocity(Vec2::new(0., 0.)))
        .insert(KinematicGravity)
        .insert(Player {
            jump_start: 0.,
            can_jump: Timer::new(Duration::from_secs_f32(PLAYER_COYOTE_TIME), TimerMode::Once),
        })
        .insert(InGameItem)
        .insert(Actor)
        .insert(SpriteBundle {
            texture: texture_handles.char_outline.clone(),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: sprite_size,
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        })
        .with_children(|f| {
            f.spawn(SpriteBundle {
                texture: texture_handles.char_body.clone(),
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
                texture: texture_handles.char_face_angry.clone(),
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
        &KinematicCharacterControllerOutput,
        &mut CCAcceleration,
        &mut CCVelocity,
        &mut Player,
    )>,
) {
    for (output, mut acc, mut vel, mut player) in &mut player_info {
        let up_start = keyboard_input.any_just_pressed([KeyCode::W, KeyCode::Up, KeyCode::Space]);
        let up_held = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up, KeyCode::Space]);
        // let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

        if output.grounded {
            player.can_jump.reset();
        } else {
            player.can_jump.tick(time.delta());
        }

        let x_axis = (-(left as i8) + right as i8) as f32 * PLAYER_WALK_ACCEL;

        let y_axis = if up_start && !player.can_jump.finished() {
            // JUMP
            player.jump_start = time.elapsed_seconds();
            PLAYER_JUMP_ACCEL
        } else if up_held && time.elapsed_seconds() - PLAYER_JUMP_MAX_DURATION < player.jump_start {
            PLAYER_JUMP_ACCEL
                * (1. - (time.elapsed_seconds() - player.jump_start) / PLAYER_JUMP_MAX_DURATION)
                    .powf(PLAYER_JUMP_FALLOFF_EXPONENT)
        } else {
            0.
        };

        acc.0.x += x_axis;
        acc.0.y = if y_axis != 0. {
            // JUMP
            // extra jump check superfluous but allows expansion elsewhere
            vel.0.y = if vel.0.y < 0. && !player.can_jump.finished() {
                0.
            } else {
                vel.0.y
            };
            player
                .can_jump
                .set_elapsed(Duration::from_secs_f32(PLAYER_COYOTE_TIME));
            y_axis
        } else {
            0.
        }
    }
}

fn camera_follow_player(
    mut camera_transform: Query<&mut Transform, With<Camera2d>>,
    query: Query<&GlobalTransform, With<Player>>,
) {
    // set camera translation to player translation
    for player_transform in &query {
        camera_transform.single_mut().translation = Vec3::new(
            player_transform.translation().x + 20.,
            0.,
            // player_transform.translation().y + 25.,
            0.,
        )
    }
}
