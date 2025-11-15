use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct AvailableAttractions(pub Vec<Entity>);

impl AvailableAttractions {
    pub fn random(&self) -> Entity {
        let amt = self.0.len();
        let mut random = rand::thread_rng();
        self.0[random.gen_range(0..amt)]
    }
}

pub fn get_available_attractions(
    query: Query<(Entity, &Attraction)>,
    mut available: ResMut<AvailableAttractions>,
) {
    available.0.clear();
    for (entity, attraction) in query.iter() {
        if !attraction.full() {
            available.0.push(entity);
        }
    }
}

#[derive(Component, Clone, Copy)]
pub enum AttractionType {
    Roulette,
    BlackJack,
    Bar
}

impl AttractionType {
    pub fn get_sprite(&self) -> String {
        match self {
            AttractionType::Roulette => "Roulette.png".to_string(),
            AttractionType::BlackJack => "BlackJack.png".to_string(),
            AttractionType::Bar => "Bar.png".to_string(),
        }
    }
}

#[derive(Resource)]
pub struct AttractionBlueprints {
    blueprints: Vec<Attraction>
}

impl AttractionBlueprints {
    pub fn new() -> Self {
        AttractionBlueprints { 
            blueprints: vec![
                Attraction::new(5000, 10, 10.0, 0.51,  100, 5), // Roulette
                Attraction::new(10000, 5,  5.0,  0.55,  200, 5),  // BlackJack
                Attraction::new(19167, 8,  30.0, 100.0, 20,  5),  // Bar
            ]
        }
    }
    pub fn get(&self, attraction: AttractionType) -> Attraction {
        match attraction {
            AttractionType::Roulette => self.blueprints[0].dup(),
            AttractionType::BlackJack => self.blueprints[1].dup(),
            AttractionType::Bar => self.blueprints[2].dup(),
        }
    }
}

#[derive(Component)]
pub struct Attraction {
    pub cost: i64,
    players: u32,
    capacity: u32,
    cooldown: f32,
    win_rate: f32,
    max_bet: i64,
    min_bet: i64,
}

impl Attraction {
    pub fn new(cash: i64, max_cap: u32, cooldown: f32, win_rate: f32, max_bet: i64, min_bet: i64) -> Self {
        Attraction{cost: cash, players: 0, capacity: max_cap, cooldown, win_rate, max_bet, min_bet}
    }
    pub fn dup(&self) -> Self {
        Attraction::new(self.cost, self.capacity, self.cooldown, self.win_rate, self.max_bet, self.min_bet)
    }
    pub fn add_player(&mut self) -> bool {
        if self.players < self.capacity {
            self.players += 1;
            return true;
        }
        false
    }
    pub fn set_capacity(&mut self, amt: u32) {
        self.capacity = amt;
    }
    pub fn set_cooldown(&mut self, time: f32) {
        self.cooldown = time;
    }
    pub fn set_win_rate(&mut self, rate: f32) {
        self.win_rate = rate;
    }
    pub fn full(&self) -> bool {
        self.players < self.capacity
    }
}

pub fn spawn_attraction(
    position: Vec2,
    attraction: &AttractionType,
    blueprints: &Res<AttractionBlueprints>,
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands.spawn((
        Sprite {
            image: asset_server.load(attraction.get_sprite()),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, position.y),
        blueprints.get(*attraction)
    )).id()
}