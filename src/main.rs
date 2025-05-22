mod components;
mod systems;
mod resources;
mod plugins;

use bevy::prelude::*;
use systems::{
    setup_background, 
    setup_camera, 
    setup_chess_pieces, 
    update_piece_transforms,
    handle_chess_piece_click,
    update_piece_selection
};
use resources::load_fonts;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 字体需优先加载
        .add_systems(Startup, (load_fonts, setup_camera).chain())
        // 背景需在字体加载后绘制
        .add_systems(Startup, setup_background.after(load_fonts))
        // 初始化游戏状态
        .add_systems(Startup, setup_chess_pieces.after(setup_background))
        // 更新棋子变换
        .add_systems(Update, update_piece_transforms)
        // 处理棋盘点击和放置棋子
        .add_systems(Update, handle_chess_piece_click)
        // 更新选中棋子闪烁效果
        .add_systems(Update, update_piece_selection)
        .run();
}
