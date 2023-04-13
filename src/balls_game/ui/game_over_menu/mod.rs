use bevy::prelude::*;

use crate::balls_game::AppState;

use layout::*;

mod components;
mod layout;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_game_over_menu.in_schedule(OnEnter(AppState::GameOver)))
            .add_system(despawn_game_over_menu.in_schedule(OnExit(AppState::GameOver)));
    }
}
