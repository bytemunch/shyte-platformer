use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::state::CurrentState;

use crate::{
    enemy::{Enemy, EnemyMover, KillEnemyHitbox, KillPlayerHitbox},
    level::Wall,
    player::Player,
    states::PauseState,
    ActorDead, SystemOrderLabel,
};

// physical constants
pub const CC_GRAVITY: f32 = 0.1;
pub const CC_FRICTION_COEFFICIENT: f32 = 1.2;

pub const PLAYER_WALK_SPEED: f32 = 0.3;

pub const ENEMY_WALK_ACCEL: f32 = 0.05;
pub const ENEMY_WALK_SPEED: f32 = 0.05;

#[derive(Component)]
pub struct KinematicGravity;

#[derive(Component)]
pub struct CCAcceleration(pub Vec2);

#[derive(Component)]
pub struct CCVelocity(pub Vec2);

pub struct KinematicPhysics;

impl Plugin for KinematicPhysics {
    fn build(&self, app: &mut App) {
        app // kinematic systems
            .add_system(kinematic_clear_acceleration.before(SystemOrderLabel::Input))
            .add_system_set(
                SystemSet::new()
                    .label(SystemOrderLabel::Movement)
                    .with_system(kinematic_gravity)
                    .with_system(kinematic_apply_friction)
                    // Collisions
                    .with_system(
                        player_enemy_collision
                            .label(SystemOrderLabel::Collisions)
                            .after(kinematic_gravity),
                    )
                    .with_system(
                        enemy_player_collision
                            .label(SystemOrderLabel::Collisions)
                            .after(kinematic_gravity),
                    )
                    .with_system(
                        player_kill_enemy
                            .label(SystemOrderLabel::Collisions)
                            .after(kinematic_gravity),
                    )
                    .with_system(move_enemies.after(kinematic_gravity))
                    .with_system(
                        enemy_bounce_off_obstacle
                            .label(SystemOrderLabel::Collisions)
                            .after(kinematic_gravity),
                    )
                    .with_system(
                        kinematic_set_velocity
                            .after(move_enemies)
                            .after(kinematic_gravity)
                            .after(SystemOrderLabel::Collisions)
                            .after(kinematic_apply_friction), // .after(collision_test),
                    )
                    .with_system(
                        player_max_speed
                            .after(move_enemies)
                            .after(kinematic_set_velocity)
                            .after(SystemOrderLabel::Collisions)
                            .before(kinematic_apply_velocity),
                    )
                    .with_system(
                        enemy_max_speed
                            .after(move_enemies)
                            .after(kinematic_set_velocity)
                            .after(SystemOrderLabel::Collisions)
                            .before(kinematic_apply_velocity),
                    )
                    .with_system(player_wall_raycast.before(kinematic_set_velocity))
                    .with_system(kinematic_apply_velocity.after(kinematic_set_velocity))
                    .after(SystemOrderLabel::Input),
            );
    }
}

fn player_wall_raycast(
    rapier_context: Res<RapierContext>,
    mut q_player: Query<(Entity, &Transform, &mut CCVelocity, &mut CCAcceleration), With<Player>>,
    q_walls: Query<Entity, With<Wall>>,
) {
    if let Ok((player, player_transform, mut vel, mut acc)) = q_player.get_single_mut() {
        let max_toi = 1.0;
        let ray_origin = Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        );
        let ray_dir = Vec2::new(1.0, 0.0);
        let solid = true;
        let filter = QueryFilter::default().exclude_rigid_body(player);

        let mut callback = |entity, intersection: RayIntersection| {
            if let Ok(_wall) = q_walls.get(entity) {
                let hit_normal = intersection.normal;

                if (vel.0.x > 0. || acc.0.x > 0.) && hit_normal.x < 0. {
                    vel.0.x = 0.;
                    acc.0.x = 0.;
                } else if (vel.0.x < 0. || acc.0.x < 0.) && hit_normal.x > 0. {
                    vel.0.x = 0.;
                    acc.0.x = 0.;
                }
            }

            true
        };

        rapier_context.intersections_with_ray(
            ray_origin,
            ray_dir,
            max_toi,
            solid,
            filter,
            &mut callback,
        );

        rapier_context.intersections_with_ray(
            ray_origin,
            ray_dir * -1.,
            max_toi,
            solid,
            filter,
            callback,
        );
    }
}

/* Read the character controller collisions stored in the character controller’s output. */
fn player_kill_enemy(
    mut commands: Commands,
    mut q_player: Query<(&KinematicCharacterControllerOutput, &mut CCAcceleration), With<Player>>,
    q_attackboxes: Query<(&Parent, Entity), With<KillEnemyHitbox>>,
) {
    for (output, mut acc) in q_player.iter_mut() {
        for collision in &output.collisions {
            if let Ok((parent, _hitbox)) = q_attackboxes.get(collision.entity) {
                // kill enemy
                commands.entity(parent.get()).insert(ActorDead);
                // bounce
                acc.0.y += 0.4;
            }
        }
    }
}

fn player_enemy_collision(
    mut commands: Commands,
    mut q_player: Query<(Entity, &KinematicCharacterControllerOutput), With<Player>>,
    q_killboxes: Query<Entity, With<KillPlayerHitbox>>,
) {
    for (player, output) in q_player.iter_mut() {
        for collision in &output.collisions {
            if let Ok(_killbox) = q_killboxes.get(collision.entity) {
                // kill player
                commands.entity(player).insert(ActorDead);
            }
        }
    }
}

fn enemy_player_collision(
    mut commands: Commands,
    q_player: Query<Entity, With<Player>>,
    q_enemies: Query<&KinematicCharacterControllerOutput, With<KillPlayerHitbox>>,
) {
    for output in q_enemies.iter() {
        for collision in &output.collisions {
            if let Ok(player) = q_player.get(collision.entity) {
                commands.entity(player).insert(ActorDead);
            }
        }
    }
}

fn enemy_bounce_off_obstacle(
    mut q_enemies: Query<(&KinematicCharacterControllerOutput, &mut EnemyMover)>,
) {
    for (output, mut mover) in &mut q_enemies.iter_mut() {
        for collision in &output.collisions {

            let x = collision.toi.normal1.x;
            mover.dir = if x > -1.1 && x < -0.9 {
                -1.
            } else if x < 1.1 && x > 0.9 {
                1.
            } else {
                mover.dir
            }
        }
    }
}

fn move_enemies(mut q: Query<(&mut CCAcceleration, &EnemyMover), With<Enemy>>) {
    for (mut acc, mover) in q.iter_mut() {
        acc.0.x += ENEMY_WALK_ACCEL * mover.dir;
    }
}

fn kinematic_gravity(
    mut query: Query<
        (
            &mut CCAcceleration,
            &mut CCVelocity,
            &KinematicCharacterControllerOutput,
            Option<&Player>,
        ),
        With<KinematicGravity>,
    >,
    pause_state: Res<CurrentState<PauseState>>,
) {
    for (mut acc, mut vel, output, player) in &mut query {
        if pause_state.0 == PauseState::Running {
            if !output.grounded {
                if let Some(player) = player {
                    if player.can_jump.finished() {
                        acc.0.y -= CC_GRAVITY;
                    }
                } else {
                    acc.0.y -= CC_GRAVITY;
                }
            }

            if output.grounded && vel.0.y < 0. {
                vel.0.y = 0.;
                acc.0.y = 0.;
            }
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

fn kinematic_apply_velocity(
    mut query: Query<(&mut KinematicCharacterController, &CCVelocity)>,
    pause_state: Res<CurrentState<PauseState>>,
) {
    for (mut controller, vel) in &mut query {
        if pause_state.0 == PauseState::Running {
            controller.translation = Some(vel.0);
        }
    }
}

fn player_max_speed(mut query: Query<&mut CCVelocity, With<Player>>) {
    for mut vel in &mut query {
        if vel.0.x > PLAYER_WALK_SPEED {
            vel.0.x = PLAYER_WALK_SPEED;
        }

        if vel.0.x < -PLAYER_WALK_SPEED {
            vel.0.x = -PLAYER_WALK_SPEED;
        }
    }
}

fn enemy_max_speed(mut query: Query<&mut CCVelocity, With<Enemy>>) {
    for mut vel in &mut query {
        if vel.0.x > ENEMY_WALK_SPEED {
            vel.0.x = ENEMY_WALK_SPEED;
        }

        if vel.0.x < -ENEMY_WALK_SPEED {
            vel.0.x = -ENEMY_WALK_SPEED;
        }
    }
}
