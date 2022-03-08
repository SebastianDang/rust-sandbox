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
        .add_system(rigid_body_system)
        .add_system(player_movement_system)
        .add_system(player_collider_system)
        .run();
}

fn setup(mut commands: Commands) {
    let layer_0: [Vec2; 8] = [
        Vec2::new(-1000.0, 300.0),
        Vec2::new(45.0, 300.0),
        Vec2::new(135.0, 240.0),
        Vec2::new(225.0, 180.0),
        Vec2::new(315.0, 120.0),
        Vec2::new(405.0, 60.0),
        Vec2::new(495.0, 0.0),
        Vec2::new(534.0, 0.0),
    ];

    let layer_1: [Vec2; 8] = [
        Vec2::new(96.0, -180.0),
        Vec2::new(96.0, -120.0),
        Vec2::new(186.0, -120.0),
        Vec2::new(186.0, -60.0),
        Vec2::new(276.0, -60.0),
        Vec2::new(276.0, 0.0),
        Vec2::new(534.0, 0.0),
        Vec2::new(1000.0, 0.0),
    ];

    let layer_2: [Vec2; 3] = [
        Vec2::new(-1000.0, -180.0),
        Vec2::new(0.0, -180.0),
        Vec2::new(1000.0, -180.0),
    ];

    spawn_lines_from_points(&mut commands, &layer_0, 0);
    spawn_lines_from_points(&mut commands, &layer_1, 1);
    spawn_lines_from_points(&mut commands, &layer_2, 2);

    commands
        .spawn()
        .insert(Player)
        .insert(Quad2d::new(0.0, 100.0, 20.0, 40.0))
        .insert(RenderColor::default())
        .insert(RigidBody::default());
}

fn spawn_lines_from_points(commands: &mut Commands, points: &[Vec2], _layer: u32) {
    for it in 1..points.len() {
        commands
            .spawn()
            .insert(Line2d::from_points(
                points[it - 1].clone(),
                points[it].clone(),
            ))
            .insert(RenderColor::default());
    }
}

// Horizontal constants
const MOVEMENT_SPEED: f32 = 1.0;
const MAX_MOVEMENT_SPEED: f32 = 2.0;
const MOVEMENT_FRICTION: f32 = -0.2;

// Vertical constants
const JUMP_FORCE: f32 = 4.0;
const MAX_JUMP_SPEED: f32 = 4.0;
const GRAVITY: f32 = -1.0;
const MAX_FALL_SPEED: f32 = -2.0;

#[derive(Component, Default)]
struct RigidBody {
    velocity: Vec2,
    acceleration: Vec2,
}

fn rigid_body_system(mut rigid_bodies: Query<&mut RigidBody>) {
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

#[derive(Component)]
pub struct Player;

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

fn player_collider_system(
    mut player: Query<(&mut Quad2d, &RigidBody), With<Player>>,
    mut lines: Query<&Line2d, Without<Player>>,
) {
    if player.is_empty() {
        return;
    }
    let (mut current, body) = player.single_mut();

    // Calculate the next position
    let mut next = current.clone();
    next.position += Vec2::new(body.velocity.x, body.velocity.y)
        + Vec2::new(0.5 * body.acceleration.x, 0.5 * body.acceleration.y);

    // Check for collisions and update
    for line in lines.iter_mut() {
        // Get the anchor points to compare
        let current_anchor = quad_anchor_point(&current);
        let next_anchor = quad_anchor_point(&next);

        if line_x_in_range(line, current_anchor.x) || line_x_in_range(line, next_anchor.x) {
            let current_line_y = line_y_at_x(line, current_anchor.x);
            let next_line_y = line_y_at_x(line, next_anchor.x);

            if current_anchor.y >= current_line_y && next_anchor.y <= next_line_y {
                quad_set_pos_from_anchor_point(&mut next, None, Some(next_line_y));
            }
        }
    }

    // Finally, update the player
    current.position = next.position;
}

/// Provide an anchor point for the quad
fn quad_anchor_point(quad: &Quad2d) -> Vec2 {
    quad.mid_bottom()
}

/// Set the position using the anchor point for the quad
fn quad_set_pos_from_anchor_point(quad: &mut Quad2d, x: Option<f32>, y: Option<f32>) {
    if x.is_some() {
        quad.position.x = x.unwrap();
    }
    if y.is_some() {
        quad.position.y = (y.unwrap() + (quad.height / 2.)).ceil();
    }
}

/// Given a horizontal flat or sloped line, determine if x is within range
fn line_x_in_range(line: &Line2d, x: f32) -> bool {
    x >= line.p1.x && x <= line.p2.x
}

/// Given a horizontal flat or sloped line, calculate the y coordinate for x coordinate
/// Keep the y coordinate within range of the two points
fn line_y_at_x(line: &Line2d, x: f32) -> f32 {
    // Calculate the slope of the line
    let slope = (line.p2.y - line.p1.y) / (line.p2.x - line.p1.x);

    // Get the range of values for y
    let min = line.p1.y.min(line.p2.y);
    let max = line.p1.y.max(line.p2.y);

    // Calculate the y at x
    let value = line.p2.y + ((x - line.p2.x) * slope);

    // Clamp the values
    if min == max {
        value
    } else {
        clamp::clamp(min, value, max)
    }
}
