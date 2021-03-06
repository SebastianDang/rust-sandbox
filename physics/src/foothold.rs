use bevy::prelude::*;

/// Represents a foothold as a set of points.
#[derive(Clone, Component, Debug)]
pub struct Foothold {
    pub points: Vec<Vec2>,
}

impl Foothold {
    /// Creates a foothold from a set of points.
    ///
    /// # Arguments
    ///
    /// * `points`: An array of points that represent the foothold.
    ///
    /// # Examples
    /// ```
    /// let points: [Vec2; 2] = [
    ///     Vec2::new(0.0, 0.0),
    ///     Vec2::new(45.0, 0.0),
    /// ];
    /// let foothold = Foothold::from_points(points);
    /// ```
    pub fn from_points(points: &[Vec2]) -> Self {
        Foothold {
            points: points.to_vec(),
        }
    }

    /// Gets the y coordinate if x is within the range of points of this foothold.
    ///
    /// # Arguments
    ///
    /// * `x`: The value to evaluate for y.
    ///
    /// # Examples
    /// ```
    /// match foothold.get_y_at_x(5.0) {
    ///     Some(y) => { println!("{}", y) },
    ///     None => { },
    /// };
    /// ```
    pub fn get_y_at_x(&self, x: f32) -> Option<f32> {
        let points = &self.points;

        // Loop through each pair of points
        for it in 1..points.len() {
            let p1 = points[it - 1];
            let p2 = points[it];

            // Check if 2 points contain x
            if x >= p1.x && x <= p2.x {
                let slope = (p2.y - p1.y) / (p2.x - p1.x);
                let y = p2.y + ((x - p2.x) * slope);
                return Some(y);
            }
        }
        None
    }

    /// Gets the angle if x is within the range of points of this foothold.
    /// Returns the angle in radians.
    ///
    /// # Arguments
    ///
    /// * `x`: The value to evaluate for y.
    ///
    /// # Examples
    /// ```
    /// match foothold.get_angle_at_x(5.0) {
    ///     Some(angle) => { println!("{}", angle) },
    ///     None => { },
    /// };
    /// ```
    pub fn get_angle_at_x(&self, x: f32) -> Option<f32> {
        let points = &self.points;

        // Loop through each pair of points
        for it in 1..points.len() {
            let p1 = points[it - 1];
            let p2 = points[it];

            // Check if 2 points contain x
            if x >= p1.x && x <= p2.x {
                let slope = (p2.y - p1.y) / (p2.x - p1.x);
                let angle = slope.atan();
                return Some(angle);
            }
        }
        None
    }
}

/// Represents the layer a foothold belongs to.
#[derive(Clone, Component)]
pub struct FootholdLayer(pub u32);
