use bevy::{app::AppExit, prelude::*};

use crate::balls_game::{ui::styles::*, AppState};

#[derive(Component)]
pub struct Title;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct MainMenuButton;

pub fn play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut bg_color) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON_COLOR.into();
                {
                    next_app_state.set(AppState::InGame);
                }
            }
            Interaction::Hovered => *bg_color = HOVER_BUTTON_COLOR.into(),
            Interaction::None => *bg_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut exit_evenr_writer: EventWriter<AppExit>,
) {
    for (interaction, mut bg_color) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON_COLOR.into();
                exit_evenr_writer.send(AppExit);
            }
            Interaction::Hovered => *bg_color = HOVER_BUTTON_COLOR.into(),
            Interaction::None => *bg_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut bg_color) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON_COLOR.into();
                {
                    next_app_state.set(AppState::MainMenu);
                }
            }
            Interaction::Hovered => *bg_color = HOVER_BUTTON_COLOR.into(),
            Interaction::None => *bg_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
