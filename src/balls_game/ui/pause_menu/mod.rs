use bevy::prelude::*;

use crate::balls_game::{AppState, GameState};

use components::*;
use layout::*;

mod components;
mod layout;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_pause_menu
                .in_schedule(OnEnter(GameState::Paused))
                .run_if(in_state(AppState::InGame)),
        )
        .add_system(despawn_pause_menu.in_schedule(OnExit(GameState::Paused)))
        .add_system(resume_button.in_set(OnUpdate(GameState::Paused)));
    }
}
