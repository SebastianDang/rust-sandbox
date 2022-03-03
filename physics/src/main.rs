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
    let points: [Point2d; 4] = [
        Point2d { x: -500.0, y: 0.0 },
        Point2d { x: 50.0, y: 0.0 },
        Point2d { x: 100.0, y: 50.0 },
        Point2d { x: 150.0, y: 50.0 },
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

    commands
        .spawn()
        .insert(Line2d::new(-500.0, 0.0, -500.0, 500.0))
        .insert(RenderColor::default());

    commands
        .spawn()
        .insert(Line2d::new(150.0, 50.0, 150.0, 500.0))
        .insert(RenderColor::default());

    commands
        .spawn()
        .insert(Line2d::new(-500.0, 500.0, 150.0, 500.0))
        .insert(RenderColor::default());

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
    next.position += Point2d::new(body.velocity.x, body.velocity.y)
        + Point2d::new(0.5 * body.acceleration.x, 0.5 * body.acceleration.y);

    // Check for collisions and update
    for line in lines.iter_mut() {
        let collisions = collide_quad_line(&next, line);
        if collisions.contains_key(&Collision::Left) || collisions.contains_key(&Collision::Right) {
            if let Some(point) = collisions.get(&Collision::Left) {
                if next.bottom_left().y < point.y {
                    next.position.y = (point.y + (current.height / 2.)).ceil();
                }
            }
            if let Some(point) = collisions.get(&Collision::Right) {
                if next.bottom_right().y < point.y {
                    next.position.y = (point.y + (current.height / 2.)).ceil();
                }
            }
        }
    }

    // Finally, update position of the player
    current.position = next.position;
}
