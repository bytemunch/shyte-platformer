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
use player::{
    CCAcceleration, CCVelocity, PlayerPlugin, CC_FRICTION_COEFFICIENT, CC_GRAVITY, CC_WALK_SPEED,
};
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

#[derive(Component)]
pub struct KinematicGravity;

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
        // kinematic systems
        .add_system(
            kinematic_clear_acceleration
                .before(kinematic_gravity)
                .before(kinematic_apply_friction),
        )
        .add_system(kinematic_gravity)
        .add_system(kinematic_apply_friction)
        .add_system(
            kinematic_set_velocity
                .after(kinematic_gravity)
                .after(kinematic_apply_friction),
        )
        .add_system(
            kinematic_max_speed
                .after(kinematic_set_velocity)
                .before(kinematic_apply_velocity),
        )
        .add_system(kinematic_apply_velocity.after(kinematic_set_velocity))
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

fn kinematic_gravity(
    mut query: Query<
        (
            &mut CCAcceleration,
            &mut CCVelocity,
            &KinematicCharacterControllerOutput,
        ),
        With<KinematicGravity>,
    >,
) {
    for (mut acc, mut vel, output) in &mut query {
        if !output.grounded {
            acc.0.y -= CC_GRAVITY;
        }

        if output.grounded && vel.0.y < 0. {
            vel.0.y = 0.;
            acc.0.y = 0.;
        }
    }
}

fn kinematic_apply_friction(mut query: Query<&mut CCVelocity>) {
    for mut vel in &mut query {
        vel.0.x /= CC_FRICTION_COEFFICIENT;
    }
}

fn kinematic_clear_acceleration(mut query: Query<&mut CCAcceleration>) {
    for mut acc in &mut query {
        acc.0 = Vec2::new(0.0, 0.0);
    }
}

fn kinematic_set_velocity(mut query: Query<(&CCAcceleration, &mut CCVelocity)>) {
    for (acc, mut vel) in &mut query {
        vel.0 += acc.0;
    }
}

fn kinematic_apply_velocity(mut query: Query<(&mut KinematicCharacterController, &CCVelocity)>) {
    for (mut controller, vel) in &mut query {
        controller.translation = Some(vel.0);
    }
}

fn kinematic_max_speed(mut query: Query<&mut CCVelocity>) {
    for mut vel in &mut query {
        if vel.0.x > CC_WALK_SPEED {
            vel.0.x = CC_WALK_SPEED;
        }

        if vel.0.x < -CC_WALK_SPEED {
            vel.0.x = -CC_WALK_SPEED;
        }
    }
}
