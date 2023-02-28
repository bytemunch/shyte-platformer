mod background;
mod cutscene;
mod end_screen;
mod enemy;
mod genocide_ending;
mod interfaces;
mod intro_cutscene;
mod kinematic_physics;
mod level;
mod level_editor;
mod normal_ending;
mod pacifist_ending;
mod pause;
mod player;
mod states;
mod util;

use background::BackgroundPlugin;
use bevy::audio::AudioSink;
use bevy_embedded_assets::EmbeddedAssetPlugin;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, SamplerDescriptor};
use bevy::render::texture::ImageSampler;
use bevy::window::WindowResized;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_rapier2d::prelude::*;

use bevy_parallax::ParallaxCameraComponent;

use bevy_tweening::TweeningPlugin;
use cutscene::CutscenePlugin;
use end_screen::EndScreenPlugin;
use genocide_ending::GenocideEndingPlugin;
use interfaces::{AudioVolume, UserInterfacesPlugin};
use intro_cutscene::IntroCutscenePlugin;
use kinematic_physics::KinematicPhysics;
use level::LevelPlugin;
use level_editor::LevelEditorPlugin;
use normal_ending::NormalEndingPlugin;
use pacifist_ending::PacifistEndingPlugin;
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
    char_body: Option<Handle<Image>>,
    char_outline: Option<Handle<Image>>,
    char_face_angry: Option<Handle<Image>>,
    char_face_laughing: Option<Handle<Image>>,
    char_face_neutral: Option<Handle<Image>>,
    ha: Option<Handle<Image>>,
    chalk_line_horizontal: Option<Handle<Image>>,
    chalk_box_fill: Option<Handle<Image>>,
    crosshair: Option<Handle<Image>>,
    respect_bar: Option<Handle<Image>>,
    respect_fill: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct SoundCollection {
    jump: Handle<AudioSource>,
    kill: Handle<AudioSource>,
    die: Handle<AudioSource>,
    fall: Handle<AudioSource>,
    beep: Handle<AudioSource>,
    angry: Handle<AudioSource>,
    land: Handle<AudioSource>,
    win: Handle<AudioSource>,
    bgm: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct BackgroundMusic(pub Handle<AudioSink>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum SystemOrderLabel {
    Input,
    Collisions,
    Movement,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // setup
        .add_startup_system(load_textures)
        .add_startup_system(load_sounds)
        .add_startup_system(setup_font)
        .add_startup_system(setup_graphics)
        .add_system_to_stage(CoreStage::PreUpdate, despawn_with::<ActorDead>)
        // testing
        .add_system(fixup_images)
        .add_system(set_scale_from_window)
        // my plugins
        .add_plugin(BackgroundPlugin)
        .add_plugin(StatesPlugin)
        .add_plugin(UserInterfacesPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(KinematicPhysics)
        .add_plugin(LevelEditorPlugin)
        .add_plugin(CutscenePlugin)
        .add_plugin(IntroCutscenePlugin)
        .add_plugin(NormalEndingPlugin)
        .add_plugin(GenocideEndingPlugin)
        .add_plugin(PacifistEndingPlugin)
        .add_plugin(EndScreenPlugin)
        // bevy_tween
        .add_plugin(TweeningPlugin)
        // particles
        .add_plugin(ParticleSystemPlugin)
        // physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

#[derive(Resource)]
struct RepeatX {
    handle: Handle<Image>,
}

#[derive(Resource)]
struct RepeatXY {
    handle: Handle<Image>,
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load images that don't need descriptor mods
    commands.insert_resource(TextureHandles {
        char_body: asset_server.load("img/character/body.png").into(),
        char_outline: asset_server.load("img/character/outline.png").into(),
        char_face_angry: asset_server.load("img/character/face_angry.png").into(),
        char_face_laughing: asset_server.load("img/character/face_laughing.png").into(),
        ha: asset_server.load("img/ha.png").into(),
        crosshair: asset_server.load("img/crosshair.png").into(),
        respect_bar: asset_server.load("img/respect_bar.png").into(),
        respect_fill: asset_server.load("img/respect_bar_fill.png").into(),
        chalk_line_horizontal: None,
        chalk_box_fill: None,
        char_face_neutral: asset_server.load("img/character/face_neutral.png").into(),
    });
    commands.insert_resource(RepeatX {
        handle: asset_server.load("img/chalk_line_horizontal.png"),
    });

    commands.insert_resource(RepeatXY {
        handle: asset_server.load("img/chalk_fill.png"),
    });
}

fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioVolume(1.));

    commands.insert_resource(SoundCollection {
        jump: asset_server.load("sounds/jump.ogg"),
        kill: asset_server.load("sounds/kill.ogg"),
        die: asset_server.load("sounds/die.ogg"),
        fall: asset_server.load("sounds/fall.ogg"),
        beep: asset_server.load("sounds/beep.ogg"),
        angry: asset_server.load("sounds/angry.ogg"),
        land: asset_server.load("sounds/land.ogg"),
        win: asset_server.load("sounds/win.ogg"),
        bgm: asset_server.load("sounds/bgm.ogg"),
    });
}

fn fixup_images(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
    repeat_x: Res<RepeatX>,
    repeat_xy: Res<RepeatXY>,
    mut texture_handles: ResMut<TextureHandles>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                // a texture was just loaded or changed!

                // TODO modularise this shiiieeeeet

                if *handle == repeat_x.handle {
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
                        address_mode_v: AddressMode::ClampToEdge,
                        ..default()
                    });

                    let clh;

                    {
                        let a = &mut assets;
                        clh = a.add(texture);
                    }

                    texture_handles.chalk_line_horizontal = Some(clh);
                } else if *handle == repeat_xy.handle {
                    let mut texture;
                    {
                        let a = &mut assets;
                        texture = a.get_mut(handle).unwrap().clone();
                    }
                    // ^ unwrap is OK, because we know it is loaded now

                    texture.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
                        address_mode_u: AddressMode::MirrorRepeat,
                        address_mode_v: AddressMode::MirrorRepeat,
                        ..default()
                    });

                    let cbf;

                    {
                        let a = &mut assets;
                        cbf = a.add(texture);
                    }

                    texture_handles.chalk_box_fill = Some(cbf);
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

#[derive(Resource)]
pub struct CameraScale(f32);

#[derive(Resource)]
pub struct UiFont(Handle<Font>);

fn setup_font(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(UiFont(asset_server.load("fonts/kindergarten.ttf")));
}

fn set_scale_from_window(
    mut ev: EventReader<WindowResized>,
    mut camera_scale: ResMut<CameraScale>,
    mut projection: Query<&mut OrthographicProjection>,
) {
    for e in ev.iter() {
        camera_scale.0 = 1. / (e.height / 720.) * (1. / 24.);

        projection.single_mut().scale = camera_scale.0;
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.insert_resource(CameraScale(1. / 24.));

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
