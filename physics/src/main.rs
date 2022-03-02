use bevy::prelude::*;

mod camera;
use camera::*;

mod geometry;
use geometry::*;

mod collider;
use collider::*;

mod render;
use render::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(RenderPlugin)
        .add_startup_system(setup)
        .add_startup_system(new_main_camera)
        .add_system(line_p1_follows_cursor_system)
        .add_system(collision_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(User)
        .insert(Line2d::new(0.0, 0.0, 0.0, 0.0))
        .insert(RenderColor::default());

    commands
        .spawn()
        .insert(Line2d::new(200.0, 200.0, 400.0, -200.0))
        .insert(RenderColor::default());

    commands
        .spawn()
        .insert(Quad2d::new(-200.0, 0.0, 100.0, 100.0))
        .insert(RenderColor::default());
}

#[derive(Component)]
struct User;

fn line_p1_follows_cursor_system(
    main_camera_state: Query<&CameraCursor2d, With<MainCamera>>,
    mut user_lines: Query<&mut Line2d, With<User>>,
) {
    if main_camera_state.is_empty() || user_lines.is_empty() {
        return;
    }

    let state = main_camera_state.single();
    let mut line = user_lines.single_mut();
    line.p1 = state.world_pos.into();
}

fn collision_system(
    user_lines: Query<&Line2d, With<User>>,
    mut quads: Query<&Quad2d, Without<User>>,
    mut lines: Query<&Line2d, Without<User>>,
) {
    // check if line exists
    if user_lines.is_empty() {
        return;
    }
    let user_line = user_lines.single();

    for quad in quads.iter_mut() {
        let collisions = collide_quad_line(quad, user_line);
        dbg!(collisions);
    }

    for line in lines.iter_mut() {
        match collide_line_line(line, user_line) {
            Some(point) => {
                dbg!(point);
            }
            None => {}
        }
    }
}
