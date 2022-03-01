use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

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

pub fn new_main_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(CameraCursor2d::default());
}

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(track_cursor_system);
    }
}

fn track_cursor_system(
    windows: Res<Windows>,
    main_camera: Query<(&Camera, &Transform), With<MainCamera>>,
    mut main_camera_state: Query<&mut CameraCursor2d, With<MainCamera>>,
) {
    // check if main camera exists
    if main_camera.is_empty() || main_camera_state.is_empty() {
        return;
    }

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = main_camera.single();

    // get the camera state
    let mut state = main_camera_state.single_mut();

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
