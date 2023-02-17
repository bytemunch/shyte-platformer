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
    level::{create_box, FLOOR_0, FLOOR_0_BOTTOM, FLOOR_1},
    player::PLAYER_RADIUS,
    states::GameState,
    util::despawn_with,
    CameraScale, TextureHandles, UiFont,
};

back_to_enum! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum IntroCutsceneProgress {
        Start = 0,
        CameraZoomIn, // fires after initial zoom in
        SpeechLine1,
        SpeechLine2,
        ActorAnimation,
        CameraZoomOut,
    }
}

#[derive(Component)]
struct IntroCutsceneTag;

#[derive(Component)]
struct PlayerTag;

#[derive(Component)]
struct EnemyTag;

#[derive(Component)]
struct PlayerBodyTag;

#[derive(Component)]
struct PlayerFaceTag;

#[derive(Component)]
struct EnemyFaceTag;

pub struct IntroCutscenePlugin;

impl Plugin for IntroCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(IntroCutsceneProgress::Start)
            .add_enter_system(GameState::IntroCutscene, start)
            .add_enter_system(IntroCutsceneProgress::CameraZoomIn, camera_zoom_in)
            .add_enter_system(IntroCutsceneProgress::SpeechLine1, speech_line_1)
            .add_enter_system(IntroCutsceneProgress::SpeechLine2, speech_line_2)
            .add_enter_system(IntroCutsceneProgress::ActorAnimation, actor_animation)
            .add_enter_system(IntroCutsceneProgress::CameraZoomOut, camera_zoom_out)
            .add_exit_system(GameState::IntroCutscene, despawn_intro_cutscene)
            .add_system(cutscene_controller.run_in_state(GameState::IntroCutscene));
    }
}

fn cutscene_controller(mut commands: Commands, mut q_ev: EventReader<TweenCompleted>) {
    // todo macro this? there's some way of automating this surely
    for ev in q_ev.iter() {
        let i = ev.user_data;
        match i.try_into() {
            // intro
            Ok(IntroCutsceneProgress::Start) => {
                commands.insert_resource(NextState(IntroCutsceneProgress::CameraZoomIn))
            }
            Ok(IntroCutsceneProgress::CameraZoomIn) => {
                commands.insert_resource(NextState(IntroCutsceneProgress::SpeechLine1))
            }
            Ok(IntroCutsceneProgress::SpeechLine1) => {
                commands.insert_resource(NextState(IntroCutsceneProgress::SpeechLine2))
            }
            Ok(IntroCutsceneProgress::SpeechLine2) => {
                commands.insert_resource(NextState(IntroCutsceneProgress::ActorAnimation))
            }
            Ok(IntroCutsceneProgress::ActorAnimation) => {
                commands.insert_resource(NextState(IntroCutsceneProgress::CameraZoomOut))
            }
            Ok(IntroCutsceneProgress::CameraZoomOut) => {
                commands.insert_resource(NextState(GameState::InGame))
            }
            Err(_) => println!("error"),
        }
    }
}

const ZOOM_IN_TIME: f32 = 2.5;
const ZOOM_OUT_TIME: f32 = 0.5;
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
        .insert(IntroCutsceneTag)
        .insert(PlayerTag)
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
                start: Vec3::new(-10., FLOOR_0 + PLAYER_RADIUS, 10.),
                end: Vec3::new(0., FLOOR_0 + PLAYER_RADIUS, 10.),
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
                    color: Color::YELLOW,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            })
            .insert(PlayerBodyTag);

            cb.spawn(SpriteBundle {
                texture: texture_handles.char_face_neutral.clone().unwrap(),
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

    // enemy
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(10., FLOOR_0 + PLAYER_RADIUS, 10.),
            ..default()
        })
        .insert(IntroCutsceneTag)
        .insert(EnemyTag)
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
                    color: Color::GREEN, // todo: pub enum of enemy colors
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
            })
            .insert(EnemyFaceTag);
        });

    // enemy 2
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(25., FLOOR_0 + PLAYER_RADIUS, 10.),
            ..default()
        })
        .insert(IntroCutsceneTag)
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
                    color: Color::BLUE,
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

    // should have vec of tl/br tuples, iterate over and create boxes
    // for now we sidestep the borrow checker this messy awful way :)

    let b1 = create_box(
        &mut commands,
        Vec2::new(-20., 100.),
        Vec2::new(-10., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    let b2 = create_box(
        &mut commands,
        Vec2::new(-10., FLOOR_0),
        Vec2::new(15., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    let b3 = create_box(
        &mut commands,
        Vec2::new(20., FLOOR_0),
        Vec2::new(30., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    let b4 = create_box(
        &mut commands,
        Vec2::new(30., FLOOR_1),
        Vec2::new(50., FLOOR_0_BOTTOM),
        &texture_handles,
        &mut meshes,
        &mut materials,
    );

    commands.entity(b1).insert(IntroCutsceneTag);
    commands.entity(b2).insert(IntroCutsceneTag);
    commands.entity(b3).insert(IntroCutsceneTag);
    commands.entity(b4).insert(IntroCutsceneTag);

    ev_w.send(TweenCompleted {
        entity: player,
        user_data: IntroCutsceneProgress::Start as u64,
    })
}

fn camera_zoom_in(
    mut commands: Commands,
    mut q_camera: Query<Entity, With<Camera2d>>,
    camera_scale: Res<CameraScale>,
) {
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
            start: Vec3::new(20., 0., 0.),
            end: Vec3::new(5., ZOOM_Y_OFFSET, 0.),
        },
    )
    .with_completed_event(IntroCutsceneProgress::CameraZoomIn as u64);

    // set camera transfrom animations
    if let Ok(camera) = q_camera.get_single_mut() {
        commands
            .entity(camera)
            .insert(Animator::new(translate))
            .insert(Animator::new(proj_scale));
    }
}

fn speech_line_1(
    mut commands: Commands,
    ui_font: Res<UiFont>,
    q_player_transform: Query<&Transform, With<PlayerTag>>,
    camera_scale: Res<CameraScale>,
) {
    let t = q_player_transform.single();
    commands.spawn(dialogue_text(
        "hello im mr shyte",
        t.translation.x,
        t.translation.y,
        ui_font.0.clone(),
        IntroCutsceneProgress::SpeechLine1 as u64,
        camera_scale.0,
    ));
}

fn speech_line_2(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,

    mut q_enemy_face: Query<&mut Handle<Image>, With<EnemyFaceTag>>,
    ui_font: Res<UiFont>,

    q_enemy_transform: Query<&Transform, With<EnemyTag>>,
    camera_scale: Res<CameraScale>,
) {
    let t = q_enemy_transform.single();

    commands.spawn(dialogue_text(
        "loll dumb name",
        t.translation.x,
        t.translation.y,
        ui_font.0.clone(),
        IntroCutsceneProgress::SpeechLine2 as u64,
        camera_scale.0,
    ));

    // change enemy expression
    for mut h in q_enemy_face.iter_mut() {
        *h = texture_handles.char_face_laughing.clone().unwrap();
    }

    // todo laugh particles
}

fn actor_animation(
    mut q_player_face: Query<&mut Handle<Image>, With<PlayerFaceTag>>,
    mut q_player_body: Query<Entity, With<PlayerBodyTag>>,
    texture_handles: Res<TextureHandles>,
    mut commands: Commands,
) {
    let yellow_to_red = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(0.3),
        SpriteColorLens {
            start: Color::YELLOW,
            end: Color::RED,
        },
    )
    .with_completed_event(IntroCutsceneProgress::ActorAnimation as u64);

    if let Ok(player_body) = q_player_body.get_single_mut() {
        commands
            .entity(player_body)
            .insert(Animator::new(yellow_to_red));
    }

    // make player face angry
    for mut h in q_player_face.iter_mut() {
        *h = texture_handles.char_face_angry.clone().unwrap();
    }

    // todo player angry particles?
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
            end: Vec3::new(20., 0., 0.),
            start: Vec3::new(3., ZOOM_Y_OFFSET, 0.),
        },
    )
    .with_completed_event(IntroCutsceneProgress::CameraZoomOut as u64);

    // set camera transfrom animations
    if let Ok(camera) = q_camera.get_single_mut() {
        commands
            .entity(camera)
            .insert(Animator::new(translate))
            .insert(Animator::new(proj_scale));
    }
}

fn despawn_intro_cutscene(commands: Commands, q: Query<Entity, With<IntroCutsceneTag>>) {
    despawn_with(commands, q)
}
