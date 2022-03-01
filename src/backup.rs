use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_movement_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider)
        .insert(Player);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(20.0, 500.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(250.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider);
}

#[derive(Component)]
pub struct Player;

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 1.0;
        }
    }
}

#[derive(Component)]
pub struct Collider;

fn collider_system(
    mut players: Query<&mut Transform, With<Player>>,
    mut query: Query<(&mut Collider, &mut Transform)>,
) {
}

// #[derive(Component)]
// pub struct PhysicsConfig {
//     pub gravity: f32,
//     pub max_down_velocity: f32,
//     pub max_up_velocity: f32,
// }

// impl Default for PhysicsConfig {
//     fn default() -> Self {
//         Self {
//             gravity: -1.0,
//             max_down_velocity: -3.0,
//             max_up_velocity: 3.0,
//         }
//     }
// }

// #[derive(Component)]
// pub struct PhysicsComponent {
//     pub position: Vec2,
//     pub velocity: Vec2,
//     pub acceleration: Vec2,
// }

// impl Default for PhysicsComponent {
//     fn default() -> Self {
//         Self {
//             position: Default::default(),
//             velocity: Default::default(),
//             acceleration: Default::default(),
//         }
//     }
// }

// fn physics_system(
//     config: Query<&PhysicsConfig>,
//     mut query: Query<(&mut PhysicsComponent, &mut Transform)>,
// ) {
//     // Check if a configuration is set
//     if config.is_empty() {
//         return;
//     }
//     let config = config.single();

//     for (mut entity, mut transform) in query.iter_mut() {
//         // Update physicss
//         entity.acceleration.y = config.gravity;
//         entity.velocity.y = clamp::clamp(
//             config.max_down_velocity,
//             entity.velocity.y + entity.acceleration.y,
//             config.max_up_velocity,
//         );

//         // Update position in x,y
//         let velocity = entity.velocity;
//         let acceleration = entity.acceleration;
//         entity.position += velocity + 0.5 * acceleration;

//         // Update transform
//         transform.translation = Vec3::new(entity.position.x, entity.position.y, 0.0);
//     }
// }
