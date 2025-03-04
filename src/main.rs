mod components;
mod systems;
mod resources;
mod entities;
mod plugins;

use bevy::prelude::*;
use systems::{
    setup_background, 
    setup_camera, 
    setup_chess_pieces, 
    add_piece_appearance, 
    update_piece_transforms, 
    spawn_piece_visuals
};
use resources::load_fonts;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 字体需优先加载
        .add_systems(Startup, (load_fonts, setup_camera).chain())
        // 背景需在字体加载后绘制
        .add_systems(Startup, setup_background.after(load_fonts))
        // 棋子需在背景绘制后生成
        .add_systems(Startup, setup_chess_pieces.after(setup_background))
        // 添加棋子外观组件
        .add_systems(Update, add_piece_appearance)
        // 更新棋子变换
        .add_systems(Update, update_piece_transforms)
        // 绘制棋子视觉表现
        .add_systems(Update, spawn_piece_visuals)
        .run();
}

