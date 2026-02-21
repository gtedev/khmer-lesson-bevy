use bevy::prelude::*;

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(game::background:: BackgroundPlugin)
        // .add_systems(Update, hello_world)
        // .add_systems(Update, hello_world)
        .run();
}

fn hello_world() {
    println!("hello world!");
}