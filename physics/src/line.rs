use bevy::prelude::*;

/// Represents a line with 2 points
#[derive(Debug, Clone, Component)]
pub struct Line2d {
    pub p1: Vec2,
    pub p2: Vec2,
}

impl Line2d {
    /// Creates a new line from values (x1, y1) to (x2, y2)
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            p1: Vec2::new(x1, y1),
            p2: Vec2::new(x2, y2),
        }
    }

    /// Creates a new line from points p1 to p2
    pub fn from_points(p1: Vec2, p2: Vec2) -> Self {
        Self { p1, p2 }
    }
}
