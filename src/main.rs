mod snake;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands) {
    println!("Welcome to Snake Game in Rust!");

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.5, 0.5, 0.0),
                scale: Vec3::new(20.0, 20.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::hsl(0.0, 0.0, 0.8),
                ..default()
            },
            ..default()
        },
        snake::Snake
    ));
}
