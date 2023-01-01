use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(HelloPlugin)
        .run();
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Grounded(bool);

#[derive(Component)]
struct PhysObj;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("img/stikman.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Velocity(Vec2 { x: (0.), y: (0.) }),
        PhysObj,
        Player,
        Grounded(true),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("img/dvdlogo.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        PhysObj,
        Velocity(Vec2 { x: (0.), y: (0.) }),
    ));
}

fn sprite_move(time: Res<Time>, mut sprite_position: Query<(&mut Velocity, &mut Transform)>) {
    for (mut vel, mut transform) in &mut sprite_position {
        transform.translation.x += vel.0.x * time.delta_seconds();
        transform.translation.y += vel.0.y * time.delta_seconds();

        // Bounds
        if transform.translation.y > 200. {
            // vel.0.y *= -1.;
            transform.translation.y = 200.;
        }

        if transform.translation.y < -200. {
            vel.0.y = 0.;
            transform.translation.y = -200.;
        }

        if transform.translation.x > 200. {
            // vel.0.x *= -1.;
            transform.translation.x = 200.;
        }

        if transform.translation.x < -200. {
            // vel.0.x *= -1.;
            transform.translation.x = -200.;
        }
    }
}

fn gravity(mut phys_obj_vel: Query<&mut Velocity, With<PhysObj>>) {
    for mut vel in &mut phys_obj_vel {
        vel.0.y -= 18.2;
    }
}

fn friction(mut phys_obj_vel: Query<&mut Velocity, With<PhysObj>>) {
    for mut vel in &mut phys_obj_vel {
        vel.0.x *= 0.9;
    }
}

fn ground_set(mut grounds: Query<(&mut Grounded, &Transform)>) {
    for (mut grounded, transform) in &mut grounds {
        if transform.translation.y <= -200. {
            grounded.0 = true;
        }
    }
}

fn keyboard_input(
    kb_input: Res<Input<KeyCode>>,
    mut vels: Query<(&mut Grounded, &mut Velocity, With<Player>)>,
) {
    for (mut grounded, mut vel,_player) in &mut vels {
        if kb_input.pressed(KeyCode::A) {
            vel.0.x = -300.;
        }

        if kb_input.pressed(KeyCode::D) {
            vel.0.x = 300.;
        }

        if kb_input.pressed(KeyCode::W) && grounded.0 {
            vel.0.y = 666.;
            grounded.0 = false;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(keyboard_input)
            .add_system(gravity)
            .add_system(friction)
            .add_system(ground_set)
            .add_system(sprite_move);
    }
}
