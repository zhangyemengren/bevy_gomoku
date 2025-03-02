use bevy::prelude::*;

// 棋盘常量
pub const BOARD_WIDTH_COUNT: f32 = 8.0;
pub const BOARD_HEIGHT_COUNT: f32 = 9.0;
pub const GRID_SIZE: f32 = 60.0; // 格子大小
pub const BOARD_WIDTH: f32 = BOARD_WIDTH_COUNT * GRID_SIZE;
pub const BOARD_HEIGHT: f32 = BOARD_HEIGHT_COUNT * GRID_SIZE;
pub const BG_WIDTH: f32 = BOARD_WIDTH + GRID_SIZE;
pub const BG_HEIGHT: f32 = BOARD_HEIGHT + GRID_SIZE;
pub const BOARD_SIZE: Vec2 = Vec2::new(BOARD_WIDTH, BOARD_HEIGHT); // 棋盘大小
pub const LINE_WIDTH: f32 = 2.0; // 线宽
pub const LINE_COLOR: Color = Color::BLACK;
pub const BACKGROUND_COLOR: Color = Color::srgb(0.93, 0.87, 0.73); // 米黄色
pub const POINT_SIZE: f32 = 10.0; // 棋子放置点大小
pub const POINT_COLOR: Color = Color::BLACK; // 放置点颜色
pub const TEXT_SIZE: f32 = 40.0;

// 棋盘位置配置
pub const PALACE_POSITIONS: [[(i32, i32); 2]; 4] = [
    // 上方九宫
    [(3, 0), (5, 2)],
    [(5, 0), (3, 2)],
    // 下方九宫
    [(3, 7), (5, 9)],
    [(5, 7), (3, 9)],
];

pub const POINT_POSITIONS: [(i32, i32); 14] = [
    // 炮位
    (1, 2), (7, 2), (1, 7), (7, 7),
    // 兵卒位
    (0, 3), (2, 3), (4, 3), (6, 3), (8, 3),
    (0, 6), (2, 6), (4, 6), (6, 6), (8, 6),
]; 