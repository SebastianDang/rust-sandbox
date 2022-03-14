use bevy::prelude::*;

/// Represents a line with 2 points
#[derive(Debug, Clone, Component)]
pub struct Quad2d {
    pub position: Vec2,
    pub width: f32,
    pub height: f32,
}

impl Quad2d {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            width,
            height,
        }
    }

    pub fn top_left(&self) -> Vec2 {
        Vec2::new(
            self.position.x - (self.width / 2.0),
            self.position.y + (self.height / 2.0),
        )
    }

    pub fn bottom_left(&self) -> Vec2 {
        Vec2::new(
            self.position.x - (self.width / 2.0),
            self.position.y - (self.height / 2.0),
        )
    }

    pub fn top_right(&self) -> Vec2 {
        Vec2::new(
            self.position.x + (self.width / 2.0),
            self.position.y + (self.height / 2.0),
        )
    }

    pub fn bottom_right(&self) -> Vec2 {
        Vec2::new(
            self.position.x + (self.width / 2.0),
            self.position.y - (self.height / 2.0),
        )
    }

    pub fn mid_top(&self) -> Vec2 {
        Vec2::new(self.position.x, self.position.y + (self.height / 2.0))
    }

    pub fn mid_bottom(&self) -> Vec2 {
        Vec2::new(self.position.x, self.position.y - (self.height / 2.0))
    }

    pub fn mid_left(&self) -> Vec2 {
        Vec2::new(self.position.x - (self.width / 2.0), self.position.y)
    }

    pub fn mid_right(&self) -> Vec2 {
        Vec2::new(self.position.x + (self.width / 2.0), self.position.y)
    }
}
