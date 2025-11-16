use bevy::prelude::*;

use crate::{grid::{attraction::{Attraction, AvailableAttractions}, door::Door, grid::{AttractionGrid, CELL_SIZE}, play_attraction::BetResult}, peeps::{drunk::PassOut, play::{GoTo, GoalReached, Playing}, profile::{MoneyProfile, NoPlayRecord}, server::{Carrying, Server}}};

#[derive(Resource)]
pub struct PeepSheet(pub Handle<TextureAtlasLayout>);

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
    peep_query: Query<(Entity, &NoPlayRecord), (With<Peep>, Without<GoTo>, Without<Playing>)>,
    available: Res<AvailableAttractions>,
    door: Single<Entity, With<Door>>,
    mut commands: Commands,
) {
    for (peep, record) in peep_query.iter() {
        if record.0 >= 3 {
            commands.entity(peep).insert(GoTo(*door));
        }
        else if let Some(attraction) = available.random() {
            commands.entity(peep).insert(GoTo(attraction));
        }
        else {
            commands.entity(peep).insert(GoTo(*door));
        }
    }
}

pub fn peep_goto(
    door: Single<Entity, With<Door>>,
    mut peep_query: Query<(Entity, &GoTo, &mut Transform), (With<Peep>, Without<PassOut>)>,
    goal_query: Query<(Entity, &Transform), Or<(Without<Peep>, With<PassOut>)>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, goto, mut transform) in peep_query.iter_mut() {
        let (a_entity, a_transform) = goal_query.get(goto.0).unwrap().clone();
        let goal = a_transform.translation.xy();
        let mut location = transform.translation.xy();
        let goal_cell = AttractionGrid::get_cell(goal);
        let loc_cell = AttractionGrid::get_cell(location);
        let dif = goal_cell - loc_cell;
        if dif.x.abs() > dif.y.abs() && location.y >= -10.0 {
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
            let mut dif = pos - location;
            if goto.0 == *door {
                dif.y += 32.0;
            }
            if dif.length_squared() > 0.1 * CELL_SIZE {
                location += dif.normalize_or_zero() * PEEP_SPEED * time.delta_secs();
            }
            else {
                commands.trigger(GoalReached::new(entity, a_entity));
            }
        }
        transform.translation = location.extend(-location.y);
    }
}

pub fn peep_reach_attraction(
    reached: On<GoalReached>,
    door: Single<Entity, With<Door>>,
    mut peep_query: Query<(Entity, &mut NoPlayRecord), With<GoTo>>,
    mut attraction_query: Query<(Entity, &mut Attraction)>,
    mut commands: Commands
) {
    if let Ok((a_entity, mut attraction)) = attraction_query.get_mut(reached.location) {
        if let Ok((entity, mut record)) = peep_query.get_mut(reached.peep) {
            let mut entity = commands.entity(entity);
            if attraction.full() {
                record.0 += 1;
                if record.0 > 3 {
                    entity.insert(GoTo(*door));
                }
            }
            else {
                entity.remove::<GoTo>();
                attraction.add_player();
                entity.insert(Playing(a_entity));
            }
        }
    }
}

pub fn peep_reach_door(
    reached: On<GoalReached>,
    door: Single<Entity, With<Door>>,
    mut peep_query: Query<Entity, (With<GoTo>, Without<Server>)>,
    mut commands: Commands
) {
    if reached.location == *door {
        if let Ok(entity) = peep_query.get_mut(reached.peep) {
            let mut entity = commands.entity(entity);
            entity.despawn();
        }
    }
}

pub fn server_reach_peep(
    reached: On<GoalReached>,
    door: Single<Entity, With<Door>>,
    server_query: Query<Entity, (With<Server>, With<Carrying>)>,
    mut peep_query: Query<(Entity, &mut Transform), (With<Peep>, Without<Server>)>,
    mut commands: Commands
) {
    if let Ok((peep_entity, mut transform)) = peep_query.get_mut(reached.location) {
        if let Ok(entity) = server_query.get(reached.peep) {
            let mut server = commands.entity(entity);
            server.insert(GoTo(*door));
            server.add_child(peep_entity);
            transform.translation = Vec3::new(0.0, 8.0, -0.01);
        }
    }
}

pub fn server_reach_door(
    reached: On<GoalReached>,
    door: Single<Entity, With<Door>>,
    mut server_query: Query<(Entity, &Carrying), With<Server>>,
    mut commands: Commands
) {
    if reached.location == *door {
        if let Ok((entity, carrying)) = server_query.get_mut(reached.peep) {
            commands.entity(entity).remove::<Carrying>();
            commands.entity(carrying.0).despawn();
        }
    }
}

pub fn bet_result(
    bet_results: On<BetResult>,
    mut peep_query: Query<(&mut Sprite, &mut MoneyProfile), With<Peep>>,
) {
    if let Ok((mut sprite, mut money_profile)) = peep_query.get_mut(bet_results.entity) {
        money_profile.0 = (money_profile.0 as i64 + bet_results.amt) as u64;
        if bet_results.amt > 0 {
            make_peep_happy(&mut sprite);
        }
        else {
            make_peep_angy(&mut sprite);
        }
    }
}