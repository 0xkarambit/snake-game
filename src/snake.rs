use bevy::prelude::*;

#[derive(Component)]
pub struct SnakeHead;

pub fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut positions: Query<&mut Transform, With<SnakeHead>>
) {
    for mut transform in positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += 2.;
        }
    }
}
