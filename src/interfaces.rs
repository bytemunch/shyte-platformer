use std::process::exit;

use bevy::audio::AudioSink;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::end_screen::mute_bgm;
use crate::states::GameState;
use crate::states::PauseState;
use crate::util::despawn_with;
use crate::BackgroundMusic;
use crate::SoundCollection;
use crate::UiFont;

// TODO a big ol refac here

#[derive(Component)]
struct MenuItem;

#[derive(Component)]
struct DeadItem;

#[derive(Component)]
struct ReplayButton;

#[derive(Component)]
struct MuteButton;

// TODO next: death and retry
#[derive(Component)]
struct MenuButton;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct EditorButton;

#[derive(Component)]
struct QuitButton;

#[derive(Component)]
struct PauseItem;

#[derive(Component)]
struct ResumeButton;
pub struct UserInterfacesPlugin;

impl Plugin for UserInterfacesPlugin {
    fn build(&self, app: &mut App) {
        println!("{:?}", GameState::MainMenu);
        app
            // main menu transitions
            .add_enter_system(GameState::MainMenu, setup_menu)
            .add_enter_system(
                GameState::MainMenu,
                mute_bgm.run_if_resource_exists::<BackgroundMusic>(),
            )
            .add_exit_system(GameState::MainMenu, despawn_with::<MenuItem>)
            // dead transitions
            .add_enter_system(GameState::Dead, setup_dead)
            .add_exit_system(GameState::Dead, despawn_with::<DeadItem>)
            // pause transitions
            .add_enter_system(PauseState::Paused, setup_pause_menu)
            .add_exit_system(PauseState::Paused, despawn_with::<PauseItem>)
            // button systems
            .add_system(quit_button)
            .add_system(replay_button)
            .add_system(menu_button)
            .add_system(editor_button)
            .add_system(pause_resume_button.run_in_state(PauseState::Paused))
            .add_system(pause_mute_button)
            .add_system(play_button.run_in_state(GameState::MainMenu));
    }
}

fn pause_resume_button(
    mut commands: Commands,
    button_query: Query<&Interaction, With<ResumeButton>>,

    audio: Res<Audio>,
    sound_collection: Res<SoundCollection>,
    audio_volume: Res<AudioVolume>,
) {
    for interact in &button_query {
        match *interact {
            Interaction::Clicked => {
                audio.play_with_settings(
                    sound_collection.beep.clone(),
                    PlaybackSettings::ONCE.with_volume(audio_volume.0),
                );

                commands.insert_resource(NextState(PauseState::Running));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[derive(Resource)]
pub struct AudioVolume(pub f32);

fn pause_mute_button(
    mut commands: Commands,
    button_query: Query<&Interaction, With<MuteButton>>,

    // audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
    // sound_collection: Res<SoundCollection>,
    audio_volume: Res<AudioVolume>,
) {
    for interact in &button_query {
        match *interact {
            Interaction::Clicked => {
                // TODO dry, one loop, refac etc
                // TODO debounce
                if audio_volume.0 == 0. {
                    for (_id, sink) in audio_sinks.iter() {
                        sink.set_volume(1.);
                    }

                    commands.insert_resource(AudioVolume(1.));
                } else {
                    for (_id, sink) in audio_sinks.iter() {
                        sink.set_volume(0.);
                    }

                    commands.insert_resource(AudioVolume(0.));
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn menu_button(
    mut commands: Commands,
    button_query: Query<&Interaction, With<MenuButton>>,

    audio: Res<Audio>,
    sound_collection: Res<SoundCollection>,
    audio_volume: Res<AudioVolume>,
) {
    for interact in &button_query {
        match *interact {
            Interaction::Clicked => {
                audio.play_with_settings(
                    sound_collection.beep.clone(),
                    PlaybackSettings::ONCE.with_volume(audio_volume.0),
                );

                commands.insert_resource(NextState(PauseState::Running));
                commands.insert_resource(NextState(GameState::MainMenu));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn setup_menu(mut commands: Commands, ui_font: Res<UiFont>) {
    // text
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "shyte (tm) platformer",
                TextStyle {
                    font: ui_font.0.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,

                position: UiRect {
                    left: Val::Px(50.),
                    top: Val::Px(50.),
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
                    font: ui_font.0.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(MenuItem)
        .insert(PlayButton);

    // commands
    //     .spawn(ButtonBundle {
    //         style: Style {
    //             size: Size::new(Val::Px(150.0), Val::Px(65.0)),
    //             // center button
    //             margin: UiRect::all(Val::Auto),
    //             // horizontally center child text
    //             justify_content: JustifyContent::Center,
    //             // vertically center child text
    //             align_items: AlignItems::Center,
    //             ..default()
    //         },
    //         background_color: Color::rgb(128., 0., 0.).into(),
    //         ..default()
    //     })
    //     .with_children(|parent| {
    //         parent.spawn(TextBundle::from_section(
    //             "EDITOR",
    //             TextStyle {
    //                 font: ui_font.0.clone(),
    //                 font_size: 40.0,
    //                 color: Color::rgb(0.9, 0.9, 0.9),
    //             },
    //         ));
    //     })
    //     .insert(MenuItem)
    //     .insert(EditorButton);

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
                    font: ui_font.0.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(MenuItem)
        .insert(QuitButton);
}

fn play_button(
    mut commands: Commands,
    button_query: Query<&Interaction, With<PlayButton>>,
    audio: Res<Audio>,
    sound_collection: Res<SoundCollection>,
    audio_volume: Res<AudioVolume>,
) {
    for interact in &button_query {
        match *interact {
            Interaction::Clicked => {
                audio.play_with_settings(
                    sound_collection.beep.clone(),
                    PlaybackSettings::ONCE.with_volume(audio_volume.0),
                );
                commands.insert_resource(NextState(GameState::IntroCutscene));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn editor_button(mut commands: Commands, button_query: Query<&Interaction, With<EditorButton>>) {
    for interact in &button_query {
        match *interact {
            Interaction::Clicked => {
                commands.insert_resource(NextState(GameState::LevelEditor));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn setup_dead(mut commands: Commands, ui_font: Res<UiFont>) {
    // text
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "oop u ded",
                TextStyle {
                    font: ui_font.0.clone(),
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
                    font: ui_font.0.clone(),
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
                    font: ui_font.0.clone(),
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
                    font: ui_font.0.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(DeadItem)
        .insert(QuitButton);
}

fn quit_button(
    query: Query<&Interaction, With<QuitButton>>,

    audio: Res<Audio>,
    sound_collection: Res<SoundCollection>,
    audio_volume: Res<AudioVolume>,
) {
    for interaction in &query {
        match *interaction {
            Interaction::Clicked => {
                audio.play_with_settings(
                    sound_collection.beep.clone(),
                    PlaybackSettings::ONCE.with_volume(audio_volume.0),
                );

                exit(0)
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn replay_button(
    mut commands: Commands,
    query: Query<&Interaction, With<ReplayButton>>,

    audio: Res<Audio>,
    sound_collection: Res<SoundCollection>,
    audio_volume: Res<AudioVolume>,
) {
    for interaction in &query {
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(NextState(GameState::InGame));
                commands.insert_resource(NextState(PauseState::Running));
                audio.play_with_settings(
                    sound_collection.beep.clone(),
                    PlaybackSettings::ONCE.with_volume(audio_volume.0),
                );
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn setup_pause_menu(mut commands: Commands, ui_font: Res<UiFont>) {
    // text
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "pausa",
                TextStyle {
                    font: ui_font.0.clone(),
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

    // REUME button
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
                    font: ui_font.0.clone(),
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
                    font: ui_font.0.clone(),
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
                    font: ui_font.0.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(PauseItem)
        .insert(ReplayButton);

    // MUTE button
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
                "MUTE",
                TextStyle {
                    font: ui_font.0.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(PauseItem)
        .insert(MuteButton);

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
                    font: ui_font.0.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(PauseItem)
        .insert(QuitButton);
}
