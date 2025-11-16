use bevy::prelude::*;
use rand::Rng;
use crate::{grid::door::Door, peeps::{drunk::PassOut, peeps::Peep, play::{GoTo, Location}}};

#[derive(Component)]
#[relationship(relationship_target = CarriedPeep)]
pub struct Carrying(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = Carrying)]
pub struct CarriedPeep(Entity);

#[derive(Component)]
#[relationship(relationship_target = CarriedIntent)]
pub struct CarryingIntent(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = CarryingIntent)]
pub struct CarriedIntent(Entity);

#[derive(Event)]
pub struct SpawnServerEvent;

#[derive(Component)]
pub struct Server;

pub fn spawn_server(
    position: Vec2,
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
) {
    let mut rand = rand::thread_rng();
    let image = if rand.gen_bool(0.5) {"BellHop.png"} else {"BellHop2.png"};
    commands.spawn((
        Peep,
        Server,
        Sprite {
            image: asset_server.load(image),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
    ));
}

pub fn server_spawner(
    _event: On<SpawnServerEvent>,
    door: Single<&Transform, With<Door>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    spawn_server(door.translation.xy(), &mut commands, &asset_server);
}

pub fn server_target(
    passed_query: Query<Entity, (With<PassOut>, Without<Server>, Without<CarriedIntent>)>,
    server_query: Query<Entity, (With<Server>, Without<CarryingIntent>)>,
    mut commands: Commands,
) {
    let mut passed_iter = passed_query.iter();
    for entity in server_query.iter() {
        if let Some(passed_entity) = passed_iter.next() {
            let mut entity = commands.entity(entity);
            entity.insert(CarryingIntent(passed_entity));
            entity.insert(GoTo(passed_entity));
        }
    }
}