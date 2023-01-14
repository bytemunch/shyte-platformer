use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{states::GameState, static_enemy::spawn_static_enemy, util::despawn_with, InGameItem, TextureHandles};
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // ingame transitions
            .add_enter_system(GameState::InGame, setup_level)
            .add_exit_system(GameState::InGame, despawn_level);
    }
}

fn setup_level(mut commands: Commands, texture_handles: Res<TextureHandles>) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -60.0, 0.0)))
        .insert(InGameItem);

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(
            100.0, 400.0, 0.0,
        )))
        .insert(InGameItem);
    // enemy
    spawn_static_enemy(&mut commands, &texture_handles, Vec3::new(10.0, 0.0, 10.0));
    spawn_static_enemy(&mut commands, &texture_handles, Vec3::new(5.0, 0.0, 10.0));
}

fn despawn_level(commands: Commands, query: Query<Entity, With<InGameItem>>) {
    despawn_with(commands, query)
}
