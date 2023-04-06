use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use super::enemy::EnemyPlugin;
use super::player::PlayerPlugin;
use super::star::StarPlugin;

pub struct BallsGame;

impl PluginGroup for BallsGame {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add(EnemyPlugin).add(PlayerPlugin).add(StarPlugin);
        group
    }
}
