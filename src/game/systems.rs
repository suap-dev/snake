use bevy::prelude::*;

use super::components::*;
use super::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// spawning new entity. it's build from..
pub fn spawn_snake(mut commands: Commands) {
    commands
        // ..quite default SpriteBundle, and..
        .spawn_bundle(SpriteBundle {
            // default Sprite with only changed color
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            // and default Transform with just adjusted scale
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        // .. SnakeHead Component
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(components::Size::square(0.8));
}

// query or Transforms (as &mut) that also have SnakeHead Component
pub fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.0;
        }
    }
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&components::Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        // The sizing logic goes like so:
        // if something has a width of 1 in a grid of 40,
        // and the window is 400px across,
        // then it should have a width of 10.
        // -- https://mbuffett.com/posts/bevy-snake-tutorial/
        transform.scale = Vec3::new(
            sprite_size.width * (window.width() / ARENA_WIDTH as f32),
            sprite_size.height * (window.height() / ARENA_HEIGHT as f32),
            1.0,
        );
    }
}