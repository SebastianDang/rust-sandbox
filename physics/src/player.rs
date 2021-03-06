use bevy::{core::FixedTimestep, prelude::*};

use crate::{foothold::*, quad::*, render::*, rigid_body::*};

const TIMESTEP_60_FRAMES_PER_SECOND: f64 = 1.0 / 60.0;
const TIMESTEP_LABEL: &str = "player_timestep";

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(PlayerState::default())
        .insert(Quad2d::new(0.0, 100.0, 20.0, 40.0))
        .insert(GlobalTransform::default())
        .insert(RigidBody::default())
        .insert(RenderColor::default())
        .insert(Player);
}

#[derive(Clone, Component, Debug)]
pub struct Player;

#[derive(Clone, Component, Debug, Default)]
pub struct PlayerState {
    ground: bool,
}

/// Plugin for players.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(player_transform_sync_system);
        app.add_system(player_movement_system);

        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(
                    FixedTimestep::step(TIMESTEP_60_FRAMES_PER_SECOND).with_label(TIMESTEP_LABEL),
                )
                .with_system(player_collider_system),
        );
    }
}

fn player_transform_sync_system(
    mut players: Query<
        (&mut GlobalTransform, &Quad2d),
        (With<GlobalTransform>, Changed<Quad2d>, With<Player>),
    >,
) {
    for (mut transform, quad) in players.iter_mut() {
        if transform.translation.x != quad.position.x {
            transform.translation.x = quad.position.x;
        }
        if transform.translation.y != quad.position.y {
            transform.translation.y = quad.position.y;
        }
    }
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<
        (&mut PlayerState, &mut RigidBody),
        (With<PlayerState>, With<RigidBody>, With<Player>),
    >,
) {
    if player.is_empty() {
        return;
    }
    let (mut state, mut body) = player.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        body.acceleration.x = -MOVEMENT_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        body.acceleration.x = MOVEMENT_SPEED;
    }

    if !keyboard_input.pressed(KeyCode::Left) && !keyboard_input.pressed(KeyCode::Right) {
        body.acceleration.x = 0.0;
    }

    if keyboard_input.pressed(KeyCode::LAlt) && state.ground {
        state.ground = false;
        body.acceleration.y = JUMP_FORCE;
    }
}

const COLLISION_THRESHOLD: f32 = 4.0;

fn player_collider_system(
    mut commands: Commands,
    mut player: Query<
        (
            Entity,
            &mut PlayerState,
            &mut Quad2d,
            &mut RigidBody,
            Option<&FootholdLayer>,
        ),
        (
            With<PlayerState>,
            With<Quad2d>,
            With<RigidBody>,
            With<Player>,
        ),
    >,
    mut footholds: Query<(&Foothold, &FootholdLayer), (With<Foothold>, With<FootholdLayer>)>,
) {
    if player.is_empty() {
        return;
    }
    let (entity, mut state, mut current, mut body, layer) = player.single_mut();

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
            if let Some(collision) =
                calculate_fh_collision(&foothold, &foothold, current_anchor, next_anchor)
            {
                state.ground = true;
                quad_set_pos_from_anchor_point(&mut next, None, Some(collision.y));
                collisions += 1; // Collision found in this layer
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
            if let Some(collision) =
                calculate_fh_collision(&foothold, &foothold, current_anchor, next_anchor)
            {
                state.ground = true;
                quad_set_pos_from_anchor_point(&mut next, None, Some(collision.y));

                commands.entity(entity).insert(foothold_layer.clone());

                // Apply angular force
                let angle = calculate_fh_angle(foothold, next.position);
                if angle != 0.0 {
                    body.acceleration.x += GRAVITY / angle.tan();
                    body.acceleration.y += GRAVITY;
                }
            }
        }
    }

    // Finally, update the player's position (Only update if changed)
    if current.position != next.position {
        current.position = next.position;
    }
}

/// Calculate any collisions for a foothold, using the current and next points
fn calculate_fh_collision(
    current_fh: &Foothold,
    next_fh: &Foothold,
    current: Vec2,
    next: Vec2,
) -> Option<Vec2> {
    // Get the foothold y position for current and next points
    if let (Some(current_fh_y), Some(next_fh_y)) =
        (current_fh.get_y_at_x(current.x), next_fh.get_y_at_x(next.x))
    {
        // Important: Use this threshold to check for realistic changes in y
        if (current_fh_y - next_fh_y).abs() < COLLISION_THRESHOLD {
            if current.y >= current_fh_y && next.y <= next_fh_y {
                return Some(Vec2::new(next.x, next_fh_y));
            }
        }
    }
    None
}

/// Calculate the angle for a given foothold, using the current point
fn calculate_fh_angle(foothold: &Foothold, current: Vec2) -> f32 {
    if let Some(angle) = foothold.get_angle_at_x(current.x) {
        angle
    } else {
        0.0
    }
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
