use crate::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RigidBodyPlugin);

        app.add_startup_system(spawn_player);
        app.add_system(player_movement_system)
            .add_system(player_foothold_collision_system);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("player.png");

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 200.0, 0.0),
            texture,
            ..Default::default()
        })
        .insert(Player)
        .insert(RigidBody::default())
        .insert(RenderColor::default());
}

fn player_movement_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &mut RigidBody), (With<RigidBody>, With<Player>)>,
) {
    if player.is_empty() {
        return;
    }
    let (entity, mut body) = player.single_mut();

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
        commands.entity(entity).remove::<FootholdId>();
        body.acceleration.y = JUMP_FORCE;
    }
}

fn player_foothold_collision_system(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    footholds_container: Res<FootholdContainer>,
    mut player: Query<
        (
            Entity,
            &mut Transform,
            &mut RigidBody,
            &Handle<Image>,
            Option<&FootholdId>,
        ),
        (With<Transform>, With<RigidBody>, With<Sprite>, With<Player>),
    >,
    footholds: Query<&Foothold, With<Foothold>>,
) {
    if player.is_empty() {
        return;
    }
    let (entity, mut transform, body, texture, foothold_id) = player.single_mut();

    // Calculate the next position
    let mut next_transform = transform.clone();
    next_transform.translation += Vec3::new(body.velocity.x, body.velocity.y, 0.0)
        + Vec3::new(0.5 * body.acceleration.x, 0.5 * body.acceleration.y, 0.0);

    if let Some(image) = images.get(texture) {
        let width = image.texture_descriptor.size.width as f32;
        let height = image.texture_descriptor.size.height as f32;
        let half_width = width / 2.0;
        let half_height = height / 2.0;

        // Determine if we need to perform collision detection
        let mut use_collision = false;

        // Footold exists: check nodes and update
        if foothold_id.is_some() {
            let curr = foothold_id.unwrap().0;

            if let Some(foothold) = footholds_container.data.get(&curr) {
                if let Some(y) = foothold.get_y_at_x(next_transform.translation.x) {
                    position_limit_ground_y(&mut (next_transform.translation), height, y);
                } else if let Some(y) = container_get_y_at_x(
                    &footholds_container,
                    foothold.prev,
                    next_transform.translation.x,
                ) {
                    println!("fh({}): previous({})", foothold.id, foothold.prev);
                    commands.entity(entity).insert(FootholdId(foothold.prev));
                    position_limit_ground_y(&mut (next_transform.translation), height, y);
                } else if let Some(y) = container_get_y_at_x(
                    &footholds_container,
                    foothold.next,
                    next_transform.translation.x,
                ) {
                    println!("fh({}): next({})", foothold.id, foothold.next);
                    commands.entity(entity).insert(FootholdId(foothold.next));
                    position_limit_ground_y(&mut (next_transform.translation), height, y);
                } else {
                    println!("fh({}): removed", foothold.id);
                    commands.entity(entity).remove::<FootholdId>();
                    use_collision = true;
                }
            }
        } else {
            use_collision = true;
        }

        // Foothold doesn't exist: check for new collisions
        if use_collision {
            for foothold in footholds.iter() {
                let collisions =
                    collide_sprite_foothold(&next_transform.translation, width, height, foothold);

                // Left slope
                if collisions.contains_key(&CollisionType::Left)
                    && collisions.contains_key(&CollisionType::Bottom)
                    && !collisions.contains_key(&CollisionType::Right)
                {
                    if let Some(y) = foothold.get_y_at_x(next_transform.translation.x - half_width)
                    {
                        if (next_transform.translation.y - half_height - y).abs() < 5.0 {
                            println!("fh({}): added", foothold.id);
                            commands.entity(entity).insert(FootholdId(foothold.id));
                            // position_limit_ground_y(&mut (next_transform.translation), height, y);
                        }
                    }
                }

                // Right slope
                if !collisions.contains_key(&CollisionType::Left)
                    && collisions.contains_key(&CollisionType::Right)
                    && collisions.contains_key(&CollisionType::Bottom)
                {
                    if let Some(y) = foothold.get_y_at_x(next_transform.translation.x + half_width)
                    {
                        if (next_transform.translation.y - half_height - y).abs() < 5.0 {
                            println!("fh({}): added", foothold.id);
                            commands.entity(entity).insert(FootholdId(foothold.id));
                            // position_limit_ground_y(&mut (next_transform.translation), height, y);
                        }
                    }
                }

                // Flat ground
                if collisions.contains_key(&CollisionType::Left)
                    && !collisions.contains_key(&CollisionType::Bottom)
                    && collisions.contains_key(&CollisionType::Right)
                {
                    if let Some(y) = foothold.get_y_at_x(next_transform.translation.x) {
                        if (next_transform.translation.y - half_height - y).abs() < 5.0 {
                            println!("fh({}): added", foothold.id);
                            commands.entity(entity).insert(FootholdId(foothold.id));
                            // position_limit_ground_y(&mut (next_transform.translation), height, y);
                        }
                    }
                }
            }
        }

        transform.translation = next_transform.translation;
    }
}

fn position_limit_ground_y(position: &mut Vec3, height: f32, y: f32) {
    let half_height = height / 2.0;
    let bottom_y = position.y - half_height;
    if bottom_y < y && position.y > y {
        position.y = y + half_height;
    }
}

fn container_get_y_at_x(container: &Res<FootholdContainer>, id: u32, x: f32) -> Option<f32> {
    if let Some(foothold) = container.data.get(&id) {
        foothold.get_y_at_x(x)
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CollisionType {
    Top,
    Bottom,
    Left,
    Right,
}

/// Calculates intersection points for a quad and line
pub fn collide_sprite_foothold(
    position: &Vec3,
    width: f32,
    height: f32,
    foothold: &Foothold,
) -> HashMap<CollisionType, Vec2> {
    let mut collisions = HashMap::new();

    let top_left = Vec3::new(position.x - width / 2.0, position.y + height / 2.0, 0.0);
    let top_right = Vec3::new(position.x + width / 2.0, position.y + height / 2.0, 0.0);
    let bottom_left = Vec2::new(position.x - width / 2.0, position.y - height / 2.0);
    let bottom_right = Vec2::new(position.x + width / 2.0, position.y - height / 2.0);
    let bottom_center = Vec2::new(position.x, position.y - height / 2.0);

    if let Some(point) = collide_segment_segment(
        foothold.x1,
        foothold.y1,
        foothold.x2,
        foothold.y2,
        top_left.x,
        top_left.y,
        bottom_left.x,
        bottom_left.y,
    ) {
        collisions.insert(CollisionType::Left, point);
    }

    if let Some(point) = collide_segment_segment(
        foothold.x1,
        foothold.y1,
        foothold.x2,
        foothold.y2,
        top_right.x,
        top_right.y,
        bottom_right.x,
        bottom_right.y,
    ) {
        collisions.insert(CollisionType::Right, point);
    }

    if let Some(point) = collide_segment_segment(
        foothold.x1,
        foothold.y1,
        foothold.x2,
        foothold.y2,
        bottom_left.x,
        bottom_left.y,
        bottom_right.x,
        bottom_right.y,
    ) {
        collisions.insert(CollisionType::Bottom, point);
    }

    if collide_segment_point(
        foothold.x1,
        foothold.y1,
        foothold.x2,
        foothold.y2,
        bottom_left.x,
        bottom_left.y,
    ) {
        collisions.insert(CollisionType::Bottom, bottom_left);
    }

    if collide_segment_point(
        foothold.x1,
        foothold.y1,
        foothold.x2,
        foothold.y2,
        bottom_right.x,
        bottom_right.y,
    ) {
        collisions.insert(CollisionType::Bottom, bottom_right);
    }

    if collide_segment_point(
        foothold.x1,
        foothold.y1,
        foothold.x2,
        foothold.y2,
        bottom_center.x,
        bottom_center.y,
    ) {
        collisions.insert(CollisionType::Bottom, bottom_center);
    }

    collisions
}

/// Calculates the intersection point for 2 lines by their points
/// Input:
///   Line A: (x1, y1) to (x2, y2)
///   Line B: (x3, y3) to (x4, y4)
/// Output:
///   Point: (x, y)
pub fn collide_segment_segment(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) -> Option<Vec2> {
    // calculate the distance to intersection point
    let num_a = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
    let den_a = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    let u_a = num_a / den_a;

    let num_b = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);
    let den_b = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    let u_b = num_b / den_b;

    // if u_a and u_b are between 0.0 and 1.0, lines are colliding
    if u_a >= 0.0 && u_a <= 1.0 && u_b >= 0.0 && u_b <= 1.0 {
        let x = x1 + (u_a * (x2 - x1));
        let y = y1 + (u_a * (y2 - y1));
        Some(Vec2::new(x, y))
    } else {
        None
    }
}

/// Calculates the intersection point for 2 lines by their points
/// Input:
///   Line: (x1, y1) to (x2, y2)
///   Point: (x, y)
/// Output:
///   Bool: True if the point lies on the segment
pub fn collide_segment_point(x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) -> bool {
    // Get the distance between the 2 points on the segment
    let len = Vec2::new(x1, y1).distance(Vec2::new(x2, y2));

    // Get the distance between the 2 points on the segment and the target point
    let d1 = Vec2::new(x, y).distance(Vec2::new(x1, y1));
    let d2 = Vec2::new(x, y).distance(Vec2::new(x2, y2));

    const BUFFER: f32 = 0.1;
    if d1 + d2 >= len - BUFFER && d1 + d2 <= len + BUFFER {
        true
    } else {
        false
    }
}
