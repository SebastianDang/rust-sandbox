use bevy::prelude::*;

// Horizontal constants
pub const MOVEMENT_SPEED: f32 = 1.0;
pub const MAX_MOVEMENT_SPEED: f32 = 2.0;
pub const MOVEMENT_FRICTION: f32 = -0.2;

// Vertical constants
pub const JUMP_FORCE: f32 = 4.0;
pub const MAX_JUMP_SPEED: f32 = 4.0;
pub const GRAVITY: f32 = -1.0;
pub const MAX_FALL_SPEED: f32 = -2.0;

#[derive(Component, Default)]
pub struct RigidBody {
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

pub struct RigidBodyPlugin;

impl Plugin for RigidBodyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(rigid_body_system);
    }
}

pub fn rigid_body_system(mut rigid_bodies: Query<&mut RigidBody>) {
    for mut body in rigid_bodies.iter_mut() {
        body.acceleration.y = clamp::clamp(GRAVITY, body.acceleration.y + GRAVITY, JUMP_FORCE);
        body.velocity.y = clamp::clamp(
            MAX_FALL_SPEED,
            body.velocity.y + body.acceleration.y,
            MAX_JUMP_SPEED,
        );

        body.acceleration.x += body.velocity.x * MOVEMENT_FRICTION;
        body.velocity.x = clamp::clamp(
            -MAX_MOVEMENT_SPEED,
            body.velocity.x + body.acceleration.x,
            MAX_MOVEMENT_SPEED,
        );
    }
}
