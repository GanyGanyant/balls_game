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
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(UIPlugin)
            .add_systems(Update, toggle_game.run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(AppState::InGame),
                         unpause_game
                             .before(GameEvents),
            )
            .add_systems(OnExit(AppState::InGame), unpause_game)
            .add_systems(Update, (transition_to_game, transition_to_main_menu));
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
