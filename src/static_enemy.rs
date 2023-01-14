use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{player::{PLAYER_RADIUS, CCAcceleration, CCVelocity}, InGameItem, TextureHandles, KinematicGravity};

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
    texture_handles: &TextureHandles,
    position: Vec3,
) -> Entity {
    let sprite_size = Some(Vec2::new(PLAYER_RADIUS * 2., PLAYER_RADIUS * 2.));
    // Enemy
    cb.spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(PLAYER_RADIUS))
        .insert(KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
            translation: Some(Vec2::ZERO),
            ..default()
        })
        .insert(StaticEnemy)
        .insert(InGameItem)
        .insert(CCAcceleration(Vec2::new(0., 0.)))
        .insert(CCVelocity(Vec2::new(0., 0.)))
        .insert(KinematicGravity)
        .insert(SpriteBundle {
            texture: texture_handles.char_outline.clone(),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: sprite_size,
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        })
        .with_children(|f| {
            f.spawn(SpriteBundle {
                texture: texture_handles.char_body.clone(),
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
                texture: texture_handles.char_face_laughing.clone(),
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
