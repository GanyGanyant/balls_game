use bevy::prelude::*;
mod systems;
use systems::*;

use crate::balls_game::{AppState, GameState};

use super::GameEvents;

pub const ENEMY_SPEED: f32 = 256.0;
pub const NUM_OF_ENEMIES: usize = 5;
pub const ENEMY_SPAWN_TIME: f32 = 8.0;

#[derive(Component)]

pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::InGame)))
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::InGame)))
            .add_system(
                player_hit_enemy
                    .in_set(GameEvents)
                    .run_if(in_state(GameState::Running))
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                (enemy_movement, limit_enemy_movements)
                    .in_set(OnUpdate(GameState::Running))
                    .in_set(OnUpdate(AppState::InGame))
                    .chain(),
            )
            .add_systems(
                (tick_enemy_timer, spawn_next_enemy)
                    .in_set(OnUpdate(GameState::Running))
                    .in_set(OnUpdate(AppState::InGame))
                    .chain(),
            );
    }
}
