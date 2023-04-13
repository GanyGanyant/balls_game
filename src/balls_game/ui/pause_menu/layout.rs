use bevy::prelude::*;

use crate::balls_game::ui::{components::*, styles::*};

use super::components::*;

pub fn despawn_pause_menu(mut commands: Commands, main_menu_query: Query<Entity, With<PauseMenu>>) {
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: GLOOM_STYLE,
                background_color: GLOOM.into(),
                ..default()
            },
            PauseMenu,
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
                                        "Pause Menu",
                                        get_text_style(&asset_server, 64.0),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // * Resume
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: BackgroundColor(NORMAL_BUTTON_COLOR),
                                ..default()
                            },
                            ResumeButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Resume",
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
