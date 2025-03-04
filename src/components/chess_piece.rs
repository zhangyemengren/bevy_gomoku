use bevy::prelude::*;

/// 棋子类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChessPieceType {
    // 红方
    Rook,    // 车
    Horse,   // 马
    Elephant, // 相
    Advisor, // 士
    General, // 帅
    Cannon,  // 炮
    Soldier, // 兵
    
    // 黑方
    BRook,    // 车
    BHorse,   // 马
    BElephant, // 象
    BAdvisor, // 士
    BGeneral, // 将
    BCannon,  // 炮
    BPawn,    // 卒
}

/// 棋子颜色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChessPieceColor {
    Red,
    Black,
}

/// 棋子组件 - 包含棋子的基本信息
#[derive(Component, Clone)]
pub struct ChessPiece {
    /// 棋子类型
    pub piece_type: ChessPieceType,
    /// 棋子颜色
    pub color: ChessPieceColor,
    /// 棋子在棋盘上的位置（格子坐标）
    pub position: (i32, i32),
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

impl ChessPiece {
    /// 创建新棋子
    pub fn new(piece_type: ChessPieceType, position: (i32, i32)) -> Self {
        // 根据棋子类型确定颜色
        let color = match piece_type {
            ChessPieceType::Rook | 
            ChessPieceType::Horse | 
            ChessPieceType::Elephant | 
            ChessPieceType::Advisor | 
            ChessPieceType::General | 
            ChessPieceType::Cannon | 
            ChessPieceType::Soldier => ChessPieceColor::Red,
            
            _ => ChessPieceColor::Black,
        };
        
        Self {
            piece_type,
            color,
            position,
        }
    }
    
    /// 获取棋子的显示文本
    pub fn get_text(&self) -> &str {
        match self.piece_type {
            // 红方
            ChessPieceType::Rook => "车",
            ChessPieceType::Horse => "马",
            ChessPieceType::Elephant => "相",
            ChessPieceType::Advisor => "士",
            ChessPieceType::General => "帅",
            ChessPieceType::Cannon => "炮",
            ChessPieceType::Soldier => "兵",
            
            // 黑方
            ChessPieceType::BRook => "车",
            ChessPieceType::BHorse => "马",
            ChessPieceType::BElephant => "象",
            ChessPieceType::BAdvisor => "士",
            ChessPieceType::BGeneral => "将",
            ChessPieceType::BCannon => "炮",
            ChessPieceType::BPawn => "卒",
        }
    }
    
    /// 获取棋子的颜色
    pub fn get_color(&self) -> Color {
        match self.color {
            ChessPieceColor::Red => Color::rgb(0.8, 0.0, 0.0),
            ChessPieceColor::Black => Color::rgb(0.0, 0.0, 0.0),
        }
    }
} 