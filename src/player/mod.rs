use bevy::prelude::*;
mod systems;
use systems::*;

pub const PLAYER_SPEED: f32 = 512.0;

#[derive(Component)]
pub struct Player;

#[derive(Resource, Default, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

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
            .add_startup_systems((spawn_player, spawn_camera))
            .add_systems((player_movement, limit_player_movements).chain())
            .add_systems(
                (
                    exit_game,
                    game_over,
                    update_score,
                    update_high_scores,
                    check_high_scores,
                )
                    .chain(),
            );
    }
}
