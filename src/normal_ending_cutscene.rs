use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{SpriteColorLens, TransformPositionLens},
    Animator, EaseFunction, Tween, TweenCompleted,
};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::NextState,
};

use crate::{
    back_to_enum,
    cutscene::{dialogue_text, OrthographicProjectionScaleLens},
    level::{create_box, FLOOR_0, FLOOR_0_BOTTOM},
    player::PLAYER_RADIUS,
    states::GameState,
    util::despawn_with,
    TextureHandles, CAMERA_SCALE,
};

back_to_enum! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NormalEndingCutsceneProgress {
        Start = 0,
        CameraZoomIn,
        SpeechLine1,
        SpeechLine2,
        ActorAnimation,
        CameraZoomOut,
    }
}

#[derive(Component)]
struct NormalEndingCutsceneTag;

#[derive(Component)]
struct PlayerTag;

#[derive(Component)]
struct PlayerBodyTag;

#[derive(Component)]
struct PlayerFaceTag;

#[derive(Component)]
struct FuqheedFaceTag;

#[derive(Component)]
struct FuqheedBodyTag;

pub struct NormalEndingCutscenePlugin;

impl Plugin for NormalEndingCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::NormalEndingCutscene, start)
            .add_enter_system(NormalEndingCutsceneProgress::CameraZoomIn, camera_zoom_in)
            .add_enter_system(NormalEndingCutsceneProgress::SpeechLine1, speech_line_1)
            .add_enter_system(NormalEndingCutsceneProgress::SpeechLine2, speech_line_2)
            .add_enter_system(
                NormalEndingCutsceneProgress::ActorAnimation,
                actor_animation,
            )
            .add_enter_system(NormalEndingCutsceneProgress::CameraZoomOut, camera_zoom_out)
            .add_exit_system(GameState::NormalEndingCutscene, despawn_intro_cutscene)
            .add_system(cutscene_controller.run_in_state(GameState::NormalEndingCutscene));
    }
}

fn cutscene_controller(mut commands: Commands, mut q_ev: EventReader<TweenCompleted>) {
    // master cutscene controller
    // todo this is gonna get messy, find a nice way of splitting it up?
    for ev in q_ev.iter() {
        let i = ev.user_data;
        match i.try_into() {
            Ok(NormalEndingCutsceneProgress::Start) => {
                commands.insert_resource(NextState(NormalEndingCutsceneProgress::CameraZoomIn))
            }
            Ok(NormalEndingCutsceneProgress::CameraZoomIn) => {
                commands.insert_resource(NextState(NormalEndingCutsceneProgress::SpeechLine1))
            }
            Ok(NormalEndingCutsceneProgress::SpeechLine1) => {
                commands.insert_resource(NextState(NormalEndingCutsceneProgress::SpeechLine2))
            }
            Ok(NormalEndingCutsceneProgress::SpeechLine2) => {
                commands.insert_resource(NextState(NormalEndingCutsceneProgress::ActorAnimation))
            }
            Ok(NormalEndingCutsceneProgress::ActorAnimation) => {
                commands.insert_resource(NextState(NormalEndingCutsceneProgress::CameraZoomOut))
            }
            Ok(NormalEndingCutsceneProgress::CameraZoomOut) => {
                commands.insert_resource(NextState(GameState::MainMenu))
            }
            Err(_) => println!("error"),
        }
    }
}

const ZOOM_IN_TIME: f32 = 2.5;
const ZOOM_OUT_TIME: f32 = 4.5;
const ZOOM_Y_OFFSET: f32 = -5.;
const ZOOM_FACTOR: f32 = 0.5;

fn start(
    mut commands: Commands,

    texture_handles: Res<TextureHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut ev_w: EventWriter<TweenCompleted>,
) {
    let sprite_size = Some(Vec2::new(PLAYER_RADIUS * 2., PLAYER_RADIUS * 2.));

    // player
    let player = commands
        .spawn(SpatialBundle { ..default() })
        .insert(Animator::new(Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            TransformPositionLens {
                start: Vec3::new(190., FLOOR_0 + PLAYER_RADIUS, 10.),
                end: Vec3::new(195., FLOOR_0 + PLAYER_RADIUS, 10.),
            },
        )))
        .insert(NormalEndingCutsceneTag)
        .insert(PlayerTag)
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
                    color: Color::RED,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            })
            .insert(PlayerBodyTag);

            cb.spawn(SpriteBundle {
                texture: texture_handles.char_face_angry.clone().unwrap(),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            })
            .insert(PlayerFaceTag);
        })
        .id();

    // mr fuqheed
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(210., FLOOR_0 + PLAYER_RADIUS, 10.),
            ..default()
        })
        .insert(Animator::new(Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(2),
            TransformPositionLens {
                start: Vec3::new(240., FLOOR_0 + PLAYER_RADIUS, 10.),
                end: Vec3::new(205., FLOOR_0 + PLAYER_RADIUS, 10.),
            },
        )))
        .insert(NormalEndingCutsceneTag)
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
                    color: Color::YELLOW,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            })
            .insert(FuqheedBodyTag);

            cb.spawn(SpriteBundle {
                texture: texture_handles.char_face_neutral.clone().unwrap(),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            })
            .insert(FuqheedFaceTag);
        });

    // floor
    let b1 = create_box(
        &mut commands,
        Vec2::new(188., FLOOR_0),
        Vec2::new(300., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    commands.entity(b1).insert(NormalEndingCutsceneTag);

    ev_w.send(TweenCompleted {
        entity: player,
        user_data: NormalEndingCutsceneProgress::Start as u64,
    })
}

fn camera_zoom_in(
    mut commands: Commands,
    mut q_camera: Query<(Entity, &Transform), With<Camera2d>>,
) {
    if let Ok((camera, transform)) = q_camera.get_single_mut() {
        let proj_scale = Tween::new(
            EaseFunction::QuadraticOut,
            Duration::from_secs_f32(ZOOM_IN_TIME),
            OrthographicProjectionScaleLens {
                start: CAMERA_SCALE,
                end: CAMERA_SCALE * ZOOM_FACTOR,
            },
        );
        let translate = Tween::new(
            EaseFunction::QuadraticOut,
            Duration::from_secs_f32(ZOOM_IN_TIME),
            TransformPositionLens {
                start: transform.translation,
                end: Vec3::new(200., ZOOM_Y_OFFSET, 0.),
            },
        )
        .with_completed_event(NormalEndingCutsceneProgress::CameraZoomIn as u64);

        commands
            .entity(camera)
            .insert(Animator::new(translate))
            .insert(Animator::new(proj_scale));
    }
}

fn speech_line_1(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(dialogue_text(
        "hello im mr fuqheed",
        400.,
        700.,
        asset_server.load("fonts/Chalk-Regular.ttf"),
        NormalEndingCutsceneProgress::SpeechLine1 as u64,
    ));
}

fn speech_line_2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_handles: Res<TextureHandles>,

    mut q_player_face: Query<&mut Handle<Image>, With<PlayerFaceTag>>,
    mut q_player_body: Query<Entity, With<PlayerBodyTag>>,
) {
    commands.spawn(dialogue_text(
        "loll dumb name",
        410.,
        300.,
        asset_server.load("fonts/Chalk-Regular.ttf"),
        NormalEndingCutsceneProgress::SpeechLine2 as u64,
    ));

    // change player expression
    for mut h in q_player_face.iter_mut() {
        *h = texture_handles.char_face_laughing.clone().unwrap();
    }

    // change player color to enemy color
    let red_to_green = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(0.3),
        SpriteColorLens {
            start: Color::RED,
            end: Color::GREEN,
        },
    );

    if let Ok(player) = q_player_body.get_single_mut() {
        commands.entity(player).insert(Animator::new(red_to_green));
    }

    // todo laugh particles
}

fn actor_animation(
    mut q_fuqheed_face: Query<&mut Handle<Image>, With<FuqheedFaceTag>>,
    mut q_fuqheed_body: Query<Entity, With<FuqheedBodyTag>>,
    texture_handles: Res<TextureHandles>,
    mut commands: Commands,
) {
    // mr fuqheed gets angry then jumps on the player, killing him
    let yellow_to_red = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(0.3),
        SpriteColorLens {
            start: Color::YELLOW,
            end: Color::RED,
        },
    )
    .with_completed_event(NormalEndingCutsceneProgress::ActorAnimation as u64);

    if let Ok(player_body) = q_fuqheed_body.get_single_mut() {
        commands
            .entity(player_body)
            .insert(Animator::new(yellow_to_red));
    }

    // make fuqheed face angry
    for mut h in q_fuqheed_face.iter_mut() {
        *h = texture_handles.char_face_angry.clone().unwrap();
    }

    // todo angry particles?

    // todo jump animation
}

fn camera_zoom_out(mut commands: Commands, mut q_camera: Query<Entity, With<Camera2d>>) {
    let proj_scale = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(ZOOM_OUT_TIME),
        OrthographicProjectionScaleLens {
            end: CAMERA_SCALE,
            start: CAMERA_SCALE * ZOOM_FACTOR,
        },
    );
    let translate = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(ZOOM_OUT_TIME),
        TransformPositionLens {
            end: Vec3::new(200., 0., 0.),
            start: Vec3::new(200., ZOOM_Y_OFFSET, 0.),
        },
    )
    .with_completed_event(NormalEndingCutsceneProgress::CameraZoomOut as u64);

    // set camera transfrom animations
    if let Ok(camera) = q_camera.get_single_mut() {
        commands
            .entity(camera)
            .insert(Animator::new(translate))
            .insert(Animator::new(proj_scale));
    }

    // todo fade to black

    // todo "game complete, normal ending" message
}

fn despawn_intro_cutscene(commands: Commands, q: Query<Entity, With<NormalEndingCutsceneTag>>) {
    despawn_with(commands, q)
}
