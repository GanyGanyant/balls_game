use bevy::prelude::*;

use layout::*;

use self::interactions::{play_button, quit_button};

use super::AppState;

// * systems
mod interactions;
mod layout;

mod components;
mod styles;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_systems((play_button, quit_button).in_set(OnUpdate(AppState::MainMenu)));
    }
}
