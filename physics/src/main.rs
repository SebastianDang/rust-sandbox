use bevy::prelude::*;

mod camera;
use camera::*;

mod geometry;
use geometry::*;

mod collider;
use collider::*;

mod render;
use render::*;

mod foothold;
use foothold::*;

mod rigid_body;
use rigid_body::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(RigidBodyPlugin)
        .add_startup_system(setup)
        .add_startup_system(new_main_camera)
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

    spawn_foothold_from_points(&mut commands, &layer_0, 0);
    spawn_foothold_from_points(&mut commands, &layer_1, 1);
    spawn_foothold_from_points(&mut commands, &layer_2, 2);

    commands
        .spawn()
        .insert(Player)
        .insert(Quad2d::new(0.0, 100.0, 20.0, 40.0))
        .insert(RenderColor::default())
        .insert(RigidBody::default());
}

fn spawn_foothold_from_points(commands: &mut Commands, points: &[Vec2], layer: u32) {
    commands
        .spawn()
        .insert(Foothold::from_points(points))
        .insert(FootholdLayer(layer))
        .insert(RenderColor::default());
}

#[derive(Clone, Component, Debug)]
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

const COLLISION_THRESHOLD: f32 = 4.0;

fn player_collider_system(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Quad2d, &RigidBody, Option<&FootholdLayer>), With<Player>>,
    mut footholds: Query<(&Foothold, &FootholdLayer), (With<Foothold>, With<FootholdLayer>)>,
) {
    if player.is_empty() {
        return;
    }
    let (entity, mut current, body, layer) = player.single_mut();

    // Calculate the next position
    let mut next = current.clone();
    next.position += Vec2::new(body.velocity.x, body.velocity.y)
        + Vec2::new(0.5 * body.acceleration.x, 0.5 * body.acceleration.y);

    // Get the anchor points
    let current_anchor = quad_anchor_point(&current);
    let next_anchor = quad_anchor_point(&next);

    // Keep track of collisions here
    let mut collisions = 0;

    // Check if there is an existing layer
    if layer.is_some() {
        let layer = layer.unwrap();

        // Foothold collision logic
        for (foothold, _) in footholds
            .iter_mut()
            .filter(|(_, foothold_layer)| foothold_layer.0 == layer.0)
        {
            // Determine if the x is in range of this foothold
            if foothold.get_x_in_range(current_anchor.x) && foothold.get_x_in_range(next_anchor.x) {
                // Get the foothold y position for current and next points
                let current_line_y = foothold.get_y_at_x(current_anchor.x);
                let next_line_y = foothold.get_y_at_x(next_anchor.x);

                // If foothold points exist, check for collision
                if current_line_y.is_some() && next_line_y.is_some() {
                    let current_line_y = current_line_y.unwrap();
                    let next_line_y = next_line_y.unwrap();

                    // Important: Use this threshold to check for realistic changes in y
                    if (current_line_y - next_line_y).abs() < COLLISION_THRESHOLD {
                        if current_anchor.y >= current_line_y && next_anchor.y <= next_line_y {
                            quad_set_pos_from_anchor_point(&mut next, None, Some(next_line_y));
                            collisions += 1; // Collision found in this layer
                        }
                    }
                }
            }
        }

        // No collisions. Remove the existing layer
        if collisions == 0 {
            commands.entity(entity).remove::<FootholdLayer>();
        }
    }

    // Check if there was a collision in the existing layer
    if collisions == 0 {
        // Foothold collision logic
        for (foothold, foothold_layer) in footholds.iter_mut() {
            // Determine if the x is in range of this foothold
            if foothold.get_x_in_range(current_anchor.x) && foothold.get_x_in_range(next_anchor.x) {
                // Get the foothold y position for current and next points
                let current_line_y = foothold.get_y_at_x(current_anchor.x);
                let next_line_y = foothold.get_y_at_x(next_anchor.x);

                // If foothold points exist, check for collision
                if current_line_y.is_some() && next_line_y.is_some() {
                    let current_line_y = current_line_y.unwrap();
                    let next_line_y = next_line_y.unwrap();

                    // Important: Use this threshold to check for realistic changes in y
                    if (current_line_y - next_line_y).abs() < COLLISION_THRESHOLD {
                        if current_anchor.y >= current_line_y && next_anchor.y <= next_line_y {
                            quad_set_pos_from_anchor_point(&mut next, None, Some(next_line_y));
                            commands.entity(entity).insert(foothold_layer.clone());
                        }
                    }
                }
            }
        }
    }

    // Finally, update the player's position
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
