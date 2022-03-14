use bevy::prelude::*;
use collider::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CollisionPlugin)
        .add_startup_system(setup)
        .add_startup_system(new_player_follow_camera)
        .run();
}

fn setup(mut commands: Commands) {
    let points: [Vec2; 11] = [
        Vec2::new(-500.0, 0.0),
        Vec2::new(-400.0, 25.0),
        Vec2::new(-300.0, 25.0),
        Vec2::new(-200.0, 0.0),
        Vec2::new(-100.0, 0.0),
        Vec2::new(0.0, 25.0),
        Vec2::new(100.0, 50.0),
        Vec2::new(200.0, 50.0),
        Vec2::new(300.0, 25.0),
        Vec2::new(400.0, 0.0),
        Vec2::new(500.0, 0.0),
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
            .insert(RenderColor::from(Color::WHITE));
    }
}
