use bevy::prelude::*;

use crate::balls_game::{enemy::Enemy, player::Score};

#[derive(Component)]
pub struct GameOverlay;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct EnemyDisplay;

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreDisplay>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = score.value.to_string();
        }
    }
}

pub fn update_enemy_text(
    mut text_query: Query<&mut Text, With<EnemyDisplay>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let count = enemy_query.iter().count();
    for mut text in text_query.iter_mut() {
        text.sections[0].value = count.to_string();
    }
}
