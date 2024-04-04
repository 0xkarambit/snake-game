mod snake;

use bevy::core::TaskPoolThreadAssignmentPolicy;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use iyes_perf_ui::prelude::*;
use bevy::tasks::available_parallelism;
use bevy::core_pipeline::bloom::BloomSettings;
use snake::SnakeHead;
use snake::snake_movement;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(TaskPoolPlugin {
            task_pool_options: TaskPoolOptions {
                compute: TaskPoolThreadAssignmentPolicy {
                    min_threads: available_parallelism(),
                    max_threads: std::usize::MAX,
                    percent: 1.0,
                },
                ..default()
            }
        }))

        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(PerfUiPlugin)

        .add_systems(Startup, setup)
        .add_systems(Update, snake_movement)

        .run();
}

fn setup(mut commands: Commands) {
    println!("Welcome to Snake Game in Rust!");

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings::OLD_SCHOOL,
    ));

    #[cfg(debug_assertions)]
    {
        commands.spawn((
            PerfUiRoot {
                display_labels: true,
                ..default()
            },
            PerfUiEntryFPSWorst::default(),
            PerfUiEntryFPS::default(),
        ));
    }

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.5, 0.5, 0.0),
                scale: Vec3::new(24.0, 24.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.4, 1.0, 0.5),
                ..default()
            },
            ..default()
        },
        SnakeHead
    ));
}
