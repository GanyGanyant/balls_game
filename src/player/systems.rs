use super::*;
use bevy::{window::PrimaryWindow, app::AppExit};

pub fn spawn_player(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = window.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: assets.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn player_movement(
    kbd_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if kbd_input.pressed(KeyCode::W) || kbd_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if kbd_input.pressed(KeyCode::A) || kbd_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if kbd_input.pressed(KeyCode::S) || kbd_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if kbd_input.pressed(KeyCode::D) || kbd_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        transform.translation +=
            direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn limit_player_movements(
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &Handle<Image>), With<Player>>,
    asset: Res<Assets<Image>>,
) {
    for (mut transform, image) in query.iter_mut() {
        let window = window.get_single().unwrap();
        let mut size = Vec2::ZERO;
        if let Some(image) = asset.get(image) {
            size = image.size();
        }
        let min_x = size.x / 2.0;
        let min_y = size.y / 2.0;
        let max_x = window.width() - min_x;
        let max_y = window.height() - min_y;
        if transform.translation.x < min_x {
            transform.translation.x = min_x;
        }
        if transform.translation.y < min_y {
            transform.translation.y = min_y;
        }
        if transform.translation.x > max_x {
            transform.translation.x = max_x;
        }
        if transform.translation.y > max_y {
            transform.translation.y = max_y;
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score is: {}", score.value);
    }
}
pub fn exit_game(kbd_input: Res<Input<KeyCode>>, mut exit_writer: EventWriter<AppExit>) {
    if kbd_input.just_pressed(KeyCode::Escape) {
        exit_writer.send(AppExit);
    }
}

pub fn game_over(mut game_over: EventReader<GameOver>) {
    for game_over_event in game_over.iter() {
        println!("Final score is: {}", game_over_event.score);
    }
}

pub fn update_high_scores(
    mut game_over: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for game_over_event in game_over.iter() {
        high_scores
            .scores
            .push(("Player".to_string(), game_over_event.score))
    }
}

pub fn check_high_scores(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High scores: {:?}", high_scores)
    }
}
