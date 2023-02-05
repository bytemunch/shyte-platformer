use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    component_animator_system,
    lens::{TextColorLens, TransformPositionLens},
    Animator, Delay, EaseFunction, Lens, Tween, TweenCompleted,
};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::NextState,
};

use crate::{
    level::{create_box, FLOOR_0, FLOOR_0_BOTTOM, FLOOR_1},
    player::PLAYER_RADIUS,
    states::GameState,
    util::despawn_with,
    TextureHandles, CAMERA_SCALE,
};

// mad enum dings ty @Shepmaster https://stackoverflow.com/a/57578431
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<u64> for $name {
            type Error = ();

            fn try_from(v: u64) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u64 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum IntroCutsceneProgress {
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

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(IntroCutsceneProgress::Start)
            .add_system(component_animator_system::<OrthographicProjection>)
            .add_system(intro_cutscene_controller.run_in_state(GameState::IntroCutscene))
            .add_enter_system(GameState::IntroCutscene, start)
            .add_enter_system(IntroCutsceneProgress::CameraZoomIn, camera_zoom_in)
            .add_enter_system(IntroCutsceneProgress::SpeechLine1, speech_line_1)
            .add_enter_system(IntroCutsceneProgress::SpeechLine2, speech_line_2)
            .add_enter_system(IntroCutsceneProgress::ActorAnimation, actor_animation)
            .add_enter_system(IntroCutsceneProgress::CameraZoomOut, camera_zoom_out)
            .add_exit_system(GameState::IntroCutscene, despawn_intro_cutscene);
    }
}

// camera zoom lens
#[derive(Debug, Copy, Clone, PartialEq)]
struct OrthographicProjectionScaleLens {
    start: f32,
    end: f32,
}

impl Lens<OrthographicProjection> for OrthographicProjectionScaleLens {
    fn lerp(&mut self, target: &mut OrthographicProjection, ratio: f32) {
        let start = self.start;
        let end = self.end;
        let value = start + (end - start) * ratio;

        target.scale = value;
    }
}

const TALK_DELAY: f32 = 1.;
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
        })
        .id();

    // enemy
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(10., FLOOR_0 + PLAYER_RADIUS, 10.),
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
                    color: Color::BLUE, // todo: const enum of enemy colors
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

fn camera_zoom_in(mut commands: Commands, mut q_camera: Query<Entity, With<Camera2d>>) {
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
            start: Vec3::new(20., 0., 0.),
            end: Vec3::new(3., ZOOM_Y_OFFSET, 0.),
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

fn speech_line_1(mut commands: Commands, asset_server: Res<AssetServer>) {
    let speech_in = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(0.3),
        TextColorLens {
            start: Color::NONE,
            end: Color::WHITE,
            section: 0,
        },
    );

    let speech_hold: Delay<Text> = Delay::new(Duration::from_secs_f32(TALK_DELAY));

    let speech_out = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(0.3),
        TextColorLens {
            end: Color::NONE,
            start: Color::WHITE,
            section: 0,
        },
    )
    .with_completed_event(IntroCutsceneProgress::SpeechLine1 as u64);

    let speech_seq = speech_in.then(speech_hold).then(speech_out);
    // add tween with end event
    commands
        .spawn(TextBundle::from_section(
            "hello im mr shyte",
            TextStyle {
                font: asset_server.load("fonts/Chalk-Regular.ttf"),
                font_size: 40.0,
                color: Color::rgba(0.9, 0.9, 0.9, 0.),
            },
        ))
        .insert(Animator::new(speech_seq));
}

fn speech_line_2(mut commands: Commands, asset_server: Res<AssetServer>) {
    //todo dry
    let speech_in = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(0.3),
        TextColorLens {
            start: Color::NONE,
            end: Color::WHITE,
            section: 0,
        },
    );

    let speech_hold: Delay<Text> = Delay::new(Duration::from_secs_f32(TALK_DELAY));

    let speech_out = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(0.3),
        TextColorLens {
            end: Color::NONE,
            start: Color::WHITE,
            section: 0,
        },
    )
    .with_completed_event(IntroCutsceneProgress::SpeechLine2 as u64);

    let speech_seq = speech_in.then(speech_hold).then(speech_out);
    // add tween with end event
    commands
        .spawn(TextBundle::from_section(
            "loll dumb name",
            TextStyle {
                font: asset_server.load("fonts/Chalk-Regular.ttf"),
                font_size: 40.0,
                color: Color::rgba(0.9, 0.9, 0.9, 0.),
            },
        ))
        .insert(Animator::new(speech_seq));
}

fn actor_animation(mut ev_w: EventWriter<TweenCompleted>, mut commands: Commands) {
    //todo

    let x = commands.spawn(IntroCutsceneTag);

    ev_w.send(TweenCompleted {
        entity: x.id(),
        user_data: IntroCutsceneProgress::ActorAnimation as u64,
    })
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

fn intro_cutscene_controller(mut commands: Commands, mut q_ev: EventReader<TweenCompleted>) {
    // todo cutscene controller macro
    for ev in q_ev.iter() {
        let i = ev.user_data;
        match i.try_into() {
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

fn despawn_intro_cutscene(commands: Commands, q: Query<Entity, With<IntroCutsceneTag>>) {
    despawn_with(commands, q)
}
