use bevy::prelude::*;

use crate::{alien, Resolution};

#[derive(Component)]
pub struct Alien {
    pub is_dead: bool,
}

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct AlienBullet;

#[derive(Resource, Default)]
pub struct AlienManager {
    pub direction: f32,
    pub shift_aliens_down: bool,
    pub dist_from_boundary: f32,
}


pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_aliens);
        app.add_systems(Update, (update_aliens, manage_alien_logic));
        app.add_systems(Update, (spawn_enemy_bullets, update_enemy_bullets, remove_dead_aliens));
    }
}

const SPEED: f32 = 100.;
const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 24.;
const ALIEN_SHIFT_AMOUNT: f32 = 10.;


fn setup_aliens(mut commands: Commands, asset_server: Res<AssetServer>, resolution: Res<Resolution>) {
    let alien_image = asset_server.load("alien.png");

    commands.insert_resource(AlienManager {
        direction: 1.,
        ..Default::default()
    });

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let position = Vec3::new(x as f32 * SPACING, y as f32 * SPACING, 0.)
                - (Vec3::X * WIDTH as f32 * SPACING * 0.5)
                - (Vec3::Y * HEIGHT as f32 * SPACING * 1.0)
                + (Vec3::Y * resolution.screen_dimension.y * 0.5);

            commands.spawn((
                Alien {
                    is_dead: false
                },
                Sprite {
                    image: alien_image.clone(),
                    ..Default::default()
                },
                Transform::from_translation(position).with_scale(Vec3::splat(resolution.pixel_ratio)),
            ));
        }
    }
}

fn update_aliens(mut alien_query: Query<(&Alien, &mut Transform), Without<Dead>>, time: Res<Time>, mut alien_manager: ResMut<AlienManager>, resolution: Res<Resolution>) {
    for (_alien, mut transform) in alien_query.iter_mut() {
        transform.translation.x += time.delta_secs() * alien_manager.direction * SPEED;
        
        if transform.translation.x.abs() > resolution.screen_dimension.x * 0.5 {
            alien_manager.shift_aliens_down = true;
            alien_manager.dist_from_boundary = resolution.screen_dimension.x * alien_manager.direction * 0.5 - transform.translation.x;
        }   
    }
}


fn manage_alien_logic(mut alien_query: Query<(&mut Alien, &mut Transform),  Without<Dead>>, mut alien_manager: ResMut<AlienManager>) {
    if alien_manager.shift_aliens_down {
        alien_manager.shift_aliens_down = false;
        alien_manager.direction *= -1.;

        for (_alien, mut transform) in alien_query.iter_mut() {
            transform.translation.x += alien_manager.dist_from_boundary;
            transform.translation.y -= ALIEN_SHIFT_AMOUNT;
        }
    }
}

fn spawn_enemy_bullets(mut commands: Commands, alien_query: Query<&Transform, (With<Alien>,  Without<Dead>)>, asset_server: Res<AssetServer>, resolution: Res<Resolution>) {
    let bullet_image = asset_server.load("bullet.png");

    for transform in alien_query.iter() {
        let alien_front_position = transform.translation - (Vec3::Y * 10.);

        commands.spawn((
            AlienBullet,
            Sprite::from_image(bullet_image.clone()),
            Transform::from_translation(alien_front_position).with_scale(Vec3::splat(resolution.pixel_ratio))
        ));
    }
}

fn update_enemy_bullets(mut alien_query: Query<&mut Transform, With<AlienBullet>>, time: Res<Time>) {
    for mut transform in alien_query.iter_mut() {
        transform.translation.y -= time.delta_secs() * 100.;
    }
}


fn remove_dead_aliens(mut commands: Commands, dead_alien_query: Query<(Entity, &Alien), With<Dead>>) {
    for (entity, alien) in dead_alien_query.iter() {
        commands.entity(entity).despawn();
    }
}