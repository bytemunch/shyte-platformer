use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_startup_system(create_character_controller)
        .add_system(print_ball_altitude)
        .add_system(read_result_system)
        .add_system(read_keyboard)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
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

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn create_character_controller(mut commands: Commands) {
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(50.))
        .insert(KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            -100.0, 10.0, 0.0,
        )));
}

fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in controllers.iter() {
        println!(
            "Entity {:?} moved by {:?} and touches the ground: {:?}",
            entity, output.effective_translation, output.grounded
        );
    }
}

fn read_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<&mut KinematicCharacterController>,
    output: Query<&KinematicCharacterControllerOutput>
) {
    players.single_mut().translation = Some(Vec2::new(0., -0.9));

    if keyboard_input.pressed(KeyCode::A) {
        players.single_mut().translation = Some(Vec2::new(-5., -0.9))
    }
    if keyboard_input.pressed(KeyCode::D) {
        players.single_mut().translation = Some(Vec2::new(5., -0.9))
    }
    if keyboard_input.pressed(KeyCode::Space) && output.single().grounded {
        players.single_mut().translation = Some(Vec2::new(0., 150.))
    }
}
