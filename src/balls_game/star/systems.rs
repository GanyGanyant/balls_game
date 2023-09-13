use bevy::window::PrimaryWindow;

use super::super::player::*;

use super::*;

pub fn spawn_stars(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = window.get_single().unwrap();
    let mut generatror = rand::thread_rng();
    let (x, y) = (window.width(), window.height());
    for _ in 0..NUM_OF_STARS {
        let x = generatror.gen_range(16.0..x - 16.0);
        let y = generatror.gen_range(16.0..y - 16.0);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: assets.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn tick_star_timer(mut star_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_timer.timer.tick(time.delta());
}

pub fn reset_star_timer(mut star_timer: ResMut<StarSpawnTimer>) {
    star_timer.timer.reset();
}

pub fn player_hit_star(
    mut command: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    assets: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    for transform in player_query.iter() {
        for (entity, star_transform) in star_query.iter() {
            let delta_x = star_transform.translation.x - transform.translation.x;
            let delta_y = star_transform.translation.y - transform.translation.y;
            if delta_x * delta_x + delta_y * delta_y < 47.0 * 47.0 {
                score.value += 1;
                command.spawn(
                    AudioBundle {
                        source: assets.load("audio/laserLarge_000.ogg"),
                        settings: Default::default(),
                    }
                );
                command.entity(entity).despawn();
            }
        }
    }
}

pub fn spawn_next_star(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
    star_timer: Res<StarSpawnTimer>,
) {
    let window = window.get_single().unwrap();
    let mut generatror = rand::thread_rng();
    let (x, y) = (window.width(), window.height());
    if star_timer.timer.finished() {
        let x = generatror.gen_range(16.0..x - 16.0);
        let y = generatror.gen_range(16.0..y - 16.0);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: assets.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for entity in star_query.iter() {
        commands.entity(entity).despawn();
    }
}
