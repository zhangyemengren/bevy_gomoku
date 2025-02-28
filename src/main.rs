mod components;
mod systems;
mod resources;
mod entities;
mod plugins;

use bevy::prelude::*;
use systems::{setup_background, setup_camera};
use resources::load_fonts;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 字体需优先加载
        .add_systems(Startup, (load_fonts, setup_camera).chain())
        // 背景需在字体加载后绘制
        .add_systems(Startup, setup_background.after(load_fonts))
        .run();
}

