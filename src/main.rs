mod interfaces;
mod util;
mod player;
mod pause;
mod level;
mod states;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use interfaces::UserInterfacesPlugin;
use level::LevelPlugin;
use pause::PausePlugin;
use player::PlayerPlugin;
use states::StatesPlugin;

// TODO: Background image

#[derive(Component)]
pub struct InGameItem;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // setup
        .add_startup_system(setup_graphics)

        // plugins
        .add_plugin(StatesPlugin)
        // more plugins
        .add_plugin(UserInterfacesPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(LevelPlugin)
        // physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
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