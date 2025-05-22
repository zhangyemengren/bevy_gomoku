use bevy::prelude::*;

mod chess_piece;

// 棋盘组件
#[derive(Component)]
pub struct GoBoard;

// 棋盘标记点组件
#[derive(Component)]
pub struct StarPoint; 

// 导出棋子组件
pub use chess_piece::*; 
