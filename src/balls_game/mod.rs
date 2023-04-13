use bevy::prelude::*;

mod enemy;
mod player;
mod star;
mod ui;

mod systems;
use systems::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use star::StarPlugin;
use ui::UIPlugin;

pub struct BallsGame;

impl Plugin for BallsGame {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<AppState>()
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(StarPlugin)
            .add_plugin(UIPlugin)
            .add_system(toggle_game.run_if(in_state(AppState::InGame)))
            .add_system(
                unpause_game
                    .in_schedule(OnEnter(AppState::InGame))
                    .before(GameEvents),
            )
            .add_system(unpause_game.in_schedule(OnExit(AppState::InGame)))
            .add_systems((transition_to_game, transition_to_main_menu));
    }
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct GameEvents;
