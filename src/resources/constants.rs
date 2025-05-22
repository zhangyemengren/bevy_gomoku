use bevy::prelude::*;

// 棋盘常量 - 五子棋使用15x15的棋盘
pub const BOARD_WIDTH_COUNT: f32 = 14.0;  // 15条线需要14个格子宽度
pub const BOARD_HEIGHT_COUNT: f32 = 14.0; // 15条线需要14个格子高度
pub const GRID_SIZE: f32 = 40.0; // 格子大小（减小以适应更大的棋盘）
pub const BOARD_WIDTH: f32 = BOARD_WIDTH_COUNT * GRID_SIZE;
pub const BOARD_HEIGHT: f32 = BOARD_HEIGHT_COUNT * GRID_SIZE;
pub const BG_WIDTH: f32 = BOARD_WIDTH + GRID_SIZE;
pub const BG_HEIGHT: f32 = BOARD_HEIGHT + GRID_SIZE;
pub const BOARD_SIZE: Vec2 = Vec2::new(BOARD_WIDTH, BOARD_HEIGHT); // 棋盘大小
pub const LINE_WIDTH: f32 = 1.5; // 线宽（稍微减小）
pub const LINE_COLOR: Color = Color::BLACK;
pub const BACKGROUND_COLOR: Color = Color::srgb(0.93, 0.87, 0.73); // 米黄色
pub const POINT_SIZE: f32 = 6.0; // 棋盘标记点大小（减小）
pub const POINT_COLOR: Color = Color::BLACK; // 标记点颜色
pub const TEXT_SIZE: f32 = 30.0; // 文字大小（减小）

// 棋子常量
pub const PIECE_SIZE: f32 = 36.0; // 棋子大小（减小以适应更小的格子）
pub const PIECE_BLACK_COLOR: Color = Color::srgb(0.1, 0.1, 0.1); // 黑棋颜色
pub const PIECE_WHITE_COLOR: Color = Color::srgb(0.95, 0.95, 0.95); // 白棋颜色（替代红色）

// 五子棋标记点位置（五个星位）
pub const STAR_POSITIONS: [(i32, i32); 5] = [
    // 中心点
    (7, 7),
    // 四个角星
    (3, 3),
    (11, 3),
    (3, 11),
    (11, 11),
];
