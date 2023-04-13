use bevy::prelude::*;

use crate::balls_game::{
    player::Score,
    ui::{components::*, styles::*},
};

use super::components::*;

pub fn despawn_game_over_menu(mut commands: Commands, query: Query<Entity, With<GameOverMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    commands
        .spawn((
            NodeBundle {
                style: GLOOM_STYLE,
                background_color: GLOOM.into(),
                ..default()
            },
            GameOverMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: BOX_STYLE,
                    background_color: BOX_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // * Title
                    parent
                        .spawn((
                            NodeBundle {
                                style: TITLE_STYLE,
                                ..default()
                            },
                            Title,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Game Over",
                                        get_text_style(&asset_server, 64.0),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // * Score
                    parent
                        .spawn((
                            NodeBundle {
                                style: TITLE_STYLE,
                                ..default()
                            },
                            Title,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        format!("Score: {}", score.value),
                                        get_text_style(&asset_server, 48.0),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // * PlayAgain
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: BackgroundColor(NORMAL_BUTTON_COLOR),
                                ..default()
                            },
                            PlayButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Play Again",
                                        get_text_style(&asset_server, 32.0),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // * MainMenu
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: BackgroundColor(NORMAL_BUTTON_COLOR),
                                ..default()
                            },
                            MainMenuButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Main Menu",
                                        get_text_style(&asset_server, 32.0),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // * Quit
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: BackgroundColor(NORMAL_BUTTON_COLOR),
                                ..default()
                            },
                            QuitButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Quit",
                                        get_text_style(&asset_server, 32.0),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        });
}
