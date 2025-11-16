use bevy::prelude::*;

use crate::{grid::{attraction::AvailableAttractions, grid::{AttractionGrid, CELL_SIZE}, play_attraction::BetResult}, peeps::{play::{GoTo, Playing}, profile::{BetProfile, MoneyProfile, RiskProfile}}};

#[derive(Resource)]
pub struct PeepSheet(Handle<TextureAtlasLayout>);

#[derive(Component)]
pub struct Peep;

const PEEP_SPEED: f32 = 40.0;

impl FromWorld for PeepSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            (16, 16).into(), // The size of each image
            2,               // The number of columns
            1,               // The number of rows
            None,            // Padding
            None,            // Offset
        );

        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        Self(texture_atlas_handle)
    }
}

pub fn spawn_peep(
    position: Vec2,
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    peep_sheet: &Res<PeepSheet>,
) {
    let money_profile = MoneyProfile(100);
    let risk = RiskProfile::Normal;
    let bets = BetProfile::new(5, 80);
    commands.spawn((
        Peep,
        Sprite {
            image: asset_server.load("PeepSheet.png"),
            texture_atlas: Some(TextureAtlas {
                layout: peep_sheet.0.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        money_profile,
        risk,
        bets
    ));
}

pub fn make_peep_angy(sprite: &mut Sprite) {
    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = 1;
    }
}

pub fn make_peep_happy(sprite: &mut Sprite) {
    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = 0;
    }
}

pub fn peep_target(
    peep_query: Query<Entity, (With<Peep>, Without<GoTo>, Without<Playing>)>,
    available: Res<AvailableAttractions>,
    mut commands: Commands,
) {
    for peep in peep_query.iter() {
        if let Some(attraction) = available.random() {
            commands.entity(peep).insert(GoTo(attraction));
        }
    }
}

pub fn peep_goto(
    mut peep_query: Query<(Entity, &GoTo, &mut Transform), With<Peep>>,
    attraction_query: Query<(Entity, &Transform), Without<Peep>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, goto, mut transform) in peep_query.iter_mut() {
        let (a_entity, a_transform) = attraction_query.get(goto.0).unwrap().clone();
        let goal = a_transform.translation.xy();
        let mut location = transform.translation.xy();
        let goal_cell = AttractionGrid::get_cell(goal);
        let loc_cell = AttractionGrid::get_cell(location);
        let dif = goal_cell - loc_cell;
        if dif.x.abs() > dif.y.abs() {
            if dif.x != 0 {
                location.x += (dif.x / dif.x.abs()) as f32 * PEEP_SPEED * time.delta_secs();
            }
        }
        else {
            if dif.y != 0 {
                location.y += (dif.y / dif.y.abs()) as f32 * PEEP_SPEED * time.delta_secs();
            }
        }
        if dif.x == 0 && dif.y == 0 {
            let pos = AttractionGrid::get_coords(goal_cell) + Vec2::new(0.5 * CELL_SIZE, 0.2 * CELL_SIZE);
            let dif = pos - location;
            if dif.length_squared() > 0.1 * CELL_SIZE {
                location += dif.normalize_or_zero() * PEEP_SPEED * time.delta_secs();
            }
            else {
                commands.entity(entity).remove::<GoTo>();
                commands.entity(entity).insert(Playing(a_entity));
            }
        }
        transform.translation = location.extend(-location.y);
    }
}

pub fn bet_result(
    bet_results: On<BetResult>,
    mut peep_query: Query<&mut MoneyProfile, With<Peep>>,
) {
    if let Ok(mut money_profile) = peep_query.get_mut(bet_results.entity) {
        money_profile.0 = (money_profile.0 as i64 - bet_results.amt) as u64;
    }
}