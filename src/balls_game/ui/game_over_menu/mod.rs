use bevy::prelude::*;

use crate::balls_game::AppState;

use layout::*;

mod components;
mod layout;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
