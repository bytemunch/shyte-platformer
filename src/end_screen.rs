use std::time::Duration;

use bevy::{audio::AudioSink, prelude::*};
use bevy_tweening::{Animator, EaseFunction, Tween, TweenCompleted};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::NextState,
};

use crate::{
    back_to_enum,
    cutscene::{title_text, BackgroundColorLens},
    genocide_ending::GenocideEndingTag,
    normal_ending::NormalEndingTag,
    pacifist_ending::PacifistEndingTag,
    states::GameState,
    util::despawn_with,
    BackgroundMusic, SoundCollection, UiFont, interfaces::AudioVolume,
};

back_to_enum! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum EndScreenProgress {
        Start = 0,
        FadeToBlack,
        WinTitle,
        WinSubtitle,
        OkButton,
    }
}

#[derive(Resource)]
pub struct Ending(pub Endings);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endings {
    Normal,
    Genocide,
    Pacifist,
}

#[derive(Component)]
struct EndScreenTag;

#[derive(Component)]
struct BigBoxTag;

#[derive(Component)]
struct RootNodeTag;

pub struct EndScreenPlugin;

impl Plugin for EndScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(EndScreenProgress::Start)
            .add_enter_system(GameState::EndScreen, fade_to_black)
            .add_enter_system(EndScreenProgress::WinTitle, win_title)
            .add_enter_system(EndScreenProgress::WinTitle, mute_bgm)
            .add_enter_system(EndScreenProgress::WinSubtitle, win_subtitle)
            .add_enter_system(EndScreenProgress::OkButton, ok_button)
            .add_exit_system(GameState::EndScreen, despawn_with::<EndScreenTag>)
            .add_exit_system(GameState::EndScreen, despawn_with::<NormalEndingTag>)
            .add_exit_system(GameState::EndScreen, despawn_with::<GenocideEndingTag>)
            .add_exit_system(GameState::EndScreen, despawn_with::<PacifistEndingTag>)
            .add_system(ok_button_pressed.run_in_state(GameState::EndScreen))
            .add_system(cutscene_controller.run_in_state(GameState::EndScreen));
    }
}

fn cutscene_controller(mut commands: Commands, mut q_ev: EventReader<TweenCompleted>) {
    // master cutscene controller
    // todo this is gonna get messy, find a nice way of splitting it up?
    for ev in q_ev.iter() {
        let i = ev.user_data;
        match i.try_into() {
            // intro
            Ok(EndScreenProgress::Start) => {
                commands.insert_resource(NextState(EndScreenProgress::FadeToBlack))
            }
            Ok(EndScreenProgress::FadeToBlack) => {
                commands.insert_resource(NextState(EndScreenProgress::WinTitle))
            }
            Ok(EndScreenProgress::WinTitle) => {
                commands.insert_resource(NextState(EndScreenProgress::WinSubtitle))
            }
            Ok(EndScreenProgress::WinSubtitle) => {
                commands.insert_resource(NextState(EndScreenProgress::OkButton))
            }
            Ok(EndScreenProgress::OkButton) => {
                // button system handles interaction
            }
            Err(_) => println!("error"),
        }
    }
}

const FADE_TIME: f32 = 0.5;

fn fade_to_black(mut commands: Commands) {
    let fade_in = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(FADE_TIME),
        BackgroundColorLens {
            start: Color::NONE,
            end: Color::BLACK,
        },
    )
    .with_completed_event(EndScreenProgress::FadeToBlack as u64);

    // spawn screen covering box
    commands
        .spawn(NodeBundle {
            background_color: Color::NONE.into(),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(0.),
                    bottom: Val::Px(0.),
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                },
                ..default()
            },
            ..default()
        })
        .insert(EndScreenTag)
        .insert(Animator::new(fade_in));

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(0.),
                    bottom: Val::Px(0.),
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                },
                ..default()
            },
            ..default()
        })
        .insert(EndScreenTag)
        .insert(RootNodeTag);
}

pub fn mute_bgm(h_bgm: Res<BackgroundMusic>, audio_sinks: Res<Assets<AudioSink>>) {
    if let Some(bgm) = audio_sinks.get(&h_bgm.0) {
        bgm.stop();
    }
}

fn win_title(
    mut commands: Commands,
    ui_font: Res<UiFont>,
    q_root_node: Query<Entity, With<RootNodeTag>>,
) {
    commands.entity(q_root_node.single()).add_children(|cb| {
        cb.spawn(title_text(
            "you win",
            ui_font.0.clone(),
            EndScreenProgress::WinTitle as u64,
            80.,
        ))
        .insert(EndScreenTag);
    });
}

fn win_subtitle(
    mut commands: Commands,
    ending_id: Res<Ending>,
    ui_font: Res<UiFont>,
    q_root_node: Query<Entity, With<RootNodeTag>>,

    audio: Res<Audio>,
    sound_collection: Res<SoundCollection>,
    audio_volume: Res<AudioVolume>,
) {
    audio.play_with_settings(sound_collection.win.clone(), PlaybackSettings::ONCE.with_volume(audio_volume.0));

    let mut ending_type: String = match ending_id.0 {
        Endings::Normal => "normal".to_owned(),
        Endings::Genocide => "genocide".to_owned(),
        Endings::Pacifist => "pacifist".to_owned(),
    };

    ending_type.push_str(" ending");
    commands.entity(q_root_node.single()).add_children(|cb| {
        cb.spawn(title_text(
            ending_type,
            ui_font.0.clone(),
            EndScreenProgress::WinSubtitle as u64,
            60.,
        ))
        .insert(EndScreenTag);
    });
}

#[derive(Component)]
struct OkButton;

fn ok_button(
    mut commands: Commands,
    ui_font: Res<UiFont>,
    q_root_node: Query<Entity, With<RootNodeTag>>,
) {
    // spawn ok button
    commands.entity(q_root_node.single()).add_children(|cb| {
        cb
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(20.)),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "OK",
                TextStyle {
                    font: ui_font.0.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(OkButton)
        .insert(EndScreenTag)
        // .insert(Animation::new(Tween::new(
        //     EaseFunction::QuadraticOut,
        //     Duration::from_secs_f32(0.3),
        //     TODO transparency tween
        // )))
        ;
    });
}

fn ok_button_pressed(
    mut commands: Commands,
    query: Query<&Interaction, With<OkButton>>,
    audio: Res<Audio>,
    sound_collection: Res<SoundCollection>,
    audio_volume: Res<AudioVolume>,
) {
    for interaction in &query {
        match *interaction {
            Interaction::Clicked => {
                audio.play_with_settings(sound_collection.beep.clone(), PlaybackSettings::ONCE.with_volume(audio_volume.0));

                commands.insert_resource(NextState(GameState::MainMenu))
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
