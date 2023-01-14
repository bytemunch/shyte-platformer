use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::SystemOrderLabel;

pub const CC_GRAVITY: f32 = 0.1;
pub const CC_WALK_SPEED: f32 = 0.3;
pub const CC_FRICTION_COEFFICIENT: f32 = 1.2;

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
                    .with_system(
                        kinematic_set_velocity
                            .after(kinematic_gravity)
                            .after(kinematic_apply_friction),
                    )
                    .with_system(
                        kinematic_max_speed
                            .after(kinematic_set_velocity)
                            .before(kinematic_apply_velocity),
                    )
                    .with_system(kinematic_apply_velocity.after(kinematic_set_velocity))
                    .after(SystemOrderLabel::Input),
            );
    }
}

fn kinematic_gravity(
    mut query: Query<
        (
            &mut CCAcceleration,
            &mut CCVelocity,
            &KinematicCharacterControllerOutput,
        ),
        With<KinematicGravity>,
    >,
) {
    for (mut acc, mut vel, output) in &mut query {
        if !output.grounded {
            acc.0.y -= CC_GRAVITY;
        }

        if output.grounded && vel.0.y < 0. {
            vel.0.y = 0.;
            acc.0.y = 0.;
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

fn kinematic_apply_velocity(mut query: Query<(&mut KinematicCharacterController, &CCVelocity)>) {
    for (mut controller, vel) in &mut query {
        controller.translation = Some(vel.0);
    }
}

fn kinematic_max_speed(mut query: Query<&mut CCVelocity>) {
    for mut vel in &mut query {
        if vel.0.x > CC_WALK_SPEED {
            vel.0.x = CC_WALK_SPEED;
        }

        if vel.0.x < -CC_WALK_SPEED {
            vel.0.x = -CC_WALK_SPEED;
        }
    }
}
