use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

mod camera;
use camera::*;

mod geometry;
use geometry::*;

mod collider;
use collider::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(new_main_camera.system())
        .add_startup_system(setup)
        .add_system(line_p1_follows_cursor_system)
        .add_system(render_lines_system)
        .add_system(render_quad_system)
        .add_system(collision_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Line2d::new(0.0, 0.0, 0.0, 0.0))
        .insert(ColorComponent::default())
        .insert(User);

    commands
        .spawn()
        .insert(Line2d::new(200.0, 200.0, 400.0, -200.0))
        .insert(ColorComponent::default());

    commands
        .spawn()
        .insert(Quad2d::new(0.0, 0.0, 100.0, 100.0))
        .insert(ColorComponent::default());
}

#[derive(Component)]
struct User;

#[derive(Debug, Component)]
struct ColorComponent {
    color: Color,
}

impl Default for ColorComponent {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
        }
    }
}

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

fn render_lines_system(
    mut debug_lines: ResMut<DebugLines>,
    lines: Query<(&Line2d, &ColorComponent)>,
) {
    for (line, color) in lines.iter() {
        debug_lines.line_colored(line.p0.as_vec3(), line.p1.as_vec3(), 0., color.color);
    }
}

fn render_quad_system(
    mut debug_lines: ResMut<DebugLines>,
    quads: Query<(&Quad2d, &ColorComponent)>,
) {
    for (quad, color) in quads.iter() {
        let top_left = quad.top_left().as_vec3();
        let top_right = quad.top_right().as_vec3();
        let bottom_left = quad.bottom_left().as_vec3();
        let bottom_right = quad.bottom_right().as_vec3();

        debug_lines.line_colored(top_left, top_right, 0., color.color);
        debug_lines.line_colored(top_left, bottom_left, 0., color.color);
        debug_lines.line_colored(bottom_right, top_right, 0., color.color);
        debug_lines.line_colored(bottom_right, bottom_left, 0., color.color);
    }
}

fn collision_system(
    user_lines: Query<&Line2d, With<User>>,
    mut quads: Query<(&Quad2d, &mut ColorComponent), Without<User>>,
    // mut lines: Query<(&Line2d, &mut ColorComponent), Without<User>>,
) {
    // check if line exists
    if user_lines.is_empty() {
        return;
    }
    let user_line = user_lines.single();

    for (quad, mut color) in quads.iter_mut() {
        color.color = if collide_line_quad(user_line, quad) {
            Color::RED
        } else {
            Color::WHITE
        };
    }

    // for (line, mut color) in lines.iter_mut() {
    //     color.color = match collide_lines(user_line, line) {
    //         Some(_) => Color::RED,
    //         None => Color::WHITE,
    //     };
    // }
}
