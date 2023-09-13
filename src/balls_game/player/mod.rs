use bevy::prelude::*;

mod systems;

use systems::*;

use crate::balls_game::{AppState, GameState};

pub const PLAYER_SPEED: f32 = 512.0;

#[derive(Component)]
pub struct Player;

#[derive(Resource, Default, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl HighScores {
    pub fn get_high_score(&self) -> u32 {
        let mut max = 0;
        for (_, score) in self.scores.iter() {
            if *score > max {
                max = *score;
            }
        }
        max
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}

impl GameOver {
    pub fn from_score(score: &Score) -> Self {
        Self { score: score.value }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_event::<GameOver>()
            .add_systems(OnEnter(AppState::InGame), insert_score)
            //.add_systems(OnEnter(AppState::InGame), remove_score)
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(OnExit(AppState::InGame), despawn_player)
            .add_systems(Update,
                         (player_movement, limit_player_movements)
                             .run_if(in_state(GameState::Running))
                             .run_if(in_state(AppState::InGame))
                             .chain(),
            )
            .add_systems(Update, (exit_game, game_over, update_high_scores));
    }
}
