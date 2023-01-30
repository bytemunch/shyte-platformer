use bevy::prelude::*;
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::NextState,
};

use crate::{states::GameState, util::despawn_with};

#[derive(Component)]
struct IntroCutsceneTag;

#[derive(Component)]
struct ActiveCutsceneTimer(Timer);
pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(test_hello.run_in_state(GameState::IntroCutscene))
            .add_system(check_cutscene_over.run_in_state(GameState::IntroCutscene))
            .add_enter_system(GameState::IntroCutscene, setup_intro_cutscene)
            .add_exit_system(GameState::IntroCutscene, despawn_intro_cutscene);
    }
}

fn setup_intro_cutscene(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    commands
        .spawn(ActiveCutsceneTimer(Timer::from_seconds(
            3.0,
            TimerMode::Once,
        )))
        .insert(IntroCutsceneTag);
}

fn check_cutscene_over(
    mut commands: Commands,
    mut q_timer: Query<&mut ActiveCutsceneTimer>,
    time: Res<Time>,
) {
    if let Ok(mut timer) = q_timer.get_single_mut() {
        if timer.0.finished() {
            commands.insert_resource(NextState(GameState::InGame));
        } else {
            timer.0.tick(time.delta());
        }
    }
}

fn despawn_intro_cutscene(commands: Commands, q: Query<Entity, With<IntroCutsceneTag>>) {
    despawn_with(commands, q)
}

fn test_hello() {
    println!("HELLO");
}
