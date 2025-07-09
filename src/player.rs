use bevy::prelude::*;

use crate::Resolution;
use crate::{Alien, AlienBullet, Dead};

const BULLET_SPEED: f32 = 1000.;

#[derive(Component)]
pub struct Player {
    pub shoot_timer: f32,
}


#[derive(Component)]
pub struct PlayerBullet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, (update_player, debug_player_position));
        app.add_systems(Update, (
            update_bullets, 
            update_alien_interations, 
            update_bullet_interaction,
            update_ammo_interactions,
        ));
    }
}

fn debug_player_position(player_query: Query<(&Player, &Transform)>) {
    for (_, transform) in player_query.iter() {
        debug!("Player Position: {:?}", transform);
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<Resolution>,
) {
    let player_image = asset_server.load("player.png");

    let position = Vec3::ZERO - Vec3::Y * resolution.screen_dimension.y * 0.45;

    commands.spawn((
        Player { shoot_timer: 1. },
        Sprite::from_image(player_image),
        Transform::from_translation(position).with_scale(Vec3::splat(resolution.pixel_ratio)),
    ));
}

fn update_player(
    mut commands: Commands,
    mut player_query: Query<(&Player, &mut Transform)>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    resolution: Res<Resolution>,
    asset_server: Res<AssetServer>,
) {
    let bullet_image = asset_server.load("bullet.png");

    let (_, mut transform) = player_query.single_mut().unwrap();
    let mut multiplier = 1.;

    if key.pressed(KeyCode::ShiftLeft) {
        multiplier *= 5.;
    }

    if key.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        transform.translation.x += time.delta_secs() * 100. * multiplier;
    }
    if key.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        transform.translation.x -= time.delta_secs() * 100. * multiplier;
    }

    if key.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        transform.translation.y += time.delta_secs() * 100. * multiplier;
    }

    if key.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        transform.translation.y -= time.delta_secs() * 100. * multiplier;
    }

    if key.just_pressed(KeyCode::Space) {
        let player_top = transform.translation + Vec3::Y * 1.5;
        commands.spawn((
            PlayerBullet,
            Sprite::from_image(bullet_image.clone()),
            Transform::from_translation(player_top)
                .with_scale(Vec3::splat(resolution.pixel_ratio)),
        ));
    }

    let right_bound = resolution.screen_dimension.x * 0.5;
    let left_bound = -resolution.screen_dimension.x * 0.5;
    let bottom_bound = -resolution.screen_dimension.y * 0.5;

    if transform.translation.x > right_bound - 10. {
        transform.translation.x = right_bound - 10.;
    }
    if transform.translation.x < left_bound + 10. {
        transform.translation.x = left_bound + 10.;
    }
    if transform.translation.y < bottom_bound + 10. {
        transform.translation.y = bottom_bound + 10.;
    }
    
}

fn update_bullets(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Transform), With<PlayerBullet>>,
    time: Res<Time>,
    resolution: Res<Resolution>,
) {
    for (entity, mut transform) in bullet_query.iter_mut() {
        transform.translation.y += time.delta_secs() * BULLET_SPEED;

        if transform.translation.y >= (resolution.screen_dimension.y * 0.5) - 20. {
            commands.entity(entity).despawn();
        }
    }
}

fn update_alien_interations(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<PlayerBullet>>,
    alien_query: Query<(Entity, &Transform), (With<Alien>, Without<Dead>)>,
) {
    for (alien_entity, alien_transform) in alien_query.iter() {
        for (_, bullet_transform) in bullet_query {
            let alien_position =
                Vec2::new(alien_transform.translation.x, alien_transform.translation.y);
            let bullet_position = Vec2::new(
                bullet_transform.translation.x,
                bullet_transform.translation.y,
            );

            if Vec2::distance(alien_position, bullet_position) < 10. {
                commands.entity(alien_entity).despawn();
            }
        }
    }
}


fn update_bullet_interaction(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut alien_bullet_query: Query<(Entity, &Transform), With<AlienBullet>>,
) {
    let (entity, transform) = player_query.single_mut().unwrap();
    let player_position = Vec2::new(transform.translation.x, transform.translation.y);

    for (_, alien_transform) in alien_bullet_query.iter_mut() {
        let alien_bullet_position = Vec2::new(alien_transform.translation.x, alien_transform.translation.y);

        if Vec2::distance(player_position, alien_bullet_position) < 10. {
            commands.entity(entity).despawn()
        }
    }
}


fn update_ammo_interactions(
    mut commands: Commands,
    player_bullet_query: Query<(Entity, &Transform), With<PlayerBullet>>,
    alien_bullet_query: Query<(Entity, &Transform), With<AlienBullet>>,
) {
    for (entity_a, transform_a) in player_bullet_query.iter() {
        for (entity_b, transform_b) in alien_bullet_query.iter() {
            let bullet_a_position = Vec2::new(transform_a.translation.x, transform_a.translation.y);
            let bullet_b_position = Vec2::new(transform_b.translation.x, transform_b.translation.y);

            let distance = Vec2::distance(bullet_a_position, bullet_b_position);
            if distance < 10. {
                commands.entity(entity_a).despawn();
                commands.entity(entity_b).despawn();
            }
        }
    }
}