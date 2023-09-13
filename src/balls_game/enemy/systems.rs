use super::super::player::*;

use super::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub fn spawn_enemies(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = window.get_single().unwrap();
    let mut generatror = rand::thread_rng();
    let (x, y) = (window.width(), window.height());
    for _ in 0..NUM_OF_ENEMIES {
        let mut x = generatror.gen_range(32.0..x - 32.0 - 128.0);
        if x > window.width() / 2.0 - 64.0 {
            x += 128.0;
        }
        let mut y = generatror.gen_range(32.0..y - 32.0 - 128.0);
        if y > window.height() / 2.0 - 64.0 {
            y += 128.0;
        }
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: assets.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5)
                    .normalize_or_zero(),
            },
        ));
    }
}

pub fn enemy_movement(mut query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in query.iter_mut() {
        transform.translation += Vec3::new(enemy.direction.x, enemy.direction.y, 0.0)
            * ENEMY_SPEED
            * time.delta_seconds();
    }
}

pub fn limit_enemy_movements(
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &mut Enemy, &Handle<Image>)>,
    asset: Res<Assets<Image>>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    for (mut transform, mut enemy, image) in query.iter_mut() {
        let window = window.get_single().unwrap();
        let mut size = Vec2::ZERO;
        if let Some(image) = asset.get(image) {
            size = image.size();
        }
        let min_x = size.x / 2.0;
        let min_y = size.y / 2.0;
        let max_x = window.width() - min_x;
        let max_y = window.height() - min_y;

        let mut bonk = false;
        if transform.translation.x < min_x {
            transform.translation.x = min_x;
            enemy.direction.x *= -1.0;
            bonk = true;
        } else if transform.translation.x > max_x {
            transform.translation.x = max_x;
            enemy.direction.x *= -1.0;
            bonk = true;
        }
        if transform.translation.y < min_y {
            transform.translation.y = min_y;
            enemy.direction.y *= -1.0;
            bonk = true;
        } else if transform.translation.y > max_y {
            transform.translation.y = max_y;
            enemy.direction.y *= -1.0;
            bonk = true;
        }
        if bonk {
            let sound1 = assets.load("audio/pluck_001.ogg");
            let sound2 = assets.load("audio/pluck_002.ogg");

            let sound = if random() { sound1 } else { sound2 };
            // audio.play(sound);
            commands.spawn(
                AudioBundle {
                    source: sound,
                    settings: Default::default(),
                }
            );
        }
    }
}
pub fn tick_enemy_timer(mut enemy_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_timer.timer.tick(time.delta());
}
pub fn reset_enemy_timer(mut enemy_timer: ResMut<EnemySpawnTimer>) {
    enemy_timer.timer.reset();
}

pub fn spawn_next_enemy(
    mut commands: Commands,
    assets: Res<AssetServer>,
    enemy_timer: Res<EnemySpawnTimer>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if enemy_timer.timer.finished() {
        let mut enemies = enemy_query.iter();
        let mut generatror = rand::thread_rng();
        let n = generatror.gen_range(0..enemy_query.iter().count());
        let pos = enemies.nth(n).unwrap();
        commands.spawn((
            SpriteBundle {
                transform: *pos,
                texture: assets.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5)
                    .normalize_or_zero(),
            },
        ));
    }
}

pub fn player_hit_enemy(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    assets: Res<AssetServer>,
    score: Res<Score>,
    mut game_over: EventWriter<GameOver>,
) {
    for (entity, transform) in player_query.iter() {
        for enemy_transform in enemy_query.iter() {
            let delta_x = enemy_transform.translation.x - transform.translation.x;
            let delta_y = enemy_transform.translation.y - transform.translation.y;
            if delta_x * delta_x + delta_y * delta_y < 64.0 * 64.0 {
                commands.spawn(
                    AudioBundle {
                        source: assets.load("audio/explosionCrunch_000.ogg"),
                        settings: Default::default(),
                    }
                );
                commands.entity(entity).despawn();
                game_over.send(GameOver::from_score(&score));
            }
        }
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
}
