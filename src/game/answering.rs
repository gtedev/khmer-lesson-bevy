use bevy::prelude::*;


pub struct AnsweringPlugin;

impl Plugin for AnsweringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, number_key_system);
    }
}

fn number_key_system(keyboard_input: Res<ButtonInput<KeyCode>>,) {
    let numbers = [
        KeyCode::Digit0, KeyCode::Digit2, KeyCode::Digit3,
        KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6,
        KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9,
        KeyCode::Digit0,
    ];

    for (i, key) in numbers.iter().enumerate() {
        if keyboard_input.just_pressed(*key) {
            println!("Digit {} pressed", i);
        }
    }
}