use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    component_animator_system, lens::TransformPositionLens, Animator, Delay, EaseFunction, Lens,
    Tween, TweenCompleted,
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

#[derive(Component)]
struct IntroCutsceneTag;

#[derive(Component)]
struct ActiveCutsceneTimer(Timer);
pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(component_animator_system::<OrthographicProjection>)
            .add_system(check_cutscene_over.run_in_state(GameState::IntroCutscene))
            .add_enter_system(GameState::IntroCutscene, setup_intro_cutscene)
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

fn setup_intro_cutscene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_handles: Res<TextureHandles>,
    mut q_camera: Query<Entity, With<Camera2d>>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    const TALK_DELAY: f32 = 3.;
    const ZOOM_IN_TIME: f32 = 2.5;
    const ZOOM_OUT_TIME: f32 = 0.5;
    const ZOOM_Y_OFFSET: f32 = -5.;
    const ZOOM_FACTOR: f32 = 0.5;


    let cam_scale_1 = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(ZOOM_IN_TIME),
        OrthographicProjectionScaleLens {
            start: CAMERA_SCALE,
            end: CAMERA_SCALE * ZOOM_FACTOR,
        },
    );

    let cam_scale_2 = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(ZOOM_OUT_TIME),
        OrthographicProjectionScaleLens {
            end: CAMERA_SCALE,
            start: CAMERA_SCALE * ZOOM_FACTOR,
        },
    );

    let cam_translate_1 = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(ZOOM_IN_TIME),
        TransformPositionLens {
            start: Vec3::new(20., 0., 0.),
            end: Vec3::new(3., ZOOM_Y_OFFSET, 0.),
        },
    );

    let cam_translate_2 = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(ZOOM_OUT_TIME),
        TransformPositionLens {
            start: Vec3::new(3., ZOOM_Y_OFFSET, 0.),
            end: Vec3::new(20., 0., 0.),
        },
    )
    .with_completed_event(0); // TODO enum for user_data values

    let cam_scale_talk_delay: Delay<OrthographicProjection> =
        Delay::new(Duration::from_secs_f32(TALK_DELAY));
    let cam_translate_talk_delay: Delay<Transform> =
        Delay::new(Duration::from_secs_f32(TALK_DELAY));

    let cam_seq_translate = cam_translate_1
        .then(cam_translate_talk_delay)
        .then(cam_translate_2);

    let cam_seq_scale = cam_scale_1.then(cam_scale_talk_delay).then(cam_scale_2);

    let cam_animator_translate = Animator::new(cam_seq_translate);
    let cam_animator_scale = Animator::new(cam_seq_scale);

    // set camera transfrom animations
    if let Ok(camera) = q_camera.get_single_mut() {
        commands
            .entity(camera)
            .insert(cam_animator_translate)
            .insert(cam_animator_scale);
    }

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

    // player
    commands
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
        });

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

    commands
        .spawn(ActiveCutsceneTimer(Timer::from_seconds(
            3.0,
            TimerMode::Once,
        )))
        .insert(IntroCutsceneTag);
}

fn check_cutscene_over(mut commands: Commands, mut q_ev: EventReader<TweenCompleted>) {
    for ev in q_ev.iter() {
        if ev.user_data == 0 {
            commands.insert_resource(NextState(GameState::InGame));
        }
    }
}

fn despawn_intro_cutscene(commands: Commands, q: Query<Entity, With<IntroCutsceneTag>>) {
    despawn_with(commands, q)
}
