use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVER_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.30, 0.15, 0.45);

pub const BUTTON_STYLE: Style = Style {
    size: Size {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
    },
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};

pub const IMAGE_STYLE: Style = Style {
    size: Size {
        width: Val::Px(64.0),
        height: Val::Px(64.0),
    },
    margin: UiRect {
        left: Val::Px(8.0),
        right: Val::Px(8.0),
        top: Val::Px(8.0),
        bottom: Val::Px(8.0),
    },
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};

pub const MAIN_MENU_STYLE: Style = Style {
    size: Size {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
    },
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    gap: Size {
        width: Val::Auto,
        height: Val::Percent(5.0),
    },
    ..Style::DEFAULT
};

pub const TITLE_STYLE: Style = Style {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size {
        width: Val::Px(300.0),
        height: Val::Px(120.0),
    },
    ..Style::DEFAULT
};

pub fn get_text_style(asset_server: &AssetServer, font_size: f32) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size,
        color: Color::WHITE,
    }
}
