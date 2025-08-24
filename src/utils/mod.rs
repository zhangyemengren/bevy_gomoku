use bevy::prelude::*;
use crate::resources::constants::*;

/// Convert board grid coordinates to world position
pub fn grid_to_world(x: i32, y: i32) -> Vec2 {
    let world_x = -BOARD_SIZE.x / 2.0 + x as f32 * GRID_SIZE;
    let world_y = BOARD_SIZE.y / 2.0 - y as f32 * GRID_SIZE;
    Vec2::new(world_x, world_y)
}

/// Convert world position to board grid coordinates
pub fn world_to_grid(world_pos: Vec2) -> Option<(i32, i32)> {
    let grid_x = ((world_pos.x + BOARD_SIZE.x / 2.0) / GRID_SIZE).round() as i32;
    let grid_y = ((BOARD_SIZE.y / 2.0 - world_pos.y) / GRID_SIZE).round() as i32;
    
    // Check if within board bounds
    if grid_x >= 0 && grid_x < 15 && grid_y >= 0 && grid_y < 15 {
        Some((grid_x, grid_y))
    } else {
        None
    }
}