use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{player::PLAYER_RADIUS, InGameItem};

#[derive(Component)]
struct StaticEnemy;

// pub struct StaticEnemyPlugin;

// impl Plugin for StaticEnemyPlugin {
//     fn build(&self, app: &mut App) {
//         app
//         .add_system();
//     }
// }

pub fn spawn_static_enemy(
    cb: &mut Commands,
    asset_server: &AssetServer,
    position: Vec3,
) -> Entity {
    let sprite_size = Some(Vec2::new(PLAYER_RADIUS * 2., PLAYER_RADIUS * 2.));
    // Enemy
    cb.spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(PLAYER_RADIUS))
        .insert(StaticEnemy)
        .insert(InGameItem)
        .insert(SpriteBundle {
            texture: asset_server.load("img/character/outline.png"),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: sprite_size,
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, position.z),
            ..default()
        })
        .with_children(|f| {
            f.spawn(SpriteBundle {
                texture: asset_server.load("img/character/body.png"),
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: sprite_size,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });
        })
        .with_children(|f| {
            f.spawn(SpriteBundle {
                texture: asset_server.load("img/character/face_angry.png"),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            });
        })
        .id()
}
