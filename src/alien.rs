use bevy::prelude::*;

use rand;
use rand::Rng;

use crate::{Resolution, PlayerBullet};

#[derive(Component)]
pub struct Alien;

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct AlienBullet {
    pub velocity: f32, 
}

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
        app.add_systems(
            Update,
            (
                spawn_enemy_bullets,
                update_enemy_bullets,
                update_alien_interations,
                remove_dead_aliens,
            ),
        );
    }
}

const SPEED: f32 = 100.;
const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 24.;
const ALIEN_SHIFT_AMOUNT: f32 = 10.;

fn setup_aliens(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<Resolution>,
) {
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
                Alien,
                Sprite {
                    image: alien_image.clone(),
                    ..Default::default()
                },
                Transform::from_translation(position)
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
            ));
        }
    }
}

fn update_aliens(
    mut alien_query: Query<(Entity, &Alien, &mut Transform), Without<Dead>>,
    time: Res<Time>,
    mut alien_manager: ResMut<AlienManager>,
    resolution: Res<Resolution>,
) {
    for (_entity, _alien, mut transform) in alien_query.iter_mut() {
        debug!("{}", _entity);

        transform.translation.x += time.delta_secs() * alien_manager.direction * SPEED;

        if transform.translation.x.abs() > resolution.screen_dimension.x * 0.5 {
            alien_manager.shift_aliens_down = true;
            alien_manager.dist_from_boundary =
                resolution.screen_dimension.x * alien_manager.direction * 0.5
                    - transform.translation.x;
        }
    }
}

fn manage_alien_logic(
    mut alien_query: Query<(&mut Alien, &mut Transform), Without<Dead>>,
    mut alien_manager: ResMut<AlienManager>,
) {
    if alien_manager.shift_aliens_down {
        alien_manager.shift_aliens_down = false;
        alien_manager.direction *= -1.;

        for (_alien, mut transform) in alien_query.iter_mut() {
            transform.translation.x += alien_manager.dist_from_boundary;
            transform.translation.y -= ALIEN_SHIFT_AMOUNT;
        }
    }
}

fn spawn_enemy_bullets(
    mut commands: Commands,
    alien_query: Query<&Transform, (With<Alien>, Without<Dead>)>,
    asset_server: Res<AssetServer>,
    resolution: Res<Resolution>,
) {
    let mut rng = rand::rng();
    let bullet_image = asset_server.load("bullet.png");
    

    for transform in alien_query.iter() {
        let mut bullet_velocity = 1.;

        let alien_shoot = rng.random_bool(0.0001);
        let boosted_shot = rng.random_bool(0.001);

        if alien_shoot {
            let alien_front_position = transform.translation - (Vec3::Y * 10.);
            
            if boosted_shot {
                bullet_velocity += 1000.;
            } 

            commands.spawn((
                AlienBullet { velocity: bullet_velocity },
                Sprite::from_image(bullet_image.clone()),
                Transform::from_translation(alien_front_position)
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
            ));
        }
    }
}

fn update_enemy_bullets(
    mut alien_query: Query<(&AlienBullet, &mut Transform)>,
    time: Res<Time>,
) {
    for (alien_bullet, mut transform) in alien_query.iter_mut() {
        transform.translation.y -= time.delta_secs() * 100. * alien_bullet.velocity;
    }
}


fn update_alien_interations(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<PlayerBullet>>,
    alien_query: Query<(Entity, &Transform), (With<Alien>, Without<Dead>)>,
) {
    for (alien_entity, alien_transform) in alien_query.iter() {
        for (bullet_entity, bullet_transform) in bullet_query {
            let alien_position =
                Vec2::new(alien_transform.translation.x, alien_transform.translation.y);
            let bullet_position = Vec2::new(
                bullet_transform.translation.x,
                bullet_transform.translation.y,
            );

            if Vec2::distance(alien_position, bullet_position) < 10. {
                commands.entity(alien_entity).insert(Dead);
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}

fn remove_dead_aliens(
    mut commands: Commands,
    dead_alien_query: Query<Entity, (With<Alien>, With<Dead>)>,
) {
    for entity in dead_alien_query.iter() {
        commands.entity(entity).despawn();
    }
}
