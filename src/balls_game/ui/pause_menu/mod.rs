use bevy::prelude::*;

use crate::balls_game::{AppState, GameState};

use components::*;
use layout::*;

mod components;
mod layout;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused),
                        spawn_pause_menu
                            .run_if(in_state(AppState::InGame)),
        )
            .add_systems(OnExit(GameState::Paused), despawn_pause_menu)
            .add_systems(Update, resume_button.run_if(in_state(GameState::Paused)));
    }
}
