use crate::*;
use bevy::prelude::*;

pub fn new_player_follow_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraFollowConfig::default());
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
        app.add_system(follow_player_system);
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
