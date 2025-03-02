use bevy::prelude::*;

mod chess_piece;

// 棋盘组件
#[derive(Component)]
pub struct ChessBoard;

// 棋子放置点组件
#[derive(Component)]
pub struct PlacementPoint; 

// 导出棋子组件
pub use chess_piece::*; 