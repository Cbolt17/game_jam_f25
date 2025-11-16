use bevy::prelude::*;
use crate::{casino::{CasinoMoney, PeepCount, Suspicion}, peeps::peeps::Peep, ui::utils::format_money_text};

const CONTAINER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const CONTAINER_HEIGHT: Val = Val::Px(40.0);

const MONEY_TEXT_COLOR: Color = Color::srgb(0.8, 1.0, 0.8);
const MONEY_TEXT_COLOR_INC: Color = Color::srgb(0.5, 1.0, 0.7);
const MONEY_TEXT_COLOR_DEC: Color = Color::srgb(1.0, 0.5, 0.5);

const CAPACITY_TEXT_COLOR: Color = Color::srgb(0.8, 0.8, 1.0);

const GAUGE_RED: Color = Color::srgb(1.0, 0.3, 0.2);
const GAUGE_GREEN: Color = Color::srgb(0.4, 1.0, 0.2);
const GAUGE_BORDER: Color = Color::srgb(0.2, 0.2, 0.3);

#[derive(Resource)]
pub struct MoneyDisplay {
    pub current: i64,
    pub change: i64
}

#[derive(Component)]
pub struct MoneyText;

#[derive(Component)]
pub struct CapacityText;

#[derive(Component)]
pub struct SusMarker;

pub fn create_header(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    let sus_gauge = (
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(35.0),
            width: Val::Percent(30.0),
            height: Val::Percent(60.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BorderRadius::all(Val::Px(16.0)),
        BorderColor::all(GAUGE_BORDER),
        BackgroundGradient::from(LinearGradient{
            color_space: InterpolationColorSpace::Oklaba,
            angle: std::f32::consts::PI * 0.5,
            stops: vec![
                GAUGE_RED.into(),
                GAUGE_GREEN.into()
            ]
        })
    );
    let sus_marker = (
        Node {
            left: Val::Percent(50.0),
            top: Val::Px(6.0),
            ..default()
        },
        ImageNode::new(asset_server.load("GaugeMarker.png")),
        SusMarker
    );
    commands.spawn((container, children![money_text, (sus_gauge, children![sus_marker]), capacity_text]));
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
    peep_query: Query<(), With<Peep>>,
    mut capacity_text: Single<&mut Text, With<CapacityText>>
) {
    ***capacity_text = format!("Peeps: {}", peep_query.count());
}

pub fn update_sus_gauge(
    suspicion: Res<Suspicion>,
    mut marker_node: Single<&mut Node, With<SusMarker>>
) {
    marker_node.left = Val::Percent((suspicion.0 - 1.0) * -100.0)
}
