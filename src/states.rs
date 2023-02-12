use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    LevelEditor,
    Dead,
    IntroCutscene, // TODO cutscene sub state
    NormalEnding,
    PacifistEnding,
    GenocideEnding,
    EndScreen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PauseState {
    Paused,
    Running,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app // states
            .add_loopless_state(GameState::MainMenu)
            .add_loopless_state(PauseState::Running);
    }
}
