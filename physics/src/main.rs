use bevy::prelude::*;

mod camera;
use camera::*;

mod geometry;
use geometry::*;

mod collider;
use collider::*;

mod render;
use render::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(RenderPlugin)
        .add_startup_system(setup)
        .add_startup_system(new_main_camera)
        .add_system(player_movement_system)
        .add_system(player_physics_system)
        .run();
}

fn setup(mut commands: Commands) {
    let points: [Vec2; 4] = [
        Vec2::new(-500.0, 0.0),
        Vec2::new(50.0, 0.0),
        Vec2::new(100.0, 50.0),
        Vec2::new(150.0, 50.0),
    ];

    for (curr, next) in [(0, 1), (1, 2), (2, 3)] {
        commands
            .spawn()
            .insert(Line2d::from_points(
                points[curr].clone(),
                points[next].clone(),
            ))
            .insert(RenderColor::default());
    }

    // Below ground
    commands
        .spawn()
        .insert(Line2d::new(-500.0, -50.0, 500.0, -50.0))
        .insert(RenderColor::default());

    // // Left wall
    // commands
    //     .spawn()
    //     .insert(Line2d::new(-500.0, 0.0, -500.0, 500.0))
    //     .insert(RenderColor::default());

    // // Right wall
    // commands
    //     .spawn()
    //     .insert(Line2d::new(150.0, 50.0, 150.0, 500.0))
    //     .insert(RenderColor::default());

    // // Top
    // commands
    //     .spawn()
    //     .insert(Line2d::new(-500.0, 500.0, 150.0, 500.0))
    //     .insert(RenderColor::default());

    commands
        .spawn()
        .insert(Player)
        .insert(Quad2d::new(0.0, 100.0, 20.0, 40.0))
        .insert(RenderColor::default())
        .insert(RigidBody::default());
}

#[derive(Component)]
pub struct Player;

const MOVEMENT_SPEED: f32 = 1.0;
const MAX_MOVEMENT_SPEED: f32 = 2.0;
const MAX_FALL_SPEED: f32 = -4.0;
const MAX_JUMP_SPEED: f32 = 4.0;
const JUMP_FORCE: f32 = 4.0;
const GRAVITY: f32 = -1.0;

#[derive(Component, Default)]
struct RigidBody {
    velocity: Vec2,
    acceleration: Vec2,
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<&mut RigidBody, With<Player>>,
) {
    if player.is_empty() {
        return;
    }
    let mut body = player.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        body.acceleration.x = -MOVEMENT_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        body.acceleration.x = MOVEMENT_SPEED;
    }

    if !keyboard_input.pressed(KeyCode::Left) && !keyboard_input.pressed(KeyCode::Right) {
        body.acceleration.x = 0.0;
    }

    if keyboard_input.pressed(KeyCode::LAlt) {
        body.acceleration.y = JUMP_FORCE;
    }
}

fn player_physics_system(
    mut player: Query<(&mut Quad2d, &mut RigidBody), With<Player>>,
    mut lines: Query<&Line2d, Without<Player>>,
) {
    if player.is_empty() {
        return;
    }
    let (mut current, mut body) = player.single_mut();

    // Update rigid body
    body.acceleration.y = clamp::clamp(GRAVITY, body.acceleration.y + GRAVITY, JUMP_FORCE);
    body.velocity.y = clamp::clamp(
        MAX_FALL_SPEED,
        body.velocity.y + body.acceleration.y,
        MAX_JUMP_SPEED,
    );

    body.acceleration.x += body.velocity.x * -0.2;
    body.velocity.x = clamp::clamp(
        -MAX_MOVEMENT_SPEED,
        body.velocity.x + body.acceleration.x,
        MAX_MOVEMENT_SPEED,
    );

    // Calculate the next position
    let mut next = current.clone();
    next.position += Vec2::new(body.velocity.x, body.velocity.y)
        + Vec2::new(0.5 * body.acceleration.x, 0.5 * body.acceleration.y);

    // Clone positions for calculations
    let current_clone = current.clone();
    let next_clone = next.clone();

    // Check for collisions and update
    for line in lines.iter_mut().filter(|line| {
        line_y_exists_at_x(line, current_clone.position.x)
            || line_y_exists_at_x(line, next_clone.position.x)
    }) {
        let current_line_y = line_y_at_x(line, current_clone.position.x);
        let next_line_y = line_y_at_x(line, next_clone.position.x);

        let current_anchor = quad_anchor_point(&current_clone);
        let next_anchor = quad_anchor_point(&next_clone);

        if current_anchor.y >= current_line_y && next_anchor.y <= next_line_y {
            next.position.y = (next_line_y + (current.height / 2.)).ceil();
        }
    }

    // Finally, update position of the player
    current.position = next.position;
}

/// Provide an anchor point for the quad
fn quad_anchor_point(quad: &Quad2d) -> Vec2 {
    quad.mid_bottom()
}

/// Given a horizontal flat or sloped line, determine if x is within range
fn line_y_exists_at_x(line: &Line2d, x: f32) -> bool {
    x > line.p0.x && x < line.p1.x
}

/// Given a horizontal flat or sloped line, calculate the y coordinate for x coordinate
fn line_y_at_x(line: &Line2d, x: f32) -> f32 {
    // Calculate the slope of the line
    let slope = (line.p1.y - line.p0.y) / (line.p1.x - line.p0.x);

    // Get the range of values for y
    let min = line.p0.y.min(line.p1.y);
    let max = line.p0.y.max(line.p1.y);

    // Calculate the y at x
    let value = line.p1.y + ((x - line.p1.x) * slope);

    // Clamp the values
    if min == max {
        value
    } else {
        clamp::clamp(min, value, max)
    }
}
