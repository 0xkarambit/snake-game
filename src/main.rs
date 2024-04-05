use std::f32::consts::PI;

use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use derive_new::new;
use iyes_perf_ui::diagnostics::{PerfUiEntryFPS, PerfUiEntryFPSWorst};
use iyes_perf_ui::{PerfUiPlugin, PerfUiRoot};

// Window dimensions
const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HEIGHT: f32 = 720.;

// Sprites
const SPRITE_SHEET_PATH: &str = "assets.png";
const TILE_SIZE: Vec2 = Vec2::new(16., 16.);
const SPRITE_SHEET_COLUMNS: usize = 4;
const SPRITE_SHEET_ROWS: usize = 4;

// Colors
const BG_COLOR: Color = Color::rgb(248./255., 248./255., 209./255.);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Snake Game Bevy".to_string(),
                        resizable: true,
                        focused: true,
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(Msaa::Off)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, (
            setup,
            setup_camera,
            spawn_snake,
        ))
        .add_systems(Update, (
            snake_direction,
            snake_movement,
            close_on_esc
        ))
        .run();
}

fn setup(mut commands: Commands) {
    #[cfg(debug_assertions)]
    {
        commands.spawn((
            PerfUiRoot {
                display_labels: true,
                ..default()
            },
            PerfUiEntryFPSWorst::default(),
            PerfUiEntryFPS::default()
        ));
    };
}

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.translation.z = 1000.;
    camera_bundle.camera.hdr = true;
    camera_bundle.tonemapping = Tonemapping::None;

    commands.spawn(camera_bundle);
}

#[derive(Clone)]
pub enum Direction {
    Up, Down,
    Left, Right
}

#[derive(Component, new)]
pub struct SnakeHead {
    direction: Direction
}

fn spawn_snake(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let texture: Handle<Image> = asset_server.load(SPRITE_SHEET_PATH);
    let layout = TextureAtlasLayout::from_grid(TILE_SIZE, SPRITE_SHEET_COLUMNS, SPRITE_SHEET_ROWS, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands
        .spawn(
            SpriteSheetBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32f32, 32f32)),
                    ..default()
                },
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0
                },
                ..default()
            }
        )
        .insert(SnakeHead {
            direction: Direction::Up
        });
}

pub fn snake_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &SnakeHead), With<SnakeHead>>
) {
    let delta = 75. * time.delta_seconds();

    for (mut transform, snake) in query.iter_mut() {
        match snake.direction {
            Direction::Up => transform.translation.y += delta,
            Direction::Down => transform.translation.y -= delta,
            Direction::Left => transform.translation.x -= delta,
            Direction::Right => transform.translation.x += delta,
        }
    }
}

pub fn snake_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut SnakeHead), With<SnakeHead>>
) {
    for (mut transform, mut snake) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            snake.direction = Direction::Left;
            transform.rotation = Quat::from_rotation_z(PI/2.);
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            snake.direction = Direction::Right;
            transform.rotation = Quat::from_rotation_z(-PI/2.);
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            snake.direction = Direction::Down;
            transform.rotation = Quat::from_rotation_z(PI);
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            snake.direction = Direction::Up;
            transform.rotation = Quat::from_rotation_z(0.);
        }
    }
}
