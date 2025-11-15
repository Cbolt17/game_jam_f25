use bevy::prelude::*;
use crate::casino::{CasinoCapacity, CasinoMoney};

const CONTAINER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const CONTAINER_HEIGHT: Val = Val::Px(40.0);

const MONEY_TEXT_COLOR: Color = Color::srgb(0.8, 1.0, 0.8);
const MONEY_TEXT_COLOR_INC: Color = Color::srgb(0.5, 1.0, 0.7);
const MONEY_TEXT_COLOR_DEC: Color = Color::srgb(1.0, 0.5, 0.5);

const CAPACITY_TEXT_COLOR: Color = Color::srgb(0.8, 0.8, 1.0);

#[derive(Resource)]
pub struct MoneyDisplay {
    pub current: i64,
    pub change: i64
}

#[derive(Component)]
pub struct MoneyText;

#[derive(Component)]
pub struct CapacityText;

pub fn create_header(mut commands: Commands) {
    let container = (
        BackgroundColor(CONTAINER_COLOR),
        Node {
        width: Val::Percent(100.0),
        height: CONTAINER_HEIGHT,
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        ..default()
        }
    );
    let money_text = (
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            ..default()
        },
        Text::new(""),
        TextColor(MONEY_TEXT_COLOR),
        MoneyText
    );
    let capacity_text = (
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            ..default()
        },
        Text::new(""),
        TextColor(CAPACITY_TEXT_COLOR),
        CapacityText
    );
    commands.spawn((container, children![money_text, capacity_text]));
}

pub fn update_money_text(
    money: Res<CasinoMoney>,
    mut money_display: ResMut<MoneyDisplay>,
    mut money_text: Single<&mut Text, With<MoneyText>>,
    mut text_color: Single<&mut TextColor, With<MoneyText>>
) {
    if money.0 > money_display.current {
        money_display.current = std::cmp::min(money.0, money_display.current + money_display.change);
        text_color.0 = MONEY_TEXT_COLOR_INC;
    } else if money.0 < money_display.current {
        text_color.0 = MONEY_TEXT_COLOR_DEC;
        money_display.current = std::cmp::max(money.0, money_display.current - money_display.change);
    } else {
        text_color.0 = MONEY_TEXT_COLOR;
    }
    ***money_text = format_money_text(money_display.current);
}

pub fn update_capacity_text(
    capacity: Res<CasinoCapacity>,
    mut capacity_text: Single<&mut Text, With<CapacityText>>
) {
    ***capacity_text = format!("Peeps: {}/{}", capacity.current, capacity.max);
}

fn format_money_text(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut count = 0;

    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }
    result.push('$');
    result.chars().rev().collect()
}
