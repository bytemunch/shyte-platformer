mod background;
mod enemy;
mod interfaces;
mod kinematic_physics;
mod level;
mod pause;
mod player;
mod states;
mod util;
mod level_editor;

use background::BackgroundPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, SamplerDescriptor};
use bevy::render::texture::ImageSampler;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_rapier2d::prelude::*;

use bevy_parallax::ParallaxCameraComponent;

use interfaces::UserInterfacesPlugin;
use kinematic_physics::KinematicPhysics;
use level::LevelPlugin;
use level_editor::LevelEditorPlugin;
use pause::PausePlugin;
use player::PlayerPlugin;
use states::StatesPlugin;
use util::despawn_with;

pub const CAMERA_SCALE: f32 = 1. / 24.;

pub const DEATHPLANE: f32 = -25.;

#[derive(Component)]
pub struct Actor;

#[derive(Component)]
pub struct ActorDead;

#[derive(Component)]
pub struct InGameItem;

#[derive(Resource)]
pub struct TextureHandles {
    char_body: Handle<Image>,
    char_outline: Handle<Image>,
    char_face_angry: Handle<Image>,
    char_face_laughing: Handle<Image>,
    ha: Handle<Image>,
    chalk_line_horizontal: Handle<Image>,
    crosshair: Handle<Image>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum SystemOrderLabel {
    Input,
    Collisions,
    Movement,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // setup
        .add_startup_system(load_textures)
        .add_startup_system(setup_graphics)
        .add_system_to_stage(CoreStage::PreUpdate, remove_dead_actors)
        // testing
        .add_system(fixup_images)
        // my plugins
        .add_plugin(BackgroundPlugin)
        .add_plugin(StatesPlugin)
        .add_plugin(UserInterfacesPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(KinematicPhysics)
        .add_plugin(LevelEditorPlugin)
        // particles
        .add_plugin(ParticleSystemPlugin)
        // physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

#[derive(Resource)]
struct UntiledChalkLine {
    handle: Handle<Image>,
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UntiledChalkLine {
        handle: asset_server.load("img/chalk_line_horizontal.png"),
    });
}

fn fixup_images(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
    my_img: Res<UntiledChalkLine>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                // a texture was just loaded or changed!

                if *handle == my_img.handle {
                    // it is our special map image!

                    // WARNING: this mutable access will cause another
                    // AssetEvent (Modified) to be emitted!
                    let mut texture;
                    {
                        let a = &mut assets;
                        texture = a.get_mut(handle).unwrap().clone();
                    }
                    // ^ unwrap is OK, because we know it is loaded now

                    texture.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
                        address_mode_u: AddressMode::MirrorRepeat,
                        address_mode_v: AddressMode::ClampToBorder,
                        ..default()
                    });

                    let clh;

                    {
                        let a = &mut assets;
                        clh = a.add(texture);
                    }

                    commands.insert_resource(TextureHandles {
                        char_body: asset_server.load("img/character/body.png"),
                        char_outline: asset_server.load("img/character/outline.png"),
                        char_face_angry: asset_server.load("img/character/face_angry.png"),
                        char_face_laughing: asset_server.load("img/character/face_laughing.png"),
                        ha: asset_server.load("img/ha.png"),
                        chalk_line_horizontal: clh,
                        crosshair: asset_server.load("img/crosshair.png"),
                    });
                } else {
                    // it is some other image
                }
            }
            AssetEvent::Modified { handle: _ } => {
                // an image was modified
            }
            AssetEvent::Removed { handle: _ } => {
                // an image was unloaded
            }
        }
    }
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

fn remove_dead_actors(commands: Commands, q: Query<Entity, With<ActorDead>>) {
    despawn_with(commands, q);
}
