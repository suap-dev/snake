pub mod components;
pub mod systems;
pub mod entities {}

pub const ARENA_WIDTH: u32 = 10;
pub const ARENA_HEIGHT: u32 = 10;

use bevy::prelude::Color;
pub const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
