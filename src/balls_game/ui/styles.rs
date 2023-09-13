use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVER_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.30, 0.15, 0.45);
pub const BOX_COLOR: Color = Color::rgb(0.40, 0.40, 0.40);
pub const GLOOM: Color = Color::rgba(0.15, 0.15, 0.15, 0.3);

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};

pub const IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(64.0);
    style.height = Val::Px(64.0);
    style.margin = UiRect {
        left: Val::Px(8.0),
        right: Val::Px(8.0),
        top: Val::Px(8.0),
        bottom: Val::Px(8.0),
    };
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};

pub const MAIN_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Percent(5.0);
    style
};

pub const TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(300.0);
    style.height = Val::Px(120.0);
    style
};

pub fn get_text_style(asset_server: &AssetServer, font_size: f32) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size,
        color: Color::WHITE,
    }
}

pub const HUD_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Auto;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceBetween;
    style.padding = UiRect {
        left: Val::Px(25.0),
        right: Val::Px(25.0),
        top: Val::Px(25.0),
        bottom: Val::Px(25.0),
    };
    style
};
pub const HUD_BOX_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Auto;
    style.height = Val::Auto;
    style.flex_direction = FlexDirection::Row;
    style.column_gap = Val::Px(32.0);
    style.padding = UiRect {
        left: Val::Px(32.0),
        right: Val::Px(32.0),
        top: Val::Px(24.0),
        bottom: Val::Px(24.0),
    };
    style.margin = UiRect {
        left: Val::Px(0.0),
        right: Val::Px(0.0),
        top: Val::Px(0.0),
        bottom: Val::Auto,
    };
    style
};
pub const HUD_BOX_IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(48.0);
    style.height = Val::Px(48.0);
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};
pub const BOX_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Auto;
    style.height = Val::Auto;
    style.margin = UiRect::all(Val::Auto);
    style.padding = UiRect::all(Val::Px(32.0));
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(10.0);
    style
};

pub const GLOOM_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.position_type = PositionType::Absolute;
    style
};

pub const TEXT_HOLDER_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Auto;
    style.height = Val::Auto;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(32.0);
    style.margin = UiRect {
        left: Val::Auto,
        right: Val::Auto,
        top: Val::Auto,
        bottom: Val::Px(32.0),
    };
    style
};
