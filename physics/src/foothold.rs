use bevy::prelude::*;

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

    /// Determines if x within the range of points of this foothold.
    ///
    /// # Arguments
    ///
    /// * `x`: The value to find.
    ///
    /// # Examples
    /// ```
    /// let in_range = foothold.get_x_in_range(5.0);
    /// ```
    pub fn get_x_in_range(&self, x: f32) -> bool {
        let points = &self.points;
        for it in 1..points.len() {
            if x >= points[it - 1].x && x <= points[it].x {
                return true;
            }
        }
        false
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
                // Calculate the slope of the line
                let slope = (p2.y - p1.y) / (p2.x - p1.x);
                let y = p2.y + ((x - p2.x) * slope);
                return Some(y);
            }
        }
        None
    }
}

#[derive(Clone, Component)]
pub struct FootholdLayer(pub u32);
