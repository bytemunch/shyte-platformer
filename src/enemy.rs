use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    kinematic_physics::{CCAcceleration, CCVelocity, KinematicGravity},
    player::PLAYER_RADIUS,
    InGameItem, TextureHandles,
};

#[derive(Bundle)]
struct EnemyBundle {
    pub rb: RigidBody,
    pub collider: Collider,
    pub controller: KinematicCharacterController,
    pub velocity: CCVelocity,
    pub acceleration: CCAcceleration,
    pub sprite_bundle: SpriteBundle,

    _kg: KinematicGravity,
    _e: Enemy,
    _igi: InGameItem,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            rb: RigidBody::KinematicPositionBased,
            collider: Collider::ball(PLAYER_RADIUS),
            controller: KinematicCharacterController {
                apply_impulse_to_dynamic_bodies: true,
                ..default()
            },
            velocity: CCVelocity(Vec2::new(0.0, 0.0)),
            acceleration: CCAcceleration(Vec2::new(0.0, 0.0)),
            sprite_bundle: SpriteBundle { ..default() },
            _e: Enemy,
            _kg: KinematicGravity,
            _igi: InGameItem,
        }
    }
}

#[derive(Component)]
struct StaticEnemy;

#[derive(Component)]
struct Enemy;

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
    cb.spawn(EnemyBundle {
        sprite_bundle: SpriteBundle {
            texture: texture_handles.char_outline.clone(),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: sprite_size,
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
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
