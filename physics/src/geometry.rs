use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}

impl Point2d {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

impl From<Vec2> for Point2d {
    fn from(item: Vec2) -> Self {
        Point2d {
            x: item.x,
            y: item.y,
        }
    }
}

#[derive(Debug, Component)]
pub struct Line2d {
    pub p0: Point2d,
    pub p1: Point2d,
}

impl Line2d {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            p0: Point2d::new(x1, y1),
            p1: Point2d::new(x2, y2),
        }
    }

    pub fn from_points(p0: Point2d, p1: Point2d) -> Self {
        Self { p0, p1 }
    }
}

#[derive(Debug, Component)]
pub struct Quad2d {
    pub position: Point2d,
    pub width: f32,
    pub height: f32,
}

impl Quad2d {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Point2d::new(x, y),
            width,
            height,
        }
    }

    pub fn top_left(&self) -> Point2d {
        Point2d::new(
            self.position.x - (self.width / 2.0),
            self.position.y + (self.height / 2.0),
        )
    }

    pub fn top_right(&self) -> Point2d {
        Point2d::new(
            self.position.x + (self.width / 2.0),
            self.position.y + (self.height / 2.0),
        )
    }

    pub fn bottom_left(&self) -> Point2d {
        Point2d::new(
            self.position.x - (self.width / 2.0),
            self.position.y - (self.height / 2.0),
        )
    }

    pub fn bottom_right(&self) -> Point2d {
        Point2d::new(
            self.position.x + (self.width / 2.0),
            self.position.y - (self.height / 2.0),
        )
    }
}
