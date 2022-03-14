use bevy::prelude::*;

mod camera;
mod foothold;
mod player;
mod render;

use camera::*;
use foothold::*;
use player::*;
use render::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .add_startup_system(new_player_follow_camera)
        .run();
}

fn setup(mut commands: Commands) {
    let points: [Vec2; 5] = [
        Vec2::new(-100.0, 0.0),
        Vec2::new(-50.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(50.0, 0.0),
        Vec2::new(100.0, 0.0),
    ];

    for it in 1..points.len() {
        let foothold = Foothold {
            x1: points[it - 1].x,
            y1: points[it - 1].y,
            x2: points[it].x,
            y2: points[it].y,
        };

        commands
            .spawn()
            .insert(foothold)
            .insert(RenderColor::with_id(rand::random()));
    }
}
