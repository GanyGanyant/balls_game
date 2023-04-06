use bevy::prelude::*;
mod systems;
use systems::*;

pub const ENEMY_SPEED: f32 = 256.0;
pub const NUM_OF_ENEMIES: usize = 7;
pub const ENEMY_SPAWN_TIME: f32 = 10.0;

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
            .add_startup_system(spawn_enemies)
            .add_systems((enemy_movement, player_hit_enemy, limit_enemy_movements).chain())
            .add_systems((tick_enemy_timer, spawn_next_enemy).chain());
    }
}
