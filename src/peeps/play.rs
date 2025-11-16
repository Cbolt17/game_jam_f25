use bevy::prelude::*;

#[derive(Component)]
#[relationship(relationship_target = Players)]
pub struct Playing(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = Playing)]
pub struct Players(Vec<Entity>);

impl Players {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn get_players(&self) -> Vec<Entity> {
        self.0.clone()
    }
}

#[derive(Component)]
#[relationship(relationship_target = Location)]
pub struct GoTo(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = GoTo)]
pub struct Location(Vec<Entity>);