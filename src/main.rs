mod components;
mod systems;
mod resources;
mod plugins;
mod utils;
mod game;

use bevy::prelude::*;
use systems::{
    setup_background, 
    setup_camera, 
    setup_game, 
    handle_mouse_click
};
use resources::load_fonts;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Load fonts first
        .add_systems(Startup, (load_fonts, setup_camera).chain())
        // Setup board after fonts
        .add_systems(Startup, setup_background.after(load_fonts))
        // Initialize game state
        .add_systems(Startup, setup_game.after(setup_background))
        // Handle player moves
        .add_systems(Update, handle_mouse_click)
        .run();
}
