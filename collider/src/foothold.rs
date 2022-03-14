use bevy::prelude::*;

/// Represents a foothold as a set of points.
#[derive(Clone, Component, Debug)]
pub struct Foothold {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl Foothold {
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
    ///
    pub fn get_y_at_x(&self, x: f32) -> Option<f32> {
        // Check if 2 points contain x
        if x >= self.x1 && x <= self.x2 {
            let slope = (self.y2 - self.y1) / (self.x2 - self.x1);
            let y = self.y1 + ((self.x1 + x) * slope);
            return Some(y);
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
        // Check if 2 points contain x
        if x >= self.x1 && x <= self.x2 {
            let slope = (self.y2 - self.y1) / (self.x2 - self.x1);
            let angle = slope.atan();
            return Some(angle);
        }
        None
    }
}
