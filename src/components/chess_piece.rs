use bevy::prelude::*;

use crate::resources::constants::{PIECE_BLACK_COLOR, PIECE_RED_COLOR};

/// 棋子类型 - 不再区分红黑方，只表示棋子种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum ChessPieceType {
    Rook,     // 车
    Horse,    // 马
    Elephant, // 相/象
    Advisor,  // 士/仕
    General,  // 帅/将
    Cannon,   // 炮
    Soldier,  // 兵/卒
}

/// 棋子阵营 - 红方或黑方
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Side {
    Red,   // 红方
    Black, // 黑方
}

/// 棋子位置组件 - 用于表示棋子在棋盘上的位置
#[derive(Component, Clone, Debug)]
pub struct BoardPosition {
    /// 棋子在棋盘上的位置（格子坐标）
    pub position: (i32, i32),
}

/// 棋子外观组件 - 用于表示棋子的视觉外观
#[derive(Component, Clone)]
pub struct PieceAppearance {
    /// 棋子的显示文本
    pub text: String,
    /// 棋子的颜色
    pub color: Color,
}

/// 棋子标记组件 - 用于标识一个实体是棋子
#[derive(Component)]
pub struct ChessPiece;

/// 棋子背景组件标记 - 用于标识棋子的背景圆
#[derive(Component)]
pub struct PieceBackground;

/// 棋子文本组件标记 - 用于标识棋子的文本
#[derive(Component)]
pub struct PieceText;

/// 选中状态组件 - 用于标识棋子是否被选中
#[derive(Component)]
pub struct Selected {
    /// 闪烁计时器
    pub timer: Timer,
    /// 是否在闪烁状态
    pub is_highlighted: bool,
}

impl Default for Selected {
    fn default() -> Self {
        Self {
            // 创建一个重复计时器，每0.3秒触发一次
            timer: Timer::from_seconds(0.3, TimerMode::Repeating),
            is_highlighted: false,
        }
    }
}

/// 获取棋子阵营对应的颜色值
pub fn get_side_color_value(side: Side) -> Color {
    match side {
        Side::Red => PIECE_RED_COLOR,
        Side::Black => PIECE_BLACK_COLOR,
    }
}

/// 获取棋子类型和阵营对应的显示文本
pub fn get_piece_text(piece_type: ChessPieceType, side: Side) -> &'static str {
    match (piece_type, side) {
        // 红方
        (ChessPieceType::Rook, Side::Red) => "车",
        (ChessPieceType::Horse, Side::Red) => "马",
        (ChessPieceType::Elephant, Side::Red) => "相",
        (ChessPieceType::Advisor, Side::Red) => "仕",
        (ChessPieceType::General, Side::Red) => "帅",
        (ChessPieceType::Cannon, Side::Red) => "炮",
        (ChessPieceType::Soldier, Side::Red) => "兵",
        
        // 黑方
        (ChessPieceType::Rook, Side::Black) => "车",
        (ChessPieceType::Horse, Side::Black) => "马",
        (ChessPieceType::Elephant, Side::Black) => "象",
        (ChessPieceType::Advisor, Side::Black) => "士",
        (ChessPieceType::General, Side::Black) => "将",
        (ChessPieceType::Cannon, Side::Black) => "炮",
        (ChessPieceType::Soldier, Side::Black) => "卒",
    }
} 