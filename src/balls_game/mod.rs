use bevy::prelude::*;

mod enemy;
mod main_menu;
mod player;
mod star;

mod systems;
use systems::*;

use enemy::EnemyPlugin;
use main_menu::MainMenuPlugin;
use player::PlayerPlugin;
use star::StarPlugin;

pub struct BallsGame;

impl Plugin for BallsGame {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<AppState>()
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(StarPlugin)
            .add_plugin(MainMenuPlugin)
            .add_system(toggle_game.run_if(in_state(AppState::InGame)))
            .add_system(pause_game.in_schedule(OnEnter(AppState::InGame)))
            .add_systems((transition_to_game, transition_to_main_menu));
    }
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}
