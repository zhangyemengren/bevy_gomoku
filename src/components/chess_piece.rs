use bevy::prelude::*;
use crate::resources::constants::{PIECE_BLACK_COLOR, PIECE_WHITE_COLOR};

/// Stone color - Black or White
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum StoneColor {
    Black,
    White,
}

impl StoneColor {
    /// Get the display color for this stone
    pub fn color(&self) -> Color {
        match self {
            StoneColor::Black => PIECE_BLACK_COLOR,
            StoneColor::White => PIECE_WHITE_COLOR,
        }
    }
    
    /// Get the opposite color
    pub fn opposite(&self) -> Self {
        match self {
            StoneColor::Black => StoneColor::White,
            StoneColor::White => StoneColor::Black,
        }
    }
}

/// Stone component - marks an entity as a stone with position
#[derive(Component)]
pub struct Stone {
    pub x: i32,
    pub y: i32,
}
