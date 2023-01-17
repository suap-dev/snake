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
    let window = windows.get_primary().unwrap();

    let width_ratio = window.width() / ARENA_WIDTH as f32;
    let height_ratio = window.height() / ARENA_HEIGHT as f32;

    for (sprite_size, mut transform) in q.iter_mut() {
        let scaled_width = sprite_size.width * width_ratio;
        let scaled_height = sprite_size.height * height_ratio;

        transform.scale = Vec3::new(scaled_width, scaled_height, 1.0);
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    let window = windows.get_primary().unwrap();

    let window_width = window.width() as f32;
    let window_height = window.height() as f32;

    let game_width = ARENA_WIDTH as f32;
    let game_height = ARENA_HEIGHT as f32;

    let tile_size_x = window_width / game_width;
    let tile_size_y = window_height / game_height;

    let half_window_width = window_width / 2.;
    let half_window_height = window_height / 2.;

    let half_tile_size_x = tile_size_x / 2.;
    let half_tile_size_y = tile_size_y / 2.;

    for (pos, mut transform) in q.iter_mut() {
        let x_ratio = pos.x as f32 / game_width;
        let y_ratio = pos.y as f32 / game_height;

        let x = (x_ratio * window_width) + half_tile_size_x - half_window_width;
        let y = (y_ratio * window_height) + half_tile_size_y - half_window_height;

        transform.translation = Vec3::new(x, y, 0.0);
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
