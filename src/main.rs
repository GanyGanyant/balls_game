use balls_game::BallsGame;
use bevy::prelude::*;

pub mod balls_game;
mod enemy;
mod player;
mod star;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BallsGame)
        .run();
}
