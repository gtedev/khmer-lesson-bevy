use std::time::Duration;

use bevy::prelude::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_music);
    }
}

fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let music: Handle<AudioSource> = asset_server.load("khmer_music.ogg");

        commands.spawn((
        AudioPlayer::new(music),
        PlaybackSettings::LOOP
            .with_start_position(Duration::from_secs(3))
            .with_volume(bevy::audio::Volume::Linear(0.3)),
    ));
}
