use crate::*;
use bevy::prelude::*;

#[derive(Clone, Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RigidBodyPlugin);

        app.add_startup_system(spawn_player);
        app.add_system(player_movement_system)
            .add_system(player_foothold_collision_system);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("player.png");

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 200.0, 0.0),
            texture,
            ..Default::default()
        })
        .insert(Player)
        .insert(RigidBody::default())
        .insert(RenderColor::default());
}

fn player_movement_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &mut RigidBody), (With<RigidBody>, With<Player>)>,
) {
    if player.is_empty() {
        return;
    }
    let (entity, mut body) = player.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        body.acceleration.x = -MOVEMENT_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        body.acceleration.x = MOVEMENT_SPEED;
    }

    if !keyboard_input.pressed(KeyCode::Left) && !keyboard_input.pressed(KeyCode::Right) {
        body.acceleration.x = 0.0;
    }

    if keyboard_input.pressed(KeyCode::LAlt) {
        commands.entity(entity).remove::<FootholdId>();
        body.acceleration.y = JUMP_FORCE;
    }
}

fn player_foothold_collision_system(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    footholds_container: Res<FootholdContainer>,
    mut player: Query<
        (
            Entity,
            &mut Transform,
            &mut RigidBody,
            &Handle<Image>,
            Option<&FootholdId>,
        ),
        (With<Transform>, With<RigidBody>, With<Sprite>, With<Player>),
    >,
    footholds: Query<&Foothold, With<Foothold>>,
) {
    if player.is_empty() {
        return;
    }
    let (entity, mut transform, body, texture, foothold_id) = player.single_mut();

    // Calculate the next position
    let mut next_transform = transform.clone();
    next_transform.translation += Vec3::new(body.velocity.x, body.velocity.y, 0.0)
        + Vec3::new(0.5 * body.acceleration.x, 0.5 * body.acceleration.y, 0.0);

    if let Some(image) = images.get(texture) {
        let height = image.texture_descriptor.size.height as f32;

        // Determine if we need to perform collision detection
        let mut use_collision = false;

        // Footold exists: check nodes and update
        if foothold_id.is_some() {
            let curr = foothold_id.unwrap().0;

            if let Some(foothold) = footholds_container.data.get(&curr) {
                if let Some(y) = foothold.get_y_at_x(next_transform.translation.x) {
                    position_limit_ground_y(&mut (next_transform.translation), height, y);
                } else if let Some(y) = container_get_y_at_x(
                    &footholds_container,
                    foothold.prev,
                    next_transform.translation.x,
                ) {
                    info!("fh({}): previous({})", foothold.id, foothold.prev);
                    commands.entity(entity).insert(FootholdId(foothold.prev));
                    position_limit_ground_y(&mut (next_transform.translation), height, y);
                } else if let Some(y) = container_get_y_at_x(
                    &footholds_container,
                    foothold.next,
                    next_transform.translation.x,
                ) {
                    info!("fh({}): next({})", foothold.id, foothold.next);
                    commands.entity(entity).insert(FootholdId(foothold.next));
                    position_limit_ground_y(&mut (next_transform.translation), height, y);
                } else {
                    info!("fh({}): removed", foothold.id);
                    commands.entity(entity).remove::<FootholdId>();
                    use_collision = true;
                }
            }
        } else {
            use_collision = true;
        }

        // Foothold doesn't exist: check for new collisions
        if use_collision {
            for foothold in footholds.iter() {
                if let Some(collision) = calculate_fh_collision(
                    &footholds_container,
                    &foothold,
                    transform.translation,
                    next_transform.translation,
                    height,
                ) {
                    commands.entity(entity).insert(FootholdId(foothold.id));
                    position_limit_ground_y(&mut (next_transform.translation), height, collision.y);
                    break;
                }
            }
        }

        transform.translation = next_transform.translation;
    }
}

fn position_limit_ground_y(position: &mut Vec3, height: f32, y: f32) {
    let half_height = height / 2.0;
    let bottom_y = position.y - half_height;
    if bottom_y < y && position.y > y {
        position.y = y + half_height;
    }
}

/// Calculate any collisions for a foothold, using the current and next points
fn calculate_fh_collision(
    container: &Res<FootholdContainer>,
    foothold: &Foothold,
    current: Vec3,
    next: Vec3,
    height: f32,
) -> Option<Vec2> {
    // TODO: Remove this later
    let current = Vec3::new(current.x, current.y - height / 2.0, 0.0);
    let next = Vec3::new(next.x, next.y - height / 2.0, 0.0);

    // Check current foothold
    if let (Some(current_fh_y), Some(next_fh_y)) =
        (foothold.get_y_at_x(current.x), foothold.get_y_at_x(next.x))
    {
        if current.y >= current_fh_y && next.y <= next_fh_y {
            info!("added fh({})): current", foothold.id);
            return Some(Vec2::new(next.x, next_fh_y));
        }
    }
    // Check current and previous
    else if let (Some(current_fh_y), Some(next_fh_y)) = (
        foothold.get_y_at_x(current.x),
        container_get_y_at_x(container, foothold.prev, next.x),
    ) {
        if current.y >= current_fh_y && next.y <= next_fh_y {
            info!("added fh({}): previous({})", foothold.id, foothold.prev);
            return Some(Vec2::new(next.x, next_fh_y));
        }
    }
    // Check current and next
    else if let (Some(current_fh_y), Some(next_fh_y)) = (
        foothold.get_y_at_x(current.x),
        container_get_y_at_x(container, foothold.next, next.x),
    ) {
        if current.y >= current_fh_y && next.y <= next_fh_y {
            info!("added fh({}): next({})", foothold.id, foothold.next);
            return Some(Vec2::new(next.x, next_fh_y));
        }
    }

    None
}

fn container_get_y_at_x(container: &Res<FootholdContainer>, id: u32, x: f32) -> Option<f32> {
    if let Some(foothold) = container.data.get(&id) {
        foothold.get_y_at_x(x)
    } else {
        None
    }
}
