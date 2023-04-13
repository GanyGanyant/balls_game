use bevy::prelude::*;

mod styles;

mod game_over_menu;
mod hud;
mod main_menu;
mod pause_menu;

use main_menu::MainMenuPlugin;

use self::{hud::HUDPlugin, pause_menu::PauseMenuPlugin, game_over_menu::GameOverMenuPlugin};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MainMenuPlugin)
            .add_plugin(HUDPlugin)
            .add_plugin(PauseMenuPlugin)
            .add_plugin(GameOverMenuPlugin);
    }
}
