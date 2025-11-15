use bevy::prelude::*;

#[derive(Component)]
#[relationship(relationship_target = PlayedBy)]
struct Playing(Entity);

#[derive(Component)]
#[relationship_target(relationship = Playing)]
struct PlayedBy(Vec<Entity>);