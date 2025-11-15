use bevy::prelude::*;

#[derive(Component)]
#[relationship(relationship_target = PlayedBy)]
pub struct Playing(Entity);

#[derive(Component)]
#[relationship_target(relationship = Playing)]
pub struct PlayedBy(Vec<Entity>);

impl PlayedBy {
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