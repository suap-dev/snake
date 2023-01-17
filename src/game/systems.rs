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
    mut head_positions: Query<&mut Position, With<SnakeHead>>,
) {
    for mut position in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            position.x -= 2;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            position.x += 2;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            position.y -= 2;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            position.y += 2;
        }
    }
}

// TODO: make this readable
pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&components::Size, &mut Transform)>) {
    // The sizing logic goes like so:
    // if something has a width of 1 in a grid of 40,
    // and the window is 400px across,
    // then it should have a width of 10.
    // -- https://mbuffett.com/posts/bevy-snake-tutorial/
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width * (window.width() / ARENA_WIDTH as f32),
            sprite_size.height * (window.height() / ARENA_HEIGHT as f32),
            1.0,
        );
    }
}

// TODO: make this readable
pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    // The position translation:
    // if an item’s x coordinate is at 5 in our system,
    // the width in our system is 10, and the window width is 200,
    // then the coordinate should be 5 / 10 * 200 - 200 / 2.
    // We subtract half the window width because our coordinate system
    // starts at the bottom left, and Translation starts from the center.
    // We then add half the size of a single tile, because we want our sprites
    // bottom left corner to be at the bottom left of a tile not the center.
    // -- https://mbuffett.com/posts/bevy-snake-tutorial/
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

pub fn food_spawner(mut commands: Commands) {
    use rand::prelude::random;
    
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
            })
        .insert(components::Size::square(0.8));
}
