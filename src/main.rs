mod background;
mod interfaces;
mod level;
mod pause;
mod player;
mod states;
mod static_enemy;
mod util;

use background::BackgroundPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy_parallax::ParallaxCameraComponent;

use interfaces::UserInterfacesPlugin;
use level::LevelPlugin;
use pause::PausePlugin;
use player::PlayerPlugin;
use states::StatesPlugin;

pub const CAMERA_SCALE: f32 = 1. / 24.;

// TODO: look into every-other-flip for parallax plugin, do a PR

// TODO after: enemy OR level loader
// TODO later: Preload assets

#[derive(Component)]
pub struct InGameItem;

#[derive(Component)]
pub struct TextureHandles {
    body: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // setup
        .add_startup_system(setup_graphics)
        // my plugins
        .add_plugin(BackgroundPlugin)
        .add_plugin(StatesPlugin)
        .add_plugin(UserInterfacesPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(LevelPlugin)
        // physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TextureHandles {
        body: asset_server.load("img/character/body.png"),
    });

    // TODO create TextureHandles struct and insert as resource
}

fn setup_graphics(mut commands: Commands) {
    let projection = OrthographicProjection {
        scale: CAMERA_SCALE,
        far: 11.,
        near: -11.,
        ..default()
    };

    // Add a camera so we can see the debug-render.
    commands
        .spawn(Camera2dBundle {
            projection,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        // parallax
        .insert(ParallaxCameraComponent);
}
