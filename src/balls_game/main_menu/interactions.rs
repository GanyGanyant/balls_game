use bevy::{app::AppExit, prelude::*};

use crate::balls_game::AppState;

use super::{components::*, styles::*};

pub fn play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut bg_color) in button_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *bg_color = PRESSED_BUTTON_COLOR.into();
                app_next_state.set(AppState::InGame);
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
            Interaction::Clicked => {
                *bg_color = PRESSED_BUTTON_COLOR.into();
                exit_evenr_writer.send(AppExit);
            }
            Interaction::Hovered => *bg_color = HOVER_BUTTON_COLOR.into(),
            Interaction::None => *bg_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
