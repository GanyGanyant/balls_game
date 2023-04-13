use bevy::prelude::*;

use crate::balls_game::ui::styles::*;

use super::components::*;

pub fn despawn_hud(mut commands: Commands, main_menu_query: Query<Entity, With<GameOverlay>>) {
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: HUD_STYLE,
                ..default()
            },
            GameOverlay,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: HUD_BOX_STYLE,
                    background_color: GLOOM.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: HUD_BOX_IMAGE_STYLE,
                        image: asset_server.load("sprites/star.png").into(),
                        ..default()
                    });
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "0",
                                    get_text_style(&asset_server, 48.0),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ScoreDisplay,
                    ));
                });
            parent
                .spawn(NodeBundle {
                    style: HUD_BOX_STYLE,
                    background_color: GLOOM.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "7",
                                    get_text_style(&asset_server, 48.0),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        EnemyDisplay,
                    ));
                    parent.spawn(ImageBundle {
                        style: HUD_BOX_IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    });
                });
        });
}
