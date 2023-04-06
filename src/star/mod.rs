use bevy::prelude::*;
use rand::prelude::*;
mod systems;
use systems::*;

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
        .add_startup_system(spawn_stars)
        .add_system(player_hit_star)
        .add_systems((tick_star_timer, spawn_next_star).chain());
    }
}
