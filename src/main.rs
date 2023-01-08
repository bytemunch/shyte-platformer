use std::process::exit;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

// TODO next: refactor to seperate files

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
    Dead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PauseState {
    Paused,
    Running,
}

#[derive(Component)]
struct MenuItem;
#[derive(Component)]
struct DeadItem;

#[derive(Component)]
struct ReplayButton;

// TODO next: death and retry
#[derive(Component)]
struct MenuButton;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct QuitButton;

#[derive(Component)]
struct InGameItem;

#[derive(Component)]
struct PauseItem;

#[derive(Component)]
struct ResumeButton;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // setup
        .add_startup_system(setup_graphics)
        // states
        .add_loopless_state(GameState::MainMenu)
        .add_loopless_state(PauseState::Running)
        // main menu transitions
        .add_enter_system(GameState::MainMenu, setup_menu)
        .add_exit_system(GameState::MainMenu, despawn_menu)
        // ingame transitions
        .add_enter_system(GameState::InGame, setup_level)
        .add_exit_system(GameState::InGame, despawn_level)
        // ingame transitions
        .add_enter_system(GameState::Dead, setup_dead)
        .add_exit_system(GameState::Dead, despawn_dead)
        // pause transitions
        .add_enter_system(PauseState::Paused, setup_pause_menu)
        .add_enter_system(PauseState::Paused, pause)
        .add_exit_system(PauseState::Paused, despawn_pause_menu)
        .add_exit_system(PauseState::Paused, unpause)
        // systems
        .add_system(quit_button)
        .add_system(replay_button)
        // pause systems
        .add_system(pause_resume_button.run_in_state(PauseState::Paused))
        .add_system(pause_menu_button.run_in_state(PauseState::Paused))
        .add_system(pause_input)
        // menu systems
        .add_system(play_button.run_in_state(GameState::MainMenu))
        // ingame systems
        .add_system(
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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

fn player_fall_out(mut commands: Commands, query: Query<&Transform, With<Player>>) {
    for transfrorm in &query {
        if transfrorm.translation.y < -80. {
            commands.insert_resource(NextState(GameState::Dead));
        }
    }
}

fn setup_dead(mut commands: Commands, asset_server: Res<AssetServer>) {
    // text
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "oop u ded",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ),
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::FlexStart,
                position: UiRect {
                    left: Val::Percent(35.),
                    top: Val::Px(100.),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .insert(DeadItem);

    // REPLAY button
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(128., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "REPLAY",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(DeadItem)
        .insert(ReplayButton);

    // MENU button
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(128., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "MENU",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(DeadItem)
        .insert(MenuButton);

    // QUIT button
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(128., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "QUIT",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(DeadItem)
        .insert(QuitButton);
}

fn quit_button(query: Query<&Interaction, With<QuitButton>>) {
    for interaction in &query {
        match *interaction {
            Interaction::Clicked => exit(0),
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn replay_button(mut commands: Commands, query: Query<&Interaction, With<ReplayButton>>) {
    for interaction in &query {
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(NextState(GameState::InGame));
                commands.insert_resource(NextState(PauseState::Running));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn setup_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // text
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "pausa",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ),
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::FlexStart,

                position: UiRect {
                    left: Val::Percent(35.),
                    top: Val::Px(100.),
                    ..default()
                },

                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .insert(PauseItem);

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(128., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "RESUME",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(PauseItem)
        .insert(ResumeButton);

    // menu button
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child textbundle
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(128., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "MENU",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(PauseItem)
        .insert(MenuButton);

    // RESET button
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(128., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "RESET",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(PauseItem)
        .insert(ReplayButton);

    // quit button
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(128., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "QUIT",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(PauseItem)
        .insert(QuitButton);
}

fn despawn_pause_menu(commands: Commands, q: Query<Entity, With<PauseItem>>) {
    despawn_with(commands, q);
}

fn despawn_dead(commands: Commands, q: Query<Entity, With<DeadItem>>) {
    despawn_with(commands, q);
}

fn pause_input(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    state: Res<CurrentState<PauseState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        if state.0 == PauseState::Paused {
            commands.insert_resource(NextState(PauseState::Running));
        } else {
            commands.insert_resource(NextState(PauseState::Paused));
        }
    }
}

fn pause(mut rapier: ResMut<RapierConfiguration>) {
    rapier.physics_pipeline_active = false;
}

fn unpause(mut rapier: ResMut<RapierConfiguration>) {
    rapier.physics_pipeline_active = true;
}

fn pause_resume_button(
    mut commands: Commands,
    button_query: Query<&Interaction, With<ResumeButton>>,
) {
    for interact in &button_query {
        match *interact {
            Interaction::Clicked => {
                commands.insert_resource(NextState(PauseState::Running));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn pause_menu_button(mut commands: Commands, button_query: Query<&Interaction, With<MenuButton>>) {
    for interact in &button_query {
        match *interact {
            Interaction::Clicked => {
                commands.insert_resource(NextState(PauseState::Running));
                commands.insert_resource(NextState(GameState::MainMenu));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // text
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "shyte (tm) platformer",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ),
            style: Style {
                size: Size::new(Val::Px(1500.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::FlexStart,

                position: UiRect {
                    left: Val::Percent(35.),
                    top: Val::Px(100.),
                    ..default()
                },

                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .insert(MenuItem);

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(128., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PLAY",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(MenuItem)
        .insert(PlayButton);

    // quit button
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(128., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "QUIT",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(MenuItem)
        .insert(QuitButton);
}

fn play_button(mut commands: Commands, button_query: Query<&Interaction, With<PlayButton>>) {
    for interact in &button_query {
        match *interact {
            Interaction::Clicked => {
                commands.insert_resource(NextState(GameState::InGame));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

/// Despawn all entities with a given component type
fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn despawn_menu(commands: Commands, query: Query<Entity, With<MenuItem>>) {
    despawn_with(commands, query)
}

fn despawn_level(commands: Commands, query: Query<Entity, With<InGameItem>>) {
    despawn_with(commands, query)
}

fn setup_graphics(mut commands: Commands) {
    let projection = OrthographicProjection {
        scale: 1. / 9.,
        ..default()
    };
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle {
        projection,
        ..default()
    });
}

fn setup_level(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
        .insert(InGameItem);

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(
            100.0, 400.0, 0.0,
        )))
        .insert(InGameItem);

    // Player
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(2.))
        .insert(KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
            translation: Some(Vec2::ZERO),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            -100.0, 10.0, 0.0,
        )))
        .insert(CCAcceleration(Vec2::new(0., 0.)))
        .insert(CCVelocity(Vec2::new(0., 0.)))
        .insert(Player { jump_start: 0. })
        .insert(InGameItem);
}

#[derive(Component)]
struct Player {
    jump_start: f32,
}

#[derive(Component)]
struct CCAcceleration(Vec2);

#[derive(Component)]
struct CCVelocity(Vec2);

#[derive(Default)]
struct DebugCount {
    _n: usize,
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

const CC_JUMP_ACCEL: f32 = 1.2;
const CC_JUMP_MAX_DURATION: f32 = 1.;
const CC_JUMP_FALLOFF_EXPONENT: f32 = 12.;
const CC_GRAVITY: f32 = 0.3;

const CC_WALK_SPEED: f32 = 2.3;
const CC_WALK_ACCEL: f32 = 0.1;
const CC_FRICTION_COEFFICIENT: f32 = 1.1;

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
