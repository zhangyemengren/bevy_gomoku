mod components;
mod systems;
mod resources;
mod entities;
mod plugins;

use bevy::prelude::*;
use systems::{setup_background, setup_camera};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_background)
        .run();
}

