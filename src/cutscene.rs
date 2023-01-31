use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::NextState,
};

use crate::{player::PLAYER_RADIUS, states::GameState, util::despawn_with, TextureHandles};

#[derive(Component)]
struct IntroCutsceneTag;

#[derive(Component)]
struct ActiveCutsceneTimer(Timer);
pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_cutscene_over.run_in_state(GameState::IntroCutscene))
            .add_enter_system(GameState::IntroCutscene, setup_intro_cutscene)
            .add_exit_system(GameState::IntroCutscene, despawn_intro_cutscene);
    }
}

fn setup_intro_cutscene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_handles: Res<TextureHandles>,
) {
    let sprite_size = Some(Vec2::new(PLAYER_RADIUS * 2., PLAYER_RADIUS * 2.));

    commands
        .spawn(TextBundle::from_section(
            "INTRO CUTSCENE",
            TextStyle {
                font: asset_server.load("fonts/Chalk-Regular.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ))
        .insert(IntroCutsceneTag);

    // enemy
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(0., 0., 10.),
            ..default()
        })
        .insert(IntroCutsceneTag)
        .insert(Animator::new(Tween::new(
            // Use a quadratic easing on both endpoints.
            EaseFunction::QuadraticInOut,
            // Animation time (one way only; for ping-pong it takes 2 seconds
            // to come back to start).
            Duration::from_secs(1),
            // The lens gives the Animator access to the Transform component,
            // to animate it. It also contains the start and end values associated
            // with the animation ratios 0. and 1.
            TransformPositionLens {
                start: Vec3::new(0., 0., 10.),
                end: Vec3::new(10., 0., 10.),
            },
        )))
        .with_children(|cb| {
            cb.spawn(SpriteBundle {
                texture: texture_handles.char_outline.clone().unwrap(),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            });

            cb.spawn(SpriteBundle {
                texture: texture_handles.char_body.clone().unwrap(),
                sprite: Sprite {
                    color: Color::GREEN, // todo: const enum of enemy colors
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            });

            cb.spawn(SpriteBundle {
                texture: texture_handles.char_face_neutral.clone().unwrap(),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            });
        });

    commands
        .spawn(ActiveCutsceneTimer(Timer::from_seconds(
            3.0,
            TimerMode::Once,
        )))
        .insert(IntroCutsceneTag);
}

fn check_cutscene_over(
    mut commands: Commands,
    mut q_timer: Query<&mut ActiveCutsceneTimer>,
    time: Res<Time>,
) {
    if let Ok(mut timer) = q_timer.get_single_mut() {
        if timer.0.finished() {
            commands.insert_resource(NextState(GameState::InGame));
        } else {
            timer.0.tick(time.delta());
        }
    }
}

fn despawn_intro_cutscene(commands: Commands, q: Query<Entity, With<IntroCutsceneTag>>) {
    despawn_with(commands, q)
}
