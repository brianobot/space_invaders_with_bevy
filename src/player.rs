use bevy::prelude::*;

use crate::Resolution;
use crate::{Alien, Dead};

const BULLET_SPEED: f32 = 1000.;

#[derive(Component)]
pub struct Player {
    pub shoot_timer: f32,
}


#[derive(Component)]
pub struct Bullet;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, (update_player, debug_player_position));
        app.add_systems(Update, (update_bullets, update_alien_interations));
    }
}

fn debug_player_position(player_query: Query<(&Player, &Transform)>) {
    for (_, transform) in player_query.iter() {
        info!("Player Position: {:?}", transform);
    }
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, resolution: Res<Resolution>) {
    let player_image = asset_server.load("player.png");

    let position = Vec3::ZERO - Vec3::Y * resolution.screen_dimension.y * 0.45;

    commands.spawn((
        Player { shoot_timer: 1. },
        Sprite::from_image(player_image),
        Transform::from_translation(position).with_scale(Vec3::splat(resolution.pixel_ratio)),
    ));
}

fn update_player(mut commands: Commands, mut player_query: Query<(&Player, &mut Transform)>, key: Res<ButtonInput<KeyCode>>, time: Res<Time>, resolution: Res<Resolution>, asset_server: Res<AssetServer>) {
    let bullet_image = asset_server.load("bullet.png");

    for (_, mut transform) in player_query.iter_mut() {
        if key.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            transform.translation.x += time.delta_secs() * 100.;
        }
        if key.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            transform.translation.x -= time.delta_secs() * 100.;
        }
        
        if key.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            transform.translation.y += time.delta_secs() * 100.;
        }
       
        if key.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            transform.translation.y -= time.delta_secs() * 100.;
        }

        if key.just_pressed(KeyCode::Space) {
            let player_top = transform.translation + Vec3::Y * 1.5;
            commands.spawn(
                (
                    Bullet,
                    Sprite::from_image(bullet_image.clone()),
                    Transform::from_translation(player_top).with_scale(Vec3::splat(resolution.pixel_ratio))
                )
            );
        }

        let right_bound = resolution.screen_dimension.x * 0.5; 
        let left_bound = -resolution.screen_dimension.x * 0.5; 

        if transform.translation.x > right_bound - 10. {
            transform.translation.x = right_bound - 10.
        }
        if transform.translation.x < left_bound + 10. {
            transform.translation.x = left_bound + 10.
        }
    }
}

fn update_bullets(mut commands: Commands, mut bullet_query: Query<(Entity, &mut Transform), With<Bullet>>, time: Res<Time>, resolution: Res<Resolution>) {
    for (entity, mut transform) in bullet_query.iter_mut() {
        transform.translation.y += time.delta_secs() * BULLET_SPEED;

        if transform.translation.y >= (resolution.screen_dimension.y * 0.5)- 20. {
            commands.entity(entity).despawn();
        }
    }
}

fn update_alien_interations(mut commands: Commands, bullet_query: Query<(Entity, &Transform), With<Bullet>>, alien_query: Query<(Entity, &Transform), (With<Alien>,  Without<Dead>)>) {
    for (alien_entity, alien_transform) in alien_query.iter() {
        for (bullet_entity, bullet_transform) in bullet_query {
            let alien_position = Vec2::new(alien_transform.translation.x, alien_transform.translation.y);
            let bullet_position = Vec2::new(bullet_transform.translation.x, bullet_transform.translation.y);

            if Vec2::distance(alien_position, bullet_position) < 10. {
                commands.entity(alien_entity).despawn();
            }
        }
    }
}

