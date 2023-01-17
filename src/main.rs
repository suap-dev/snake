use bevy::prelude::*;

pub mod game;
use game::systems::*;

fn main() {
    let mut game = App::new();

    game.add_plugins(DefaultPlugins);

    game.add_startup_system(setup_camera)
        .add_startup_system(spawn_snake);

    game.add_system(snake_movement);

    game.run();
}
