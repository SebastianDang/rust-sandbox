use bevy::prelude::*;

// RigidBody Input
pub const MOVEMENT_SPEED: f32 = 2.0;
pub const JUMP_FORCE: f32 = 5.0;

// Horizontal constants
pub const MAX_MOVEMENT_SPEED: f32 = 3.0;
pub const MOVEMENT_FRICTION: f32 = -0.2;

// Vertical constants
pub const GRAVITY: f32 = -1.5;
pub const MAX_ACCELERATION: f32 = 5.0;
pub const MAX_VELOCITY_DOWN: f32 = -4.0;
pub const MAX_VELOCITY_UP: f32 = 5.0;

/// Represents rigid body properties.
#[derive(Component, Default)]
pub struct RigidBody {
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

/// Plugin for rigid body components.
pub struct RigidBodyPlugin;

impl Plugin for RigidBodyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(rigid_body_system);
    }
}

/// System used for updating rigid body components.
///
/// # Arguments
///
/// * `rigid_bodies`: Rigid body components.
pub fn rigid_body_system(mut rigid_bodies: Query<&mut RigidBody, With<RigidBody>>) {
    for mut body in rigid_bodies.iter_mut() {
        body.acceleration.y =
            (body.acceleration.y + GRAVITY).clamp(-MAX_ACCELERATION, MAX_ACCELERATION);
        body.velocity.y =
            (body.velocity.y + body.acceleration.y).clamp(MAX_VELOCITY_DOWN, MAX_VELOCITY_UP);

        body.acceleration.x += body.velocity.x * MOVEMENT_FRICTION;
        body.velocity.x =
            (body.velocity.x + body.acceleration.x).clamp(-MAX_MOVEMENT_SPEED, MAX_MOVEMENT_SPEED);
    }
}
