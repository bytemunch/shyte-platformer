use bevy::prelude::*;
use bevy_tweening::{TweenCompleted, Lens};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::NextState,
};

use crate::{intro_cutscene::IntroCutsceneProgress, states::GameState};

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        // add all cutscene states
        // run cutscene controller
        app.add_loopless_state(IntroCutsceneProgress::Start)
            .add_system(cutscene_controller.run_in_state(GameState::IntroCutscene));
    }
}

fn cutscene_controller(mut commands: Commands, mut q_ev: EventReader<TweenCompleted>) {
    // master cutscene controller
    // todo this is gonna get messy, find a nice way of splitting it up?
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

// custom lenses
// camera zoom lens
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OrthographicProjectionScaleLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<OrthographicProjection> for OrthographicProjectionScaleLens {
    fn lerp(&mut self, target: &mut OrthographicProjection, ratio: f32) {
        let start = self.start;
        let end = self.end;
        let value = start + (end - start) * ratio;

        target.scale = value;
    }
}

// mad enum dings ty @Shepmaster https://stackoverflow.com/a/57578431
#[macro_export]
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