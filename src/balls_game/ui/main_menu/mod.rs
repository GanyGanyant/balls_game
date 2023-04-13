use bevy::prelude::*;

use crate::balls_game::AppState;

use components::*;
use layout::*;

mod components;
mod layout;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_systems((play_button, quit_button).in_set(OnUpdate(AppState::MainMenu)));
    }
}
