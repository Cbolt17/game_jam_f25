use bevy::prelude::*;

use crate::{casino::Suspicion, grid::door::Door, peeps::{drunk::{Die, PassOut}, peeps::Peep, play::{GoTo, Location}, server::CarriedPeep}};

#[derive(Component)]
#[relationship(relationship_target = MonitoredBy)]
pub struct Monitoring(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = Monitoring)]
pub struct MonitoredBy(Vec<Entity>);

impl MonitoredBy {
    pub fn inspectors(&self) -> Vec<Entity> {
        self.0.clone()
    }
}

#[derive(Event)]
pub struct SpawnInspectorEvent;

#[derive(Resource)]
pub struct InspectorSpawner {
    rate: f32,
    countdown: f32,
}

impl InspectorSpawner {
    pub fn new(rate: f32, countdown: f32) -> Self {
        InspectorSpawner{rate, countdown}
    }
    pub fn cycle_back(&mut self) {
        self.countdown += self.rate;
    }
    pub fn reset(&mut self) {
        self.countdown = self.rate;
    }
}

#[derive(Component)]
pub struct Inspector;

pub fn spawn_inspector(
    position: Vec2,
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        Peep,
        Inspector,
        Sprite {
            image: asset_server.load("Inspector.png"),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
    ));
}

pub fn inspector_spawner(
    _event: On<SpawnInspectorEvent>,
    door: Single<&Transform, With<Door>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    spawn_inspector(door.translation.xy(), &mut commands, &asset_server);
}

pub fn inspector_spawner_timer(
    mut spawner: ResMut<InspectorSpawner>,
    time: Res<Time>,
    mut commands: Commands,
) {
    spawner.countdown -= time.delta_secs();
    while spawner.countdown < 0.0 {
        spawner.cycle_back();
        commands.trigger(SpawnInspectorEvent);
    }
}

pub fn inspector_target(
    passed_query: Query<Entity, (With<PassOut>, Without<Inspector>, Without<CarriedPeep>)>,
    inspector_query: Query<Entity, (With<Inspector>, Without<GoTo>, Without<Monitoring>)>,
    mut commands: Commands,
) {
    let mut passed_iter = passed_query.iter();
    for entity in inspector_query.iter() {
        if let Some(passed_entity) = passed_iter.next() {
            let mut entity = commands.entity(entity);
            entity.insert(GoTo(passed_entity));
        }
    }
}

pub fn inspector_taken(
    carried: On<Add, CarriedPeep>,
    query: Query<&MonitoredBy>,
    loc_query: Query<&Location>,
    mut commands: Commands
) {
    if let Ok(monitored) = query.get(carried.entity) {
        for monitor in monitored.inspectors() {
            commands.entity(monitor).remove::<Monitoring>();
        }
    }
    if let Ok(location) = loc_query.get(carried.entity) {
        for inspector in location.entities() {
            commands.entity(inspector).remove::<GoTo>();
        }
    }
}

pub fn inspector_target_die(
    dead: On<Die>,
    query: Query<&MonitoredBy>,
    mut sus: ResMut<Suspicion>,
    mut commands: Commands
) {
    if let Ok(monitored) = query.get(dead.0) {
        for monitor in monitored.inspectors() {
            sus.0 += 0.10;
            if sus.0 > 1.0 {
                sus.0 = 1.0
            }
            commands.entity(monitor).remove::<Monitoring>();
        }
    }
}