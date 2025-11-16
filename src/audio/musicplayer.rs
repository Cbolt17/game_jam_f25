use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct Song;

pub fn play_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/casiohno.ogg")),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            start_position: Some(Duration::from_millis(2503)),
            ..default()
        },
        Song
    ));
}

// pub fn update_music(music_controller: Single<&AudioSink, With<Song>>) {
//     let song_pos = music_controller.position();
//     if song_pos.as_millis() > 71300 {
//         println!("loop time");
//         let target = song_pos - Duration::from_millis(69269);
//         let _res = music_controller.try_seek(target);
//         println!("{:?}", _res);
//     }
// }