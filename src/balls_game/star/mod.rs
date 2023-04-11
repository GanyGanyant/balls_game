use bevy::prelude::*;
use rand::prelude::*;
mod systems;
use systems::*;

use crate::{balls_game::{GameState, AppState}};

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
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::InGame)))
            .add_system(despawn_stars.in_schedule(OnExit(AppState::InGame)))
            .add_system(
                player_hit_star
                    .run_if(in_state(GameState::Running))
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                (tick_star_timer, spawn_next_star)
                    .in_set(OnUpdate(GameState::Running))
                    .in_set(OnUpdate(AppState::InGame))
                    .chain(),
            );
    }
}
