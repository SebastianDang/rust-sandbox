use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

mod camera;
use camera::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(new_main_camera.system())
        .add_startup_system(setup)
        .add_system(line_p1_follows_cursor_system)
        .add_system(render_lines_system)
        .add_system(collision_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Line2d {
            p0: Point2d::new(0.0, 50.0),
            p1: Point2d::new(0.0, 0.0),
        })
        .insert(ColorComponent::default())
        .insert(User);

    commands
        .spawn()
        .insert(Line2d {
            p0: Point2d::new(50.0, 0.0),
            p1: Point2d::new(100.0, 0.0),
        })
        .insert(ColorComponent::default());
}

#[derive(Component)]
struct User;

#[derive(Debug)]
struct Point2d {
    x: f32,
    y: f32,
}

impl Point2d {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

impl From<Vec2> for Point2d {
    fn from(item: Vec2) -> Self {
        Point2d {
            x: item.x,
            y: item.y,
        }
    }
}

#[derive(Debug, Component)]
struct Line2d {
    p0: Point2d,
    p1: Point2d,
}

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

fn collision_system(
    user_lines: Query<&Line2d, With<User>>,
    mut lines: Query<(&Line2d, &mut ColorComponent), Without<User>>,
) {
    // check if line exists
    if user_lines.is_empty() {
        return;
    }
    let user_line = user_lines.single();

    for (line, mut color) in lines.iter_mut() {
        let intersection = lineline(
            user_line.p0.x,
            user_line.p0.y,
            user_line.p1.x,
            user_line.p1.y,
            line.p0.x,
            line.p0.y,
            line.p1.x,
            line.p1.y,
        );

        color.color = match intersection {
            Some(_) => Color::RED,
            None => Color::WHITE,
        };
    }
}

fn lineline(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) -> Option<Vec2> {
    // calculate the distance to intersection point
    let uA = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3))
        / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));
    let uB = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3))
        / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));

    // if uA and uB are between 0-1, lines are colliding
    if uA >= 0.0 && uA <= 1.0 && uB >= 0.0 && uB <= 1.0 {
        let intersection = Vec2::new(x1 + (uA * (x2 - x1)), y1 + (uA * (y2 - y1)));
        eprintln!("intersection point {}", intersection);
        Some(intersection)
    } else {
        None
    }
}

// #[derive(Component)]
// struct Line {
//     p0: Vec2,
//     p1: Vec2,
// }

// struct Collider {
//     width: f32,
//     height: f32,
// }

// impl Collider {
//     fn collides(pos_a: Vec2, size_a: &Collider, pos_b: Vec2, size_b: &Collider) -> bool {
//         if pos_a.x < pos_b.x + size_b.width
//             && pos_a.x + size_a.width > pos_b.x
//             && pos_a.y < pos_b.y + size_b.height
//             && pos_a.y + size_a.height > pos_b.y
//         {
//             true
//         } else {
//             false
//         }
//     }
// }
