use bevy::prelude::*;

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(game::background::BackgroundPlugin)
        .add_plugins(game::music::MusicPlugin)
        .add_plugins(game::alphabet::AlphabetPlugin)
        .run();
}
