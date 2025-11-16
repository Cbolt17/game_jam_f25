use bevy::prelude::*;

#[derive(Component)]
pub struct UiContainer;

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
    commands.spawn((container, children![title_text, hint_text]));
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