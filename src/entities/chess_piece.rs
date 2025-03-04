use bevy::prelude::*;
use crate::components::{ChessPiece, ChessPieceType, BoardPosition, PieceAppearance};
use crate::resources::constants::*;
use crate::resources::GameFonts;

/// 创建棋子实体
pub fn create_chess_piece(
    commands: &mut Commands,
    piece_type: ChessPieceType,
    position: (i32, i32),
) -> Entity {
    // 创建棋子组件
    let chess_piece = ChessPiece::new(piece_type, position);
    
    // 计算初始位置
    let x = -BOARD_SIZE.x / 2.0 + position.0 as f32 * GRID_SIZE + GRID_SIZE / 2.0;
    let y = BOARD_SIZE.y / 2.0 - position.1 as f32 * GRID_SIZE - GRID_SIZE / 2.0;
    
    // 创建棋子实体
    commands.spawn((
        chess_piece,
        BoardPosition { position },
        Transform::from_xyz(x, y, 2.0), // 初始变换
        Visibility::default(),
    )).id()
}

/// 创建初始棋盘布局
pub fn create_initial_pieces(commands: &mut Commands) {
    // 红方棋子初始位置
    let red_pieces = [
        // 底线棋子
        (ChessPieceType::Rook, (0, 9)),     // 左车
        (ChessPieceType::Horse, (1, 9)),    // 左马
        (ChessPieceType::Elephant, (2, 9)), // 左相
        (ChessPieceType::Advisor, (3, 9)),  // 左士
        (ChessPieceType::General, (4, 9)),  // 帅
        (ChessPieceType::Advisor, (5, 9)),  // 右士
        (ChessPieceType::Elephant, (6, 9)), // 右相
        (ChessPieceType::Horse, (7, 9)),    // 右马
        (ChessPieceType::Rook, (8, 9)),     // 右车
        
        // 炮
        (ChessPieceType::Cannon, (1, 7)),   // 左炮
        (ChessPieceType::Cannon, (7, 7)),   // 右炮
        
        // 兵
        (ChessPieceType::Soldier, (0, 6)),  // 兵1
        (ChessPieceType::Soldier, (2, 6)),  // 兵2
        (ChessPieceType::Soldier, (4, 6)),  // 兵3
        (ChessPieceType::Soldier, (6, 6)),  // 兵4
        (ChessPieceType::Soldier, (8, 6)),  // 兵5
    ];
    
    // 黑方棋子初始位置
    let black_pieces = [
        // 底线棋子
        (ChessPieceType::BRook, (0, 0)),     // 左车
        (ChessPieceType::BHorse, (1, 0)),    // 左马
        (ChessPieceType::BElephant, (2, 0)), // 左象
        (ChessPieceType::BAdvisor, (3, 0)),  // 左士
        (ChessPieceType::BGeneral, (4, 0)),  // 将
        (ChessPieceType::BAdvisor, (5, 0)),  // 右士
        (ChessPieceType::BElephant, (6, 0)), // 右象
        (ChessPieceType::BHorse, (7, 0)),    // 右马
        (ChessPieceType::BRook, (8, 0)),     // 右车
        
        // 炮
        (ChessPieceType::BCannon, (1, 2)),   // 左炮
        (ChessPieceType::BCannon, (7, 2)),   // 右炮
        
        // 卒
        (ChessPieceType::BPawn, (0, 3)),     // 卒1
        (ChessPieceType::BPawn, (2, 3)),     // 卒2
        (ChessPieceType::BPawn, (4, 3)),     // 卒3
        (ChessPieceType::BPawn, (6, 3)),     // 卒4
        (ChessPieceType::BPawn, (8, 3)),     // 卒5
    ];
    
    // 生成红方棋子
    for (piece_type, position) in red_pieces.iter() {
        create_chess_piece(commands, *piece_type, *position);
    }
    
    // 生成黑方棋子
    for (piece_type, position) in black_pieces.iter() {
        create_chess_piece(commands, *piece_type, *position);
    }
}

/// 创建棋子外观
pub fn create_piece_appearance(
    entity: Entity,
    appearance: &PieceAppearance,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    fonts: &Res<GameFonts>,
) {
    // 创建棋子视觉表现
    commands.entity(entity).with_children(|parent| {
        // 棋子背景圆形
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(PIECE_SIZE / 2.0))),
            MeshMaterial2d(materials.add(appearance.color)),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::default(),
        ));
        
        // 棋子文字
        parent.spawn((
            Text2d::new(appearance.text.clone()),
            TextFont {
                font: fonts.noto_sans_sc.clone(),
                font_size: PIECE_TEXT_SIZE,
                ..default()
            },
            TextColor(PIECE_TEXT_COLOR),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(0.0, 0.0, 0.1),
        ));
    });
} 