use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Foothold {
    pub points: Vec<Vec2>,
}

impl Foothold {
    pub fn from_points(points: &[Vec2]) -> Self {
        Foothold {
            points: points.to_vec(),
        }
    }

    pub fn get_x_in_range(&self, x: f32) -> bool {
        let points = &self.points;
        for it in 1..points.len() {
            if x >= points[it - 1].x && x <= points[it].x {
                return true;
            }
        }
        false
    }

    pub fn get_y_at_x(&self, x: f32) -> Option<f32> {
        let points = &self.points;

        // Loop through each pair of points
        for it in 1..points.len() {
            let p1 = points[it - 1];
            let p2 = points[it];

            // Check if 2 points contain x
            if x >= p1.x && x <= p2.x {
                // Calculate the slope of the line
                let slope = (p2.y - p1.y) / (p2.x - p1.x);
                let y = p2.y + ((x - p2.x) * slope);
                return Some(y);
            }
        }
        None
    }
}
