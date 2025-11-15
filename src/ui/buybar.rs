use bevy::prelude::*;

use crate::{grid::attraction::{AttractionBlueprints, AttractionType}, ui::utils::format_money_text};

const CONTAINER_HEIGHT: Val = Val::Percent(15.0);
const CONTAINER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

const ITEM_WIDTH: Val = Val::Px(80.0);
const BUTTON_TEXT_WIDTH : Val = Val::Px(64.0);
const BUTTON_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const BUTTON_COLOR_HOVER: Color = Color::srgb(0.57, 0.57, 0.57);
const BUTTON_COLOR_PRESS: Color = Color::srgb(0.45, 0.45, 0.45);

pub fn create_buybar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    attraction_blueprints: Res<AttractionBlueprints>
) {
    let container = (
        BackgroundColor(CONTAINER_COLOR),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            bottom: Val::Px(0.0),
            height: CONTAINER_HEIGHT,
            flex_direction: FlexDirection::Row,
            ..default()
        }
    );
    commands.spawn(container).with_children(
        |commands| {
            for attraction_type in vec![AttractionType::Roulette, AttractionType::BlackJack, AttractionType::Bar] {
                commands.spawn((
                    Button,
                    Node {
                        margin: UiRect::all(Val::Px(2.0)),
                        padding: UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(BUTTON_COLOR))
                ).with_children(|commands| {
                    commands.spawn((
                        Node {
                            height: ITEM_WIDTH,
                            width: ITEM_WIDTH,
                            ..default()
                        },
                        ImageNode::new(asset_server.load(attraction_type.get_sprite()))
                    ));
                    commands.spawn((Node {
                            bottom: Val::Px(0.0),
                            width: BUTTON_TEXT_WIDTH,
                            ..default()
                        },
                        Text::new(format_money_text(attraction_blueprints.get(attraction_type).cost)),
                        TextFont::from_font_size(14.0),
                        TextLayout::new_with_justify(Justify::Center)
                    ));
                }
                );
            }
        }
    );
}

pub fn update_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor
        ),
        (Changed<Interaction>, With<Button>)
    >
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BUTTON_COLOR_PRESS.into();
            }
            Interaction::Hovered => {
                *color = BUTTON_COLOR_HOVER.into();
            }
            Interaction::None => {
                *color = BUTTON_COLOR.into();
            }
        }
    }
}