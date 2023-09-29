use godot::prelude::*;

struct AlienAttackExtension;

#[gdextension]
unsafe impl ExtensionLibrary for AlienAttackExtension {}

mod enemy;
mod enemy_spawner;
mod game;
mod game_over_screen;
mod hud;
mod player;
mod rocket;
