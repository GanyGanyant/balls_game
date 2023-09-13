use bevy::prelude::*;
use rand::prelude::*;

mod systems;

use systems::*;

use crate::balls_game::{AppState, GameState};

use super::GameEvents;

pub const NUM_OF_STARS: usize = 10;
pub const STAR_SPAWN_TIME: f32 = 1.0;

#[derive(Component)]
pub struct Star;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::InGame), spawn_stars)
            .add_systems(OnExit(AppState::InGame), despawn_stars)
            .add_systems(Update,
                         player_hit_star
                             .in_set(GameEvents)
                             .run_if(in_state(GameState::Running))
                             .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::InGame), reset_star_timer)
            .add_systems(Update,
                         (tick_star_timer, spawn_next_star)
                             .after(reset_star_timer)
                             .run_if(in_state(GameState::Running))
                             .run_if(in_state(AppState::InGame))
                             .chain(),
            );
    }
}
