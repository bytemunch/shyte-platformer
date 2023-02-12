use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, EaseFunction, Tween, TweenCompleted};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::NextState,
};

use crate::{
    back_to_enum,
    cutscene::{title_text, BackgroundColorLens},
    normal_ending::despawn_normal_ending,
    states::GameState,
    util::despawn_with,
    Ending,
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

#[derive(Component)]
struct EndScreenTag;

#[derive(Component)]
struct BigBoxTag;

pub struct EndScreenPlugin;

impl Plugin for EndScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::EndScreen, fade_to_black)
            .add_enter_system(EndScreenProgress::WinTitle, win_title)
            .add_enter_system(EndScreenProgress::WinSubtitle, win_subtitle)
            .add_enter_system(EndScreenProgress::OkButton, ok_button)
            .add_exit_system(GameState::EndScreen, despawn_end_screen)
            .add_exit_system(GameState::EndScreen, despawn_normal_ending)
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

const FADE_TIME: f32 = 1.5;

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
}

fn win_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(title_text(
            "you win",
            200.,
            540.,
            asset_server.load("fonts/Chalk-Regular.ttf"),
            EndScreenProgress::WinTitle as u64,
            80.,
        ))
        .insert(EndScreenTag);
}

fn win_subtitle(mut commands: Commands, asset_server: Res<AssetServer>, ending_id: Res<Ending>) {
    let mut ending_type: String = match ending_id.0 {
        0 => "normal".to_owned(),
        1 => "genocide".to_owned(),
        2 => "pacifist".to_owned(),
        _ => "error".to_owned(),
    };

    ending_type.push_str(" ending");

    commands
        .spawn(title_text(
            ending_type,
            300.,
            540.,
            asset_server.load("fonts/Chalk-Regular.ttf"),
            EndScreenProgress::WinSubtitle as u64,
            60.,
        ))
        .insert(EndScreenTag);
}

#[derive(Component)]
struct OkButton;

fn ok_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn ok button

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(540.),
                    top: Val::Px(540.),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "OK",
                TextStyle {
                    font: asset_server.load("fonts/Chalk-Regular.ttf"),
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
}

fn ok_button_pressed(mut commands: Commands, query: Query<&Interaction, With<OkButton>>) {
    for interaction in &query {
        match *interaction {
            Interaction::Clicked => commands.insert_resource(NextState(GameState::MainMenu)),
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn despawn_end_screen(commands: Commands, q: Query<Entity, With<EndScreenTag>>) {
    despawn_with(commands, q)
}
