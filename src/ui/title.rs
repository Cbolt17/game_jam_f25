use bevy::prelude::*;

use crate::game::end::{YouLoseEvent, YouWonEvent};

const WIN_TEXT_COLOR: Color = Color::srgb(0.5, 1.0, 0.7);
const LOSE_TEXT_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

#[derive(Component)]
pub struct UiContainer;

#[derive(Resource)]
pub struct TitleMessage {
    pub message: String
}

#[derive(Component)]
pub struct OptionalTitleText;

pub fn create_title(
    mut commands: Commands
) {
    let container = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        Visibility::Visible,
        UiContainer
    );
    let optional_text = (
        Node {
            ..default()
        },
        Text::new(""),
        TextColor(Color::BLACK),
        TextFont::from_font_size(64.0),
        TextLayout::new_with_justify(Justify::Center),
        OptionalTitleText
    );
    let title_text = (
        Node {
            ..default()
        },
        Text::new("Casino Game"),
        TextFont::from_font_size(48.0),
        TextLayout::new_with_justify(Justify::Center)
    );
    let hint_text = (
        Node {
            ..default()
        },
        Text::new("Press space to start"),
        TextFont::from_font_size(24.0),
        TextLayout::new_with_justify(Justify::Center)
    );
    commands.spawn((container, children![optional_text, title_text, hint_text]));
}

pub fn toggle_ui_visibility(
    query: Query<&mut Visibility, With<UiContainer>>,
) {
    for mut visibility in query {
        *visibility = match *visibility {
            Visibility::Hidden => Visibility::Visible,
            Visibility::Visible => Visibility::Hidden,
            Visibility::Inherited => Visibility::Inherited,
        };
    }
}

pub fn update_win_text(
    _win: On<YouWonEvent>,
    mut text: Single<&mut Text, With<OptionalTitleText>>,
    mut color: Single<&mut TextColor, With<OptionalTitleText>>
) {
    ***text = "You Win!".to_string();
    color.0 = WIN_TEXT_COLOR;
}

pub fn update_lose_text(
    loss: On<YouLoseEvent>,
    mut text: Single<&mut Text, With<OptionalTitleText>>,
    mut color: Single<&mut TextColor, With<OptionalTitleText>>
) {
    match *loss {
        YouLoseEvent::BankRupt => {***text = "You Went Bankrupt!".to_string();},
        YouLoseEvent::OutOfTime => {***text = "You Ran Out of Time!".to_string();},
        YouLoseEvent::Arrested => {***text = "You Were Arrested!".to_string();},
    }
    color.0 = LOSE_TEXT_COLOR;
}