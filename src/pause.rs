use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::{CurrentState, NextState},
};

use crate::states::{GameState, PauseState};
pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            // pause transitions
            .add_enter_system(PauseState::Paused, pause_physics)
            .add_exit_system(PauseState::Paused, unpause_physics)
            // pause systems
            .add_system(pause_input.run_not_in_state(GameState::MainMenu));
    }
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

fn pause_physics(mut rapier: ResMut<RapierConfiguration>) {
    rapier.physics_pipeline_active = false;
}

fn unpause_physics(mut rapier: ResMut<RapierConfiguration>) {
    rapier.physics_pipeline_active = true;
}
