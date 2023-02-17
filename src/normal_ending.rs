use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{SpriteColorLens, TransformPositionLens},
    Animator, Delay, EaseFunction, EaseMethod, Sequence, Tracks, Tween, TweenCompleted,
};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::NextState,
};

use crate::{
    back_to_enum,
    cutscene::{
        dialogue_text, Dummy, OrthographicProjectionScaleLens, TransformTranslationXLens,
        TransformTranslationYLens,
    },
    level::{create_box, FLOOR_0, FLOOR_0_BOTTOM},
    player::PLAYER_RADIUS,
    states::GameState,
    util::despawn_with,
    TextureHandles, CameraScale,
};

back_to_enum! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NormalEndingProgress {
        Start = 0,
        CameraZoomIn,
        SpeechLine1,
        SpeechLine2,
        ActorAnimation,
        FuqheedJump,
        RemovePlayer,
        CameraZoomOut,
    }
}

#[derive(Component)]
pub struct NormalEndingTag;

#[derive(Component)]
struct PlayerTag;

#[derive(Component)]
struct PlayerBodyTag;

#[derive(Component)]
struct PlayerFaceTag;

#[derive(Component)]
struct FuqheedTag;

#[derive(Component)]
struct FuqheedFaceTag;

#[derive(Component)]
struct FuqheedBodyTag;

pub struct NormalEndingPlugin;

impl Plugin for NormalEndingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(NormalEndingProgress::Start)
            .add_enter_system(GameState::NormalEnding, start)
            .add_enter_system(NormalEndingProgress::CameraZoomIn, camera_zoom_in)
            .add_enter_system(NormalEndingProgress::SpeechLine1, speech_line_1)
            .add_enter_system(NormalEndingProgress::SpeechLine2, speech_line_2)
            .add_enter_system(NormalEndingProgress::ActorAnimation, actor_animation)
            .add_enter_system(NormalEndingProgress::FuqheedJump, fuqheed_jump)
            .add_enter_system(NormalEndingProgress::RemovePlayer, remove_player)
            .add_enter_system(NormalEndingProgress::CameraZoomOut, camera_zoom_out)
            .add_system(cutscene_controller.run_in_state(GameState::NormalEnding));
    }
}

fn cutscene_controller(mut commands: Commands, mut q_ev: EventReader<TweenCompleted>) {
    for ev in q_ev.iter() {
        let i = ev.user_data;
        match i.try_into() {
            Ok(NormalEndingProgress::Start) => {
                commands.insert_resource(NextState(NormalEndingProgress::CameraZoomIn))
            }
            Ok(NormalEndingProgress::CameraZoomIn) => {
                commands.insert_resource(NextState(NormalEndingProgress::SpeechLine1))
            }
            Ok(NormalEndingProgress::SpeechLine1) => {
                commands.insert_resource(NextState(NormalEndingProgress::SpeechLine2))
            }
            Ok(NormalEndingProgress::SpeechLine2) => {
                commands.insert_resource(NextState(NormalEndingProgress::ActorAnimation))
            }
            Ok(NormalEndingProgress::ActorAnimation) => {
                commands.insert_resource(NextState(NormalEndingProgress::FuqheedJump))
            }
            Ok(NormalEndingProgress::FuqheedJump) => {
                commands.insert_resource(NextState(NormalEndingProgress::RemovePlayer))
            }
            Ok(NormalEndingProgress::RemovePlayer) => {
                commands.insert_resource(NextState(NormalEndingProgress::CameraZoomOut))
            }
            Ok(NormalEndingProgress::CameraZoomOut) => {
                commands.insert_resource(NextState(GameState::EndScreen))
            }
            Err(_) => println!("error"),
        }
    }
}

const ZOOM_IN_TIME: f32 = 2.5;
const ZOOM_OUT_TIME: f32 = 1.;
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
        .insert(NormalEndingTag)
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
        .insert(FuqheedTag)
        .insert(NormalEndingTag)
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

    commands.entity(b1).insert(NormalEndingTag);

    ev_w.send(TweenCompleted {
        entity: player,
        user_data: NormalEndingProgress::Start as u64,
    })
}

fn camera_zoom_in(
    mut commands: Commands,
    mut q_camera: Query<(Entity, &Transform), With<Camera2d>>,
    camera_scale: Res<CameraScale>,
) {
    if let Ok((camera, transform)) = q_camera.get_single_mut() {
        let proj_scale = Tween::new(
            EaseFunction::QuadraticOut,
            Duration::from_secs_f32(ZOOM_IN_TIME),
            OrthographicProjectionScaleLens {
                start: camera_scale.0,
                end: camera_scale.0 * ZOOM_FACTOR,
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
        .with_completed_event(NormalEndingProgress::CameraZoomIn as u64);

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
        NormalEndingProgress::SpeechLine1 as u64,
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
        NormalEndingProgress::SpeechLine2 as u64,
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
    .with_completed_event(NormalEndingProgress::ActorAnimation as u64);

    if let Ok(player_body) = q_fuqheed_body.get_single_mut() {
        commands
            .entity(player_body)
            .insert(Animator::new(yellow_to_red));
    }

    // make fuqheed face angry
    for mut h in q_fuqheed_face.iter_mut() {
        *h = texture_handles.char_face_angry.clone().unwrap();
    }
}

const JUMP_APEX: f32 = 2.3;

fn fuqheed_jump(mut commands: Commands, mut q_fuqheed: Query<Entity, With<FuqheedTag>>) {
    let right_to_left = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(0.6),
        TransformTranslationXLens {
            start: 205.,
            end: 195.,
        },
    );

    let jump_up = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(0.3),
        TransformTranslationYLens {
            start: FLOOR_0 + PLAYER_RADIUS,
            end: JUMP_APEX,
        },
    );

    let jump_down = Tween::new(
        EaseMethod::Linear,
        Duration::from_secs_f32(0.3),
        TransformTranslationYLens {
            start: JUMP_APEX,
            end: FLOOR_0 + PLAYER_RADIUS * 2.,
        },
    )
    .with_completed_event(NormalEndingProgress::FuqheedJump as u64);

    let bounce_up = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(0.1),
        TransformTranslationYLens {
            start: FLOOR_0 + PLAYER_RADIUS * 2.,
            end: FLOOR_0 + PLAYER_RADIUS * 4.,
        },
    );

    let bounce_down = Tween::new(
        EaseMethod::Linear,
        Duration::from_secs_f32(0.15),
        TransformTranslationYLens {
            start: FLOOR_0 + PLAYER_RADIUS * 4.,
            end: FLOOR_0 + PLAYER_RADIUS,
        },
    );

    let x_seq = Sequence::new([right_to_left]);
    let y_seq = jump_up.then(jump_down).then(bounce_up).then(bounce_down);

    let tracks = Tracks::new([x_seq, y_seq]);

    if let Ok(fuqheed) = q_fuqheed.get_single_mut() {
        commands.entity(fuqheed).insert(Animator::new(tracks));
    }
}

fn remove_player(
    mut commands: Commands,
    q_player: Query<Entity, With<PlayerTag>>,
    mut ev_w: EventWriter<TweenCompleted>,
) {
    if let Ok(player) = q_player.get_single() {
        commands.entity(player).despawn_recursive();
    }

    ev_w.send(TweenCompleted {
        entity: commands.spawn(NormalEndingTag).id(),
        user_data: NormalEndingProgress::RemovePlayer as u64,
    })
}

fn camera_zoom_out(
    mut commands: Commands,
    mut q_camera: Query<Entity, With<Camera2d>>,
    camera_scale: Res<CameraScale>,
) {
    let proj_scale = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(ZOOM_OUT_TIME),
        OrthographicProjectionScaleLens {
            end: camera_scale.0,
            start: camera_scale.0 * ZOOM_FACTOR,
        },
    );
    let translate = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(ZOOM_OUT_TIME),
        TransformPositionLens {
            end: Vec3::new(200., 0., 0.),
            start: Vec3::new(200., ZOOM_Y_OFFSET, 0.),
        },
    );

    let dummy_delay: Delay<Dummy> = Delay::new(Duration::from_secs_f32(0.3))
        .with_completed_event(NormalEndingProgress::CameraZoomOut as u64);

    // set camera transfrom animations
    if let Ok(camera) = q_camera.get_single_mut() {
        commands
            .entity(camera)
            .insert(Dummy)
            .insert(Animator::new(translate))
            .insert(Animator::new(proj_scale))
            .insert(Animator::new(dummy_delay));
    }
}

pub fn despawn_normal_ending(commands: Commands, q: Query<Entity, With<NormalEndingTag>>) {
    despawn_with(commands, q)
}
