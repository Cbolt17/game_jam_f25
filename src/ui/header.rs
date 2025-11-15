use bevy::prelude::*;
use crate::casino::CasinoMoney;

const CONTAINER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const CONTAINER_HEIGHT: Val = Val::Px(40.0);

const MONEY_TEXT_COLOR: Color = Color::srgb(0.8, 1.0, 0.8);

#[derive(Resource)]
pub struct MoneyDisplay {
    pub current: i64,
    pub change: i64
}

#[derive(Component)]
pub struct MoneyText;

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
            left: Val::Px(10.0),
            ..default()
        },
        Text::new(""),
        TextColor(MONEY_TEXT_COLOR),
        MoneyText
    );
    commands.spawn((container, children![money_text]));
}

pub fn update_header(
    money: Res<CasinoMoney>,
    mut money_display: ResMut<MoneyDisplay>,
    mut money_text: Single<&mut Text, With<MoneyText>>
) {
    money_display.current = std::cmp::min(money.0, money_display.current + money_display.change);
    ***money_text = format_money_text(money_display.current);
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
