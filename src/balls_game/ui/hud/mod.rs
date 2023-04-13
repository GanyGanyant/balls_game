use bevy::prelude::*;

use crate::balls_game::AppState;

use components::*;
use layout::*;

mod components;
mod layout;
pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_hud.in_schedule(OnEnter(AppState::InGame)))
            .add_system(despawn_hud.in_schedule(OnExit(AppState::InGame)))
            .add_systems((update_enemy_text, update_score_text).in_set(OnUpdate(AppState::InGame)));
    }
}
