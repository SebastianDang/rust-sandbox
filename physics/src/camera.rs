use bevy::prelude::*;

use super::player::*;

pub fn new_cursor_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraCursor2d::default());
}

pub fn new_player_follow_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraFollowConfig::default());
}

#[derive(Component)]
pub struct CameraCursor2d {
    pub world_pos: Vec2,
}

impl Default for CameraCursor2d {
    fn default() -> Self {
        Self {
            world_pos: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct CameraFollowConfig {
    pub threshold: f32,
}

impl Default for CameraFollowConfig {
    fn default() -> Self {
        Self { threshold: 20.0 }
    }
}

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(track_cursor_system)
            .add_system(follow_player_system);
    }
}

fn track_cursor_system(
    windows: Res<Windows>,
    cameras: Query<(&Camera, &Transform), With<CameraCursor2d>>,
    mut camera_states: Query<&mut CameraCursor2d, With<CameraCursor2d>>,
) {
    // check if main camera exists
    if cameras.is_empty() || camera_states.is_empty() {
        return;
    }

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = cameras.single();

    // get the camera state
    let mut state = camera_states.single_mut();

    // get the window that the camera is displaying to
    let window = windows.get(camera.window).unwrap();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        // Update state
        state.world_pos = world_pos;
        // eprintln!("world coordinates: {} {}", world_pos.x, world_pos.y);
    }
}

fn follow_player_system(
    time: Res<Time>,
    players: Query<&GlobalTransform, With<Player>>,
    mut query: Query<(&mut Transform, &CameraFollowConfig)>,
) {
    if players.is_empty() {
        return;
    }

    let player = players.single();
    let player_transform = player.translation;
    let target = Vec2::new(player_transform.x, player_transform.y);

    for (mut transform, config) in query.iter_mut() {
        let source = Vec2::new(transform.translation.x, transform.translation.y);

        let dist = target - source;
        if dist.length() < config.threshold {
            continue;
        }

        let speed = dist * (dist.length() - config.threshold) / dist.length();
        let velocity = speed * time.delta_seconds();
        transform.translation += Vec3::new(velocity.x, velocity.y, 0.0);
    }
}
