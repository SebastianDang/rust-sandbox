use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct Line2d {
    pub p0: Vec2,
    pub p1: Vec2,
}

impl Line2d {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            p0: Vec2::new(x1, y1),
            p1: Vec2::new(x2, y2),
        }
    }

    pub fn from_points(p0: Vec2, p1: Vec2) -> Self {
        Self { p0, p1 }
    }
}

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
