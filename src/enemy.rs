use bevy::prelude::*;
use bevy_particle_systems::{
    ColorPoint, JitteredValue, ParticleSpace, ParticleSystem, ParticleSystemBundle,
    ParticleTexture, Playing,
};
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

#[derive(Component)]
pub struct EnemyMover {
    pub dir: f32,
}

impl Default for EnemyMover {
    fn default() -> Self {
        Self { dir: 1. }
    }
}

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
pub struct Enemy;

pub fn spawn_enemy(
    commands: &mut Commands,
    texture_handles: &TextureHandles,
    position: Vec3,
    mover: bool,
) -> Entity {
    let sprite_size = Some(Vec2::new(PLAYER_RADIUS * 2., PLAYER_RADIUS * 2.));

    let mut fade_out = Vec::new();
    fade_out.push(ColorPoint::new(Color::WHITE, 0.));
    fade_out.push(ColorPoint::new(Color::NONE, 1.));

    let enemy_color = if mover { Color::BLUE } else { Color::GREEN };

    // Enemy
    let e = commands
        .spawn(EnemyBundle {
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(
                position,
            )),
            ..default()
        })
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC)
        .with_children(|cb| {
            cb.spawn(ParticleSystemBundle {
                particle_system: ParticleSystem {
                    texture: ParticleTexture::Sprite(texture_handles.ha.clone().unwrap()),
                    spawn_rate_per_second: 4.0.into(),
                    spawn_radius: 1.0.into(),
                    initial_speed: JitteredValue::jittered(1.5, -0.5..0.5),
                    lifetime: JitteredValue::jittered(2., -1.0..1.0),
                    emitter_shape: std::f32::consts::PI,
                    emitter_angle: std::f32::consts::PI / 2.0,
                    looping: true,
                    scale: 0.007.into(),
                    system_duration_seconds: 50.0,
                    initial_rotation: JitteredValue::jittered(0., -0.4..0.4),
                    // z_value_override: Some(JitteredValue::new(10.0)),
                    color: fade_out.into(),
                    space: ParticleSpace::Local,
                    despawn_particles_with_system: true,

                    ..default()
                },
                ..default()
            })
            .insert(Playing);

            cb.spawn(Collider::ball(PLAYER_RADIUS - 0.1))
                .insert(TransformBundle::from_transform(Transform::from_xyz(
                    0.0, 0.3, 0.0,
                )))
                .insert(KillEnemyHitbox);

            cb.spawn(SpriteBundle {
                texture: texture_handles.char_outline.clone().unwrap(),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            });

            cb.spawn(SpriteBundle {
                texture: texture_handles.char_body.clone().unwrap(),
                sprite: Sprite {
                    color: enemy_color,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            });

            cb.spawn(SpriteBundle {
                texture: texture_handles.char_face_laughing.clone().unwrap(),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: sprite_size,
                    ..default()
                },
                ..default()
            });
        })
        .id();

    if mover {
        commands.entity(e).insert(EnemyMover::default());
    }

    e
}
