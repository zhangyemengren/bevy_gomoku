use bevy::prelude::*;

use crate::resources::constants::{PIECE_BLACK_COLOR, PIECE_WHITE_COLOR};

/// 棋子颜色 - 黑色或白色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum StoneColor {
    Black, // 黑棋
    White, // 白棋
}

/// 棋子位置组件 - 用于表示棋子在棋盘上的位置
#[derive(Component, Clone, Debug)]
pub struct BoardPosition {
    /// 棋子在棋盘上的位置（交叉点坐标）
    pub position: (i32, i32),
}

/// 棋子标记组件 - 用于标识一个实体是棋子
#[derive(Component)]
pub struct Stone;

/// 棋子背景组件标记 - 用于标识棋子的背景圆
#[derive(Component)]
pub struct StoneBackground;

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
            // 创建一个重复计时器，每0.8秒触发一次，使闪烁更加缓慢
            timer: Timer::from_seconds(0.8, TimerMode::Repeating),
            is_highlighted: false,
        }
    }
}

/// 获取棋子颜色对应的颜色值
pub fn get_stone_color_value(stone_color: StoneColor) -> Color {
    match stone_color {
        StoneColor::Black => PIECE_BLACK_COLOR,
        StoneColor::White => PIECE_WHITE_COLOR,
    }
}
