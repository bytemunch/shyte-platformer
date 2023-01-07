use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// TODO next: Higher jump for longer button press

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_startup_system(create_character_controller)
        // anything physicsy
        .add_system_to_stage(CoreStage::PreUpdate, player_movement)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

fn setup_graphics(mut commands: Commands) {
    let projection = OrthographicProjection {
        scale: 1./6.,
        ..default()
    };
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle {
        projection,
        ..default()
    });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(
            100.0, 400.0, 0.0,
        )));
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct CCAcceleration(Vec2);

#[derive(Component)]
struct CCVelocity(Vec2);

#[derive(Default)]
struct DebugCount {
    _n: usize,
}

fn create_character_controller(mut commands: Commands) {
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(2.))
        .insert(KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
            translation: Some(Vec2::ZERO),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            -100.0, 10.0, 0.0,
        )))
        .insert(CCAcceleration(Vec2::new(0., 0.)))
        .insert(CCVelocity(Vec2::new(0., 0.)))
        .insert(Player);
}

const CC_JUMP_HEIGHT: f32 = 4.;
const CC_GRAVITY: f32 = 0.2;
const CC_WALKSPEED: f32 = 1.;
const CC_WALKACCEL: f32 = 0.1;
const CC_FRICTION_COEFFICIENT: f32 = 1.1;

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<
        (
            &mut KinematicCharacterController,
            &KinematicCharacterControllerOutput,
            &mut CCAcceleration,
            &mut CCVelocity,
        ),
        With<Player>,
    >,
) {
    for (mut controller, output, mut acc, mut vel) in &mut player_info {
        // friction
        if output.grounded {
            vel.0.x /= CC_FRICTION_COEFFICIENT;
        }

        let up = keyboard_input.any_just_pressed([KeyCode::W, KeyCode::Up]);
        // let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

        let x_axis = (-(left as i8) + right as i8) as f32 * CC_WALKACCEL;

        let mut y_axis = if up && output.grounded {
            CC_JUMP_HEIGHT
        } else {
            0.
        };

        if !output.grounded {
            y_axis -= CC_GRAVITY;
        }

        acc.0 = Vec2::new(x_axis, y_axis);

        if output.grounded && vel.0.y < 0. {
            vel.0.y = 0.;
            acc.0.y = 0.;
        }

        vel.0 += acc.0;

        if vel.0.x > CC_WALKSPEED {
            vel.0.x = CC_WALKSPEED;
        }

        if vel.0.x < -CC_WALKSPEED {
            vel.0.x = -CC_WALKSPEED;
        }

        println!("V: {:?} | A: {:?}", vel.0, acc.0);

        controller.translation = Some(vel.0);
    }
}
