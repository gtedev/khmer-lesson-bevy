use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background);
    }
}

fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // On spawn une camera 2D
    commands.spawn(Camera2d);

    // On charge l’image comme texture

    // On spawn l’entité sprite avec la texture
    commands.spawn((
        Sprite::from_image(asset_server.load("angkor_background.jpg")),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}