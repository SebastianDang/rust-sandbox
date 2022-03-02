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
        .add_system(player_movement_system)
        .add_system(player_collision_system)
        .run();
}

fn setup(mut commands: Commands) {
    let points: [Point2d; 4] = [
        Point2d { x: -500.0, y: 0.0 },
        Point2d { x: 50.0, y: 0.0 },
        Point2d { x: 100.0, y: 50.0 },
        Point2d { x: 150.0, y: 50.0 },
    ];

    for (curr, next) in [(0, 1), (1, 2), (2, 3)] {
        commands
            .spawn()
            .insert(Line2d::from_points(
                points[curr].clone(),
                points[next].clone(),
            ))
            .insert(RenderColor::default());
    }

    commands
        .spawn()
        .insert(Line2d::new(-500.0, 0.0, -500.0, 500.0))
        .insert(RenderColor::default());

    commands
        .spawn()
        .insert(Line2d::new(150.0, 50.0, 150.0, 500.0))
        .insert(RenderColor::default());

    commands
        .spawn()
        .insert(Line2d::new(-500.0, 500.0, 150.0, 500.0))
        .insert(RenderColor::default());

    commands
        .spawn()
        .insert(Player)
        .insert(Quad2d::new(0.0, 20.0, 20.0, 40.0))
        .insert(RenderColor::default());
}

#[derive(Component)]
pub struct Player;

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<&mut Quad2d, With<Player>>,
) {
    if player.is_empty() {
        return;
    }
    let mut player = player.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        player.position.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        player.position.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        player.position.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        player.position.y += 1.0;
    }
}

fn player_collision_system(
    mut player: Query<&mut Quad2d, With<Player>>,
    mut lines: Query<&Line2d, Without<Player>>,
) {
    if player.is_empty() {
        return;
    }
    let mut player = player.single_mut();

    for line in lines.iter_mut() {
        for collision in collide_quad_line(&player, line) {
            match collision {
                Collision::Top(point) | Collision::Bottom(point) => {
                    // if player.mid_left().x < point.x {
                    //     player.position.x = point.x + (player.width / 2.) + 1.0;
                    // }
                    // if player.mid_right().x > point.x {
                    //     player.position.x = point.x - (player.width / 2.) - 1.0;
                    // }
                }
                Collision::Left(point) => {
                    if player.bottom_left().y < point.y {
                        player.position.y = point.y + (player.height / 2.) + 1.0;
                    }
                }
                Collision::Right(point) => {
                    if player.bottom_right().y < point.y {
                        player.position.y = point.y + (player.height / 2.) + 1.0;
                    }
                }
            }
        }
    }
}

// #[derive(Component)]
// struct User;

// fn line_p1_follows_cursor_system(
//     main_camera_state: Query<&CameraCursor2d, With<MainCamera>>,
//     mut user_lines: Query<&mut Line2d, With<User>>,
// ) {
//     if main_camera_state.is_empty() || user_lines.is_empty() {
//         return;
//     }

//     let state = main_camera_state.single();
//     let mut line = user_lines.single_mut();
//     line.p1 = state.world_pos.into();
// }

// fn collision_system(
//     user_lines: Query<&Line2d, With<User>>,
//     mut quads: Query<&Quad2d, Without<User>>,
//     mut lines: Query<&Line2d, Without<User>>,
// ) {
//     // check if line exists
//     if user_lines.is_empty() {
//         return;
//     }
//     let user_line = user_lines.single();

//     for quad in quads.iter_mut() {
//         let collisions = collide_quad_line(quad, user_line);
//         dbg!(collisions);
//     }

//     for line in lines.iter_mut() {
//         match collide_line_line(line, user_line) {
//             Some(point) => {
//                 dbg!(point);
//             }
//             None => {}
//         }
//     }
// }
