use bevy::{core::FixedTimestep, prelude::*};

pub mod game;
use game::systems::*;

fn main() {
    let mut game = App::new();

    game.insert_resource(WindowDescriptor {
        width: 800.0,
        height: 800.0,
        title: "Snake".to_string(),
        ..default()
    })
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)));

    game.add_plugins(DefaultPlugins);

    game.add_startup_system(setup_camera)
        .add_startup_system(spawn_snake);

    game.add_system(snake_movement);

    game.add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1.0))
            .with_system(food_spawner),
    );

    game.add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(position_translation)
            .with_system(size_scaling),
    );

    game.run();
}
