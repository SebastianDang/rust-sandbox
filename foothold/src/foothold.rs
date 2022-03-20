use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct FootholdContainer {
    pub data: HashMap<u32, Foothold>,
}

/// Represents a foothold as a set of points.
#[derive(Clone, Component, Debug)]
pub struct Foothold {
    pub id: u32,
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub prev: u32,
    pub next: u32,
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
            let y = self.y2 + ((x - self.x2) * slope);
            return Some(y);
        }

        None
    }
}

/// Represents a foothold id attached to an entity.
#[derive(Clone, Component, Debug)]
pub struct FootholdId(pub u32);

pub struct FootholdPlugin;

impl Plugin for FootholdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FootholdContainer>();
        app.add_system(add_new_footholds);
    }
}

/// detect new footholds and add it to the resource
fn add_new_footholds(
    mut container: ResMut<FootholdContainer>,
    query: Query<&Foothold, Added<Foothold>>,
) {
    for foothold in query.iter() {
        // Only insert footholds with ids 1 or greater
        if foothold.id != 0 {
            info!("foothold({}): inserted", foothold.id);
            container.data.insert(foothold.id, foothold.clone());
        }
    }
}
