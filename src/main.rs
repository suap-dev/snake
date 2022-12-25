use bevy::{prelude::*, transform};

fn main() {
    App::new()    
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

// spawning new entity build from..
fn spawn_snake(mut commands: Commands) {
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
        .insert(SnakeHead);
}

// query or Transforms (as &mut) that also have SnakeHead Component
fn snake_movement(mut head_positions: Query<&mut Transform, With<SnakeHead>>) {
    for mut transform in head_positions.iter_mut() {
        transform.translation.y += 2.0;
    }
}
