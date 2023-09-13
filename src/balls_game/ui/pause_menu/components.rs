use bevy::prelude::*;

use crate::balls_game::{ui::styles::*, GameState};

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct ResumeButton;

pub fn resume_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON_COLOR.into();
                {
                    next_game_state.set(GameState::Running);
                }
            }
            Interaction::Hovered => *bg_color = HOVER_BUTTON_COLOR.into(),
            Interaction::None => *bg_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
