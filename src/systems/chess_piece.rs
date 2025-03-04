use bevy::prelude::*;
use crate::components::{ChessPiece, ChessPieceType, BoardPosition, PieceAppearance};
use crate::resources::constants::*;
use crate::resources::GameFonts;
use crate::entities;

/// 初始化棋子系统
pub fn setup_chess_pieces(mut commands: Commands) {
    // 创建初始棋盘布局
    entities::create_initial_pieces(&mut commands);
}

/// 为棋子添加外观组件
pub fn add_piece_appearance(
    mut commands: Commands,
    query: Query<(Entity, &ChessPiece), Added<ChessPiece>>,
) {
    for (entity, piece) in query.iter() {
        // 获取棋子文本和颜色
        let text = piece.get_text().to_string();
        let color = match piece.color {
            crate::components::ChessPieceColor::Red => PIECE_RED_COLOR,
            crate::components::ChessPieceColor::Black => PIECE_BLACK_COLOR,
        };
        
        // 添加外观组件
        commands.entity(entity).insert(PieceAppearance {
            text,
            color,
        });
    }
}

/// 根据棋子位置更新变换组件
pub fn update_piece_transforms(
    mut query: Query<(&BoardPosition, &mut Transform), With<ChessPiece>>,
) {
    for (board_pos, mut transform) in query.iter_mut() {
        // 计算棋子在屏幕上的位置
        // 从棋盘左上角(0,0)开始，向右向下增加
        let x = -BOARD_SIZE.x / 2.0 + board_pos.position.0 as f32 * GRID_SIZE + GRID_SIZE / 2.0;
        let y = BOARD_SIZE.y / 2.0 - board_pos.position.1 as f32 * GRID_SIZE - GRID_SIZE / 2.0;
        
        // 更新变换
        transform.translation = Vec3::new(x, y, 2.0);
    }
}

/// 绘制棋子视觉表现
pub fn spawn_piece_visuals(
    mut commands: Commands,
    query: Query<(Entity, &PieceAppearance), Added<PieceAppearance>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    fonts: Res<GameFonts>,
) {
    for (entity, appearance) in query.iter() {
        // 使用实体模块创建棋子视觉表现
        entities::create_piece_appearance(
            entity,
            appearance,
            &mut commands,
            &mut meshes,
            &mut materials,
            &fonts
        );
    }
} 