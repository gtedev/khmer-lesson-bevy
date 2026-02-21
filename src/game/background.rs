use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background);
    }
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    // On spawn une camera 2D
    commands.spawn(Camera2d);

    // On charge l’image comme texture

    // On spawn l’entité sprite avec la texture
    commands.spawn((
        Sprite::from_image(asset_server.load("angkor_background.jpg")),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // left box
    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.95), // gris semi transparent
            custom_size: Some(Vec2::new(400.0, 500.0)),
            ..default()
        },
        Transform::from_xyz(-220.0, 0.0, 1.0), // Z = 1 (au dessus)
    ));

    // right box
    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.95),
            custom_size: Some(Vec2::new(400.0, 500.0)),
            ..default()
        },
        Transform::from_xyz(220.0, 0.0, 1.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.95),
            custom_size: Some(Vec2::new(1000.0, 150.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -350.0, 1.0),
    ));

    commands.spawn((
        Text2d::new("ក"), // caractère khmer
        TextFont {
            font: asset_server.load("fonts/khmer.ttf"),
            font_size: 160.0, // ajuste si trop gros / trop petit
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(-220.0, 0.0, 2.0), // même centre que la box, mais z plus grand
    ));
}
