use bevy::prelude::*;
use std::collections::HashMap;

use super::{foothold::*, render::*};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(collision_system);
    }
}

fn collision_system(
    images: Res<Assets<Image>>,
    sprites: Query<(&Transform, &Handle<Image>), (With<Transform>, With<Sprite>)>,
    mut footholds: Query<(&Foothold, &mut RenderColor), (With<Foothold>, With<RenderColor>)>,
) {
    for (transform, texture) in sprites.iter() {
        let position = transform.translation;
        if let Some(image) = images.get(texture) {
            let width = image.texture_descriptor.size.width as f32;
            let height = image.texture_descriptor.size.height as f32;

            for (foothold, mut render_color) in footholds.iter_mut() {
                let collisions = collide_sprite_foothold(&position, width, height, foothold);
                if !collisions.is_empty() {
                    render_color.color = Color::RED;
                } else {
                    render_color.color = Color::WHITE;
                }
            }
        }
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
