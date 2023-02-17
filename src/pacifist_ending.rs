use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{SpriteColorLens, TransformPositionLens},
    Animator, Delay, EaseFunction, Tween, TweenCompleted,
};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::NextState,
};

use crate::{
    back_to_enum,
    cutscene::{dialogue_text, Dummy, OrthographicProjectionScaleLens, TransformTranslationXLens},
    level::{create_box, FLOOR_0, FLOOR_0_BOTTOM},
    player::PLAYER_RADIUS,
    states::GameState,
    util::despawn_with,
    CameraScale, TextureHandles,
};

back_to_enum! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PacifistEndingProgress {
        Start = 0,
        CameraZoomIn,
        SpeechLine1,
        SpeechLine2,
        SpeechLine3,
        SpeechLine4,
        ActorAnimation,
        CameraZoomOut,
    }
}

#[derive(Component)]
pub struct PacifistEndingTag;

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

pub struct PacifistEndingPlugin;

impl Plugin for PacifistEndingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(PacifistEndingProgress::Start)
            .add_enter_system(GameState::PacifistEnding, start)
            .add_enter_system(PacifistEndingProgress::CameraZoomIn, camera_zoom_in)
            .add_enter_system(PacifistEndingProgress::SpeechLine1, speech_line_1)
            .add_enter_system(PacifistEndingProgress::SpeechLine2, speech_line_2)
            .add_enter_system(PacifistEndingProgress::SpeechLine3, speech_line_3)
            .add_enter_system(PacifistEndingProgress::SpeechLine4, speech_line_4)
            .add_enter_system(PacifistEndingProgress::ActorAnimation, actor_animation)
            .add_enter_system(PacifistEndingProgress::CameraZoomOut, camera_zoom_out)
            .add_system(cutscene_controller.run_in_state(GameState::PacifistEnding));
    }
}

fn cutscene_controller(mut commands: Commands, mut q_ev: EventReader<TweenCompleted>) {
    for ev in q_ev.iter() {
        let i = ev.user_data;
        match i.try_into() {
            Ok(PacifistEndingProgress::Start) => {
                commands.insert_resource(NextState(PacifistEndingProgress::CameraZoomIn))
            }
            Ok(PacifistEndingProgress::CameraZoomIn) => {
                commands.insert_resource(NextState(PacifistEndingProgress::SpeechLine1))
            }
            Ok(PacifistEndingProgress::SpeechLine1) => {
                commands.insert_resource(NextState(PacifistEndingProgress::SpeechLine2))
            }
            Ok(PacifistEndingProgress::SpeechLine2) => {
                commands.insert_resource(NextState(PacifistEndingProgress::SpeechLine3))
            }
            Ok(PacifistEndingProgress::SpeechLine3) => {
                commands.insert_resource(NextState(PacifistEndingProgress::SpeechLine4))
            }
            Ok(PacifistEndingProgress::SpeechLine4) => {
                commands.insert_resource(NextState(PacifistEndingProgress::ActorAnimation))
            }
            Ok(PacifistEndingProgress::ActorAnimation) => {
                commands.insert_resource(NextState(PacifistEndingProgress::CameraZoomOut))
            }
            Ok(PacifistEndingProgress::CameraZoomOut) => {
                commands.insert_resource(NextState(GameState::EndScreen))
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
        .insert(PacifistEndingTag)
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
        .insert(PacifistEndingTag)
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

    commands.entity(b1).insert(PacifistEndingTag);

    ev_w.send(TweenCompleted {
        entity: player,
        user_data: PacifistEndingProgress::Start as u64,
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
        .with_completed_event(PacifistEndingProgress::CameraZoomIn as u64);

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
        PacifistEndingProgress::SpeechLine1 as u64,
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
        "i get it man im mr shyte",
        410.,
        300.,
        asset_server.load("fonts/Chalk-Regular.ttf"),
        PacifistEndingProgress::SpeechLine2 as u64,
    ));

    // player calms down

    // change player expression
    for mut h in q_player_face.iter_mut() {
        *h = texture_handles.char_face_neutral.clone().unwrap();
    }

    // change player color to calm color
    let red_to_yellow = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(0.3),
        SpriteColorLens {
            start: Color::RED,
            end: Color::YELLOW,
        },
    );

    if let Ok(player) = q_player_body.get_single_mut() {
        commands.entity(player).insert(Animator::new(red_to_yellow));
    }
}

fn speech_line_3(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(dialogue_text(
        "coffee?",
        400.,
        300.,
        asset_server.load("fonts/Chalk-Regular.ttf"),
        PacifistEndingProgress::SpeechLine3 as u64,
    ));
}

fn speech_line_4(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(dialogue_text(
        "sure",
        400.,
        700.,
        asset_server.load("fonts/Chalk-Regular.ttf"),
        PacifistEndingProgress::SpeechLine4 as u64,
    ));
}

fn actor_animation(
    q_fuqheed: Query<Entity, With<FuqheedTag>>,
    q_player: Query<Entity, With<PlayerTag>>,
    mut commands: Commands,
) {
    commands.spawn((
        PacifistEndingTag,
        Dummy,
        Animator::new(
            Delay::<Dummy>::new(Duration::from_secs_f32(0.1))
                .with_completed_event(PacifistEndingProgress::ActorAnimation as u64),
        ),
    ));
    // both exit stage right
    let player_move = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs_f32(4.),
        TransformTranslationXLens {
            start: 195.,
            end: 300.,
        },
    );

    let fuqheed_move = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs_f32(4.2),
        TransformTranslationXLens {
            start: 205.,
            end: 310.,
        },
    );

    if let Ok(player) = q_player.get_single() {
        commands.entity(player).insert(Animator::new(player_move));
    }

    if let Ok(fuqheed) = q_fuqheed.get_single() {
        commands.entity(fuqheed).insert(Animator::new(fuqheed_move));
    }
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
        .with_completed_event(PacifistEndingProgress::CameraZoomOut as u64);

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

pub fn despawn_pacifist_ending(commands: Commands, q: Query<Entity, With<PacifistEndingTag>>) {
    despawn_with(commands, q)
}
