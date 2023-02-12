use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    component_animator_system, lens::TextColorLens, Animator, Delay, EaseFunction, Lens, Tween,
};
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    end_screen::EndScreenProgress, intro_cutscene::IntroCutsceneProgress,
    normal_ending_cutscene::NormalEndingCutsceneProgress,
};

#[derive(Component)]
pub struct Dummy; // allows for custom delayed events, for mid-animation transitions

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        // add all cutscene states
        // run cutscene controller
        app.add_system(component_animator_system::<OrthographicProjection>)
            .add_system(component_animator_system::<BackgroundColor>)
            .add_system(component_animator_system::<Dummy>)
            .add_loopless_state(IntroCutsceneProgress::Start)
            .add_loopless_state(NormalEndingCutsceneProgress::Start)
            .add_loopless_state(EndScreenProgress::Start);
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

// translate x
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformTranslationXLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<Transform> for TransformTranslationXLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let start = self.start;
        let end = self.end;
        let value = start + (end - start) * ratio;

        target.translation.x = value;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformTranslationYLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<Transform> for TransformTranslationYLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let start = self.start;
        let end = self.end;
        let value = start + (end - start) * ratio;

        target.translation.y = value;
    }
}

// style color
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BackgroundColorLens {
    pub start: Color,
    pub end: Color,
}

impl Lens<BackgroundColor> for BackgroundColorLens {
    fn lerp(&mut self, target: &mut BackgroundColor, ratio: f32) {
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();
        let value = start.lerp(end, ratio);
        target.0 = value.into();
    }
}

// tweens

// consts
const TALK_DELAY: f32 = 0.7;

// todo there's gonna be a better way of doing this, i know it
pub fn dialogue_text(
    value: impl Into<String>,
    top: f32,
    left: f32,
    font: Handle<Font>,
    user_data: u64,
) -> (TextBundle, Animator<Text>) {
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
    .with_completed_event(user_data);

    let speech_seq = speech_in.then(speech_hold).then(speech_out);

    (
        TextBundle {
            z_index: ZIndex::Global(10),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(top),
                    left: Val::Px(left),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section(
                value,
                TextStyle {
                    font,
                    font_size: 40.0,
                    color: Color::rgba(0.9, 0.9, 0.9, 0.),
                },
            ),
            ..default()
        },
        Animator::new(speech_seq),
    )
}

pub fn title_text(
    value: impl Into<String>,
    top: f32,
    left: f32,
    font: Handle<Font>,
    user_data: u64,
    font_size: f32,
) -> (TextBundle, Animator<Text>) {
    // todo dry
    let title_in = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs_f32(0.3),
        TextColorLens {
            start: Color::NONE,
            end: Color::WHITE,
            section: 0,
        },
    ).with_completed_event(user_data);

    (
        TextBundle {
            z_index: ZIndex::Global(10),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(top),
                    left: Val::Px(left),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section(
                value,
                TextStyle {
                    font,
                    font_size,
                    color: Color::rgba(0.9, 0.9, 0.9, 0.),
                },
            ),
            ..default()
        },
        Animator::new(title_in),
    )
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
