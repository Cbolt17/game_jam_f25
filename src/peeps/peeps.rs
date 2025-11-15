use bevy::prelude::*;

#[derive(Resource)]
pub struct PeepSheet(Handle<TextureAtlasLayout>);

#[derive(Component)]
pub struct Peep;

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