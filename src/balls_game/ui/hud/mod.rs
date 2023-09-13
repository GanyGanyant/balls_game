use bevy::prelude::*;

use crate::balls_game::AppState;

use components::*;
use layout::*;

mod components;
mod layout;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_hud)
            .add_systems(OnExit(AppState::InGame), despawn_hud)
            .add_systems(Update, (update_enemy_text, update_score_text).run_if(in_state(AppState::InGame)));
    }
}
