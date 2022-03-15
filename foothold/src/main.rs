use bevy::prelude::*;
use foothold_crate::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FootholdPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .add_startup_system(new_player_follow_camera)
        .run();
}

fn setup(mut commands: Commands) {
    let top: [Vec2; 11] = [
        Vec2::new(-500.0, 100.0),
        Vec2::new(-400.0, 125.0),
        Vec2::new(-300.0, 125.0),
        Vec2::new(-200.0, 100.0),
        Vec2::new(-100.0, 100.0),
        Vec2::new(0.0, 125.0),
        Vec2::new(100.0, 150.0),
        Vec2::new(200.0, 150.0),
        Vec2::new(300.0, 125.0),
        Vec2::new(400.0, 100.0),
        Vec2::new(500.0, 100.0),
    ];

    for (pos, it) in (1..top.len()).enumerate() {
        let id = (pos + 1) as u32; // 1-indexed
        let prev = if pos > 0 { id - 1 } else { 0 };
        let next = if pos < top.len() { id + 1 } else { 0 };

        let foothold = Foothold {
            id,
            x1: top[it - 1].x,
            y1: top[it - 1].y,
            x2: top[it].x,
            y2: top[it].y,
            prev,
            next,
        };

        commands
            .spawn()
            .insert(foothold)
            .insert(RenderColor::from(Color::WHITE));
    }

    let bottom: [Vec2; 11] = [
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

    let top_len = top.len();
    for (pos, it) in (1..bottom.len()).enumerate() {
        let id = (top_len + pos + 1) as u32; // 1-indexed
        let prev = if pos > 0 { id - 1 } else { 0 };
        let next = if pos < bottom.len() { id + 1 } else { 0 };

        let foothold = Foothold {
            id,
            x1: bottom[it - 1].x,
            y1: bottom[it - 1].y,
            x2: bottom[it].x,
            y2: bottom[it].y,
            prev,
            next,
        };

        commands
            .spawn()
            .insert(foothold)
            .insert(RenderColor::from(Color::WHITE));
    }

    let slope: [Vec2; 6] = [
        Vec2::new(400.0, 0.0),
        Vec2::new(500.0, 25.0),
        Vec2::new(600.0, 50.0),
        Vec2::new(700.0, 0.0),
        Vec2::new(800.0, 0.0),
        Vec2::new(900.0, 0.0),
    ];

    let bottom_len = bottom.len();
    for (pos, it) in (1..slope.len()).enumerate() {
        let id = (top_len + bottom_len + pos + 1) as u32; // 1-indexed
        let prev = if pos > 0 { id - 1 } else { 0 };
        let next = if pos < slope.len() { id + 1 } else { 0 };

        let foothold = Foothold {
            id,
            x1: slope[it - 1].x,
            y1: slope[it - 1].y,
            x2: slope[it].x,
            y2: slope[it].y,
            prev,
            next,
        };

        commands
            .spawn()
            .insert(foothold)
            .insert(RenderColor::from(Color::WHITE));
    }
}
