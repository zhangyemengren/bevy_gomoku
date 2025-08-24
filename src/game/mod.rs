use bevy::prelude::*;
use crate::components::StoneColor;

/// Game state resource - tracks current player and game status
#[derive(Resource)]
pub struct GameState {
    /// Current player
    pub current_player: StoneColor,
    /// Is game over
    pub game_over: bool,
    /// Winner if game is over
    pub winner: Option<StoneColor>,
    /// Board state - stores stone color at each position, None means empty
    pub board: [[Option<StoneColor>; 15]; 15],
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_player: StoneColor::Black, // Black goes first
            game_over: false,
            winner: None,
            board: [[None; 15]; 15],
        }
    }
}

/// Check if there's a win condition (5 in a row)
pub fn check_win(board: &[[Option<StoneColor>; 15]; 15], x: i32, y: i32, color: StoneColor) -> bool {
    let directions = [
        (1, 0),   // Horizontal
        (0, 1),   // Vertical
        (1, 1),   // Diagonal
        (1, -1),  // Anti-diagonal
    ];

    for (dx, dy) in directions.iter() {
        let mut count = 1; // Current position already has one stone

        // Check forward direction
        let mut nx = x + dx;
        let mut ny = y + dy;
        while nx >= 0 && nx < 15 && ny >= 0 && ny < 15 {
            if board[ny as usize][nx as usize] == Some(color) {
                count += 1;
                nx += dx;
                ny += dy;
            } else {
                break;
            }
        }

        // Check backward direction
        nx = x - dx;
        ny = y - dy;
        while nx >= 0 && nx < 15 && ny >= 0 && ny < 15 {
            if board[ny as usize][nx as usize] == Some(color) {
                count += 1;
                nx -= dx;
                ny -= dy;
            } else {
                break;
            }
        }

        // Win if 5 or more stones in a row
        if count >= 5 {
            return true;
        }
    }

    false
}