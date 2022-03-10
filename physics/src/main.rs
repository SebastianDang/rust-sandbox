use bevy::prelude::*;

mod camera;
use camera::*;

mod geometry;
use geometry::*;

mod collider;
use collider::*;

mod render;
use render::*;

mod foothold;
use foothold::*;

mod rigid_body;
use rigid_body::*;

mod player;
use player::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(RigidBodyPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .add_startup_system(new_main_camera)
        .run();
}

fn setup(mut commands: Commands) {
    let layer_0: [Vec2; 8] = [
        Vec2::new(-1000.0, 300.0),
        Vec2::new(45.0, 300.0),
        Vec2::new(135.0, 240.0),
        Vec2::new(225.0, 180.0),
        Vec2::new(315.0, 120.0),
        Vec2::new(405.0, 60.0),
        Vec2::new(495.0, 0.0),
        Vec2::new(534.0, 0.0),
    ];

    let layer_1: [Vec2; 8] = [
        Vec2::new(96.0, -180.0),
        Vec2::new(96.0, -120.0),
        Vec2::new(186.0, -120.0),
        Vec2::new(186.0, -60.0),
        Vec2::new(276.0, -60.0),
        Vec2::new(276.0, 0.0),
        Vec2::new(534.0, 0.0),
        Vec2::new(1000.0, 0.0),
    ];

    let layer_2: [Vec2; 3] = [
        Vec2::new(-1000.0, -180.0),
        Vec2::new(0.0, -180.0),
        Vec2::new(1000.0, -180.0),
    ];

    spawn_foothold_from_points(&mut commands, &layer_0, 0);
    spawn_foothold_from_points(&mut commands, &layer_1, 1);
    spawn_foothold_from_points(&mut commands, &layer_2, 2);

    commands
        .spawn()
        .insert(Player)
        .insert(Quad2d::new(0.0, 100.0, 20.0, 40.0))
        .insert(RenderColor::default())
        .insert(RigidBody::default());
}

fn spawn_foothold_from_points(commands: &mut Commands, points: &[Vec2], layer: u32) {
    commands
        .spawn()
        .insert(Foothold::from_points(points))
        .insert(FootholdLayer(layer))
        .insert(RenderColor::default());
}
