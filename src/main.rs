mod background;
mod interfaces;
mod kinematic_physics;
mod level;
mod pause;
mod player;
mod states;
mod enemy;
mod util;

use background::BackgroundPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy_parallax::ParallaxCameraComponent;

use interfaces::UserInterfacesPlugin;
use kinematic_physics::KinematicPhysics;
use level::LevelPlugin;
use pause::PausePlugin;
use player::PlayerPlugin;
use states::StatesPlugin;

pub const CAMERA_SCALE: f32 = 1. / 24.;

// TODO: look into every-other-flip for parallax plugin, do a PR

// TODO: Platform graphics
// TODO: enemy collision, attack and die
// TODO: level loader
// TODO: enemy "ha ha" particle effects
// TODO: animate chalk
// TODO: moving enemies, a la goomba
// TODO: enemies die when past deathplane
// TODO: deathplane as pub const

#[derive(Component)]
pub struct InGameItem;

#[derive(Resource)]
pub struct TextureHandles {
    char_body: Handle<Image>,
    char_outline: Handle<Image>,
    char_face_angry: Handle<Image>,
    char_face_laughing: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // setup
        .add_startup_system(load_textures)
        .add_startup_system(setup_graphics)
        // my plugins
        .add_plugin(BackgroundPlugin)
        .add_plugin(StatesPlugin)
        .add_plugin(UserInterfacesPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(KinematicPhysics)
        // physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(TextureHandles {
        char_body: asset_server.load("img/character/body.png"),
        char_outline: asset_server.load("img/character/outline.png"),
        char_face_angry: asset_server.load("img/character/face_angry.png"),
        char_face_laughing: asset_server.load("img/character/face_laughing.png"),
    });
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
