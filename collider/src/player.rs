use bevy::prelude::*;

use super::render::*;

#[derive(Clone, Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
        app.add_system(player_movement_system);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("player.png");

    commands
        .spawn_bundle(SpriteBundle {
            texture,
            ..Default::default()
        })
        .insert(Player)
        .insert(RenderColor::default());
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Sprite), (With<Transform>, With<Sprite>, With<Player>)>,
) {
    if player.is_empty() {
        return;
    }
    let (mut transform, mut sprite) = player.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x -= 1.0;
        sprite.flip_x = false;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        transform.translation.x += 1.0;
        sprite.flip_x = true;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        transform.translation.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        transform.translation.y -= 1.0;
    }
}
