use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    kinematic_physics::{CCAcceleration, CCVelocity, KinematicGravity},
    player::PLAYER_RADIUS,
    InGameItem, TextureHandles,
};

#[derive(Component)]
pub struct KillEnemyHitbox;

#[derive(Component)]
pub struct KillPlayerHitbox;

#[derive(Bundle)]
struct EnemyBundle {
    pub rb: RigidBody,
    pub collider: Collider,
    pub controller: KinematicCharacterController,
    pub velocity: CCVelocity,
    pub acceleration: CCAcceleration,
    pub transform_bundle: TransformBundle,

    _kph: KillPlayerHitbox,
    _vb: VisibilityBundle,
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
            transform_bundle: TransformBundle::from_transform(Transform::from_xyz(0., 0., 10.)),
            velocity: CCVelocity(Vec2::new(0.0, 0.0)),
            acceleration: CCAcceleration(Vec2::new(0.0, 0.0)),

            _kph: KillPlayerHitbox,
            _vb: VisibilityBundle::default(),
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
    commands: &mut Commands,
    texture_handles: &TextureHandles,
    position: Vec3,
) -> Entity {
    let sprite_size = Some(Vec2::new(PLAYER_RADIUS * 2., PLAYER_RADIUS * 2.));
    // Enemy
    commands
        .spawn(EnemyBundle {
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(
                position,
            )),
            ..default()
        })
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC)

        .with_children(|cb| {
            // cb.spawn(Collider::ball(PLAYER_RADIUS))
            //     .insert(TransformBundle::from_transform(Transform::from_xyz(
            //         0.0, 0.3, 0.0,
            //     )))
            //     .insert(KillEnemyHitbox);

            cb.spawn(SpriteBundle {
                texture: texture_handles.char_outline.clone(),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            });

            cb.spawn(SpriteBundle {
                texture: texture_handles.char_body.clone(),
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            });

            cb.spawn(SpriteBundle {
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
