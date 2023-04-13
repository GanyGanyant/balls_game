use super::*;
pub fn transition_to_game(
    mut next_app_state: ResMut<NextState<AppState>>,
    kbd_in: Res<Input<KeyCode>>,
    game_state: Res<State<AppState>>,
) {
    if kbd_in.just_pressed(KeyCode::G) {
        match game_state.0 {
            AppState::InGame => (),
            _ => {
                next_app_state.set(AppState::InGame);
            }
        }
    }
}

pub fn transition_to_main_menu(
    mut next_app_state: ResMut<NextState<AppState>>,
    kbd_in: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if kbd_in.just_pressed(KeyCode::M) {
        match app_state.0 {
            AppState::MainMenu => (),
            _ => {
                next_app_state.set(AppState::MainMenu);
            }
        }
    }
}

pub fn unpause_game(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Running);
}

pub fn toggle_game(
    mut next_game_state: ResMut<NextState<GameState>>,
    kbd_in: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
) {
    if kbd_in.just_pressed(KeyCode::Space) {
        match game_state.0 {
            GameState::Paused => next_game_state.set(GameState::Running),
            GameState::Running => next_game_state.set(GameState::Paused),
        }
    }
}
