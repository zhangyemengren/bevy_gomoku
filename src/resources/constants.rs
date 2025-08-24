use bevy::prelude::*;

// Board dimensions - 15x15 Gomoku board
pub const BOARD_SIZE: Vec2 = Vec2::new(560.0, 560.0); // 14 * 40
pub const GRID_SIZE: f32 = 40.0;
pub const BOARD_WIDTH: f32 = 560.0;  // 14 cells * 40 pixels
pub const BOARD_HEIGHT: f32 = 560.0; // 14 cells * 40 pixels
pub const BG_WIDTH: f32 = 600.0;     // Board + margin
pub const BG_HEIGHT: f32 = 600.0;    // Board + margin

// Visual properties
pub const LINE_WIDTH: f32 = 1.5;
pub const LINE_COLOR: Color = Color::BLACK;
pub const BACKGROUND_COLOR: Color = Color::srgb(0.93, 0.87, 0.73); // Beige
pub const POINT_SIZE: f32 = 6.0;
pub const POINT_COLOR: Color = Color::BLACK;
pub const TEXT_SIZE: f32 = 30.0;

// Stone properties
pub const PIECE_SIZE: f32 = 36.0;
pub const PIECE_BLACK_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
pub const PIECE_WHITE_COLOR: Color = Color::srgb(0.95, 0.95, 0.95);

// Star positions for Gomoku (5 points)
pub const STAR_POSITIONS: [(i32, i32); 5] = [
    (7, 7),    // Center
    (3, 3),    // Top-left
    (11, 3),   // Top-right
    (3, 11),   // Bottom-left
    (11, 11),  // Bottom-right
];