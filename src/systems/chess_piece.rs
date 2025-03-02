use bevy::prelude::*;
use crate::components::ChessPiece;
use crate::components::ChessPieceType;
use crate::resources::constants::*;
use crate::resources::GameFonts;

/// 初始化棋子系统
pub fn setup_chess_pieces(
    mut commands: Commands,
    fonts: Res<GameFonts>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 创建初始棋盘布局
    spawn_initial_pieces(&mut commands, &fonts, &mut meshes, &mut materials);
}

/// 生成初始棋子布局
fn spawn_initial_pieces(
    commands: &mut Commands, 
    fonts: &Res<GameFonts>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
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
        spawn_chess_piece(commands, *piece_type, *position, fonts, meshes, materials);
    }
    
    // 生成黑方棋子
    for (piece_type, position) in black_pieces.iter() {
        spawn_chess_piece(commands, *piece_type, *position, fonts, meshes, materials);
    }
}

/// 生成单个棋子
fn spawn_chess_piece(
    commands: &mut Commands,
    piece_type: ChessPieceType,
    position: (i32, i32),
    fonts: &Res<GameFonts>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // 创建棋子组件
    let chess_piece = ChessPiece::new(piece_type, position);
    
    // 计算棋子在屏幕上的位置
    // 从棋盘左上角(0,0)开始，向右向下增加
    let x = -BOARD_SIZE.x / 2.0 + position.0 as f32 * GRID_SIZE;
    let y = BOARD_SIZE.y / 2.0 - position.1 as f32 * GRID_SIZE ;
    
    // 获取棋子颜色和文字
    let (piece_color, piece_text) = match piece_type {
        // 红方棋子
        ChessPieceType::General => (PIECE_RED_COLOR, "帅"),
        ChessPieceType::Advisor => (PIECE_RED_COLOR, "仕"),
        ChessPieceType::Elephant => (PIECE_RED_COLOR, "相"),
        ChessPieceType::Horse => (PIECE_RED_COLOR, "马"),
        ChessPieceType::Rook => (PIECE_RED_COLOR, "车"),
        ChessPieceType::Cannon => (PIECE_RED_COLOR, "炮"),
        ChessPieceType::Soldier => (PIECE_RED_COLOR, "兵"),
        
        // 黑方棋子
        ChessPieceType::BGeneral => (PIECE_BLACK_COLOR, "将"),
        ChessPieceType::BAdvisor => (PIECE_BLACK_COLOR, "士"),
        ChessPieceType::BElephant => (PIECE_BLACK_COLOR, "象"),
        ChessPieceType::BHorse => (PIECE_BLACK_COLOR, "马"),
        ChessPieceType::BRook => (PIECE_BLACK_COLOR, "车"),
        ChessPieceType::BCannon => (PIECE_BLACK_COLOR, "炮"),
        ChessPieceType::BPawn => (PIECE_BLACK_COLOR, "卒"),
    };
    
    // 创建棋子实体
    commands.spawn((
        chess_piece,
        Transform::from_xyz(x, y, 2.0), // z=2.0 确保棋子在棋盘上方
        Visibility::default(),
    )).with_children(|parent| {
        // 棋子背景圆形
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(PIECE_SIZE / 2.0))),
            MeshMaterial2d(materials.add(piece_color)),
            Transform::from_xyz(0.0, 0.0, 2.0),
            Visibility::default(),
        ));
        
        // 棋子文字
        parent.spawn((
            Text2d::new(piece_text),
            TextFont {
                font: fonts.noto_sans_sc.clone(),
                font_size: PIECE_TEXT_SIZE,
                ..default()
            },
            TextColor(PIECE_TEXT_COLOR),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(0.0, 0.0, 3.0),
        ));
    });
} 