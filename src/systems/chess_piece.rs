use bevy::prelude::*;
use crate::components::{
    ChessPiece, 
    ChessPieceType, 
    Side,
    BoardPosition, 
    PieceAppearance,
    Selected,
    PieceBackground,
    PieceText,
    get_side_color_value,
    get_piece_text
};
use crate::resources::constants::*;
use crate::resources::GameFonts;

/// 初始化棋子系统
pub fn setup_chess_pieces(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    fonts: Res<GameFonts>,
) {
    // 创建初始棋盘布局
    create_initial_pieces(&mut commands, &mut meshes, &mut materials, &fonts);
}

/// 创建棋子实体
fn create_chess_piece(
    commands: &mut Commands,
    piece_type: ChessPieceType,
    side: Side,
    position: (i32, i32),
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    fonts: &Res<GameFonts>,
) -> Entity {
    // 计算初始位置
    let x = -BOARD_SIZE.x / 2.0 + position.0 as f32 * GRID_SIZE;
    let y = BOARD_SIZE.y / 2.0 - position.1 as f32 * GRID_SIZE;
    
    // 获取棋子文本和颜色
    let text = get_piece_text(piece_type, side).to_string();
    let display_color = get_side_color_value(side);
    let piece_appearance = PieceAppearance {
        text: text.clone(),
        color: display_color,
    };
    
    // 创建棋子实体
    let entity = commands.spawn((
        ChessPiece,
        piece_type,
        side,
        BoardPosition { position },
        piece_appearance,
        Transform::from_xyz(x, y, 2.0), // 初始变换
        Visibility::default(),
    )).id();
    
    // 直接添加视觉表现
    commands.entity(entity).with_children(|parent| {
        // 棋子背景圆形
        parent.spawn((
            PieceBackground,
            Mesh2d(meshes.add(Circle::new(PIECE_SIZE / 2.0))),
            MeshMaterial2d(materials.add(display_color)),
            Transform::from_xyz(0.0, 0.0, 2.0),
            Visibility::Visible,
        ));
        
        // 棋子文字
        parent.spawn((
            PieceText,
            Text2d::new(text),
            TextFont {
                font: fonts.noto_sans_sc.clone(),
                font_size: PIECE_TEXT_SIZE,
                ..default()
            },
            TextColor(PIECE_TEXT_COLOR),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(0.0, 0.0, 3.0),
            Visibility::Visible,
        ));
    });
    
    entity
}

/// 创建初始棋盘布局
fn create_initial_pieces(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    fonts: &Res<GameFonts>,
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
        (ChessPieceType::Rook, (0, 0)),     // 左车
        (ChessPieceType::Horse, (1, 0)),    // 左马
        (ChessPieceType::Elephant, (2, 0)), // 左象
        (ChessPieceType::Advisor, (3, 0)),  // 左士
        (ChessPieceType::General, (4, 0)),  // 将
        (ChessPieceType::Advisor, (5, 0)),  // 右士
        (ChessPieceType::Elephant, (6, 0)), // 右象
        (ChessPieceType::Horse, (7, 0)),    // 右马
        (ChessPieceType::Rook, (8, 0)),     // 右车
        
        // 炮
        (ChessPieceType::Cannon, (1, 2)),   // 左炮
        (ChessPieceType::Cannon, (7, 2)),   // 右炮
        
        // 卒
        (ChessPieceType::Soldier, (0, 3)),  // 卒1
        (ChessPieceType::Soldier, (2, 3)),  // 卒2
        (ChessPieceType::Soldier, (4, 3)),  // 卒3
        (ChessPieceType::Soldier, (6, 3)),  // 卒4
        (ChessPieceType::Soldier, (8, 3)),  // 卒5
    ];
    
    // 生成红方棋子
    for (piece_type, position) in red_pieces.iter() {
        create_chess_piece(commands, *piece_type, Side::Red, *position, meshes, materials, fonts);
    }
    
    // 生成黑方棋子
    for (piece_type, position) in black_pieces.iter() {
        create_chess_piece(commands, *piece_type, Side::Black, *position, meshes, materials, fonts);
    }
}

/// 根据棋子位置更新变换组件
pub fn update_piece_transforms(
    mut query: Query<(&BoardPosition, &mut Transform), With<ChessPiece>>,
) {
    for (board_pos, mut transform) in query.iter_mut() {
        // 计算棋子在屏幕上的位置
        // 从棋盘左上角(0,0)开始，向右向下增加
        let x = -BOARD_SIZE.x / 2.0 + board_pos.position.0 as f32 * GRID_SIZE;
        let y = BOARD_SIZE.y / 2.0 - board_pos.position.1 as f32 * GRID_SIZE;
        
        // 更新变换
        transform.translation = Vec3::new(x, y, 2.0);
    }
}

/// 处理棋子点击事件
pub fn handle_chess_piece_click(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    pieces: Query<(Entity, &Transform, &BoardPosition), With<ChessPiece>>,
    selected: Query<Entity, With<Selected>>,
) {
    // 当鼠标左键被点击时
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // 获取主窗口和鼠标位置
        let window = windows.single();
        
        if let Some(cursor_position) = window.cursor_position() {
            // 获取相机
            let (camera, camera_transform) = cameras.single();
            
            // 清除之前选中的棋子
            for entity in selected.iter() {
                commands.entity(entity).remove::<Selected>();
            }
            
            // 检查是否点击了棋子
            for (entity, transform, _) in pieces.iter() {
                // 将棋子位置转换为屏幕坐标
                if let Some(ndc) = camera.world_to_ndc(camera_transform, transform.translation) {
                    // 从NDC坐标转换为窗口坐标
                    let screen_x = (ndc.x + 1.0) * 0.5 * window.width();
                    let screen_y = (1.0 - ndc.y) * 0.5 * window.height();
                    let piece_screen_pos = Vec2::new(screen_x, screen_y);
                    
                    // 计算鼠标到棋子的距离
                    let distance = cursor_position.distance(piece_screen_pos);
                    
                    // 如果鼠标点击在棋子范围内
                    if distance < PIECE_SIZE {
                        info!("选中棋子: {:?}", entity);
                        // 添加选中组件
                        commands.entity(entity).insert(Selected::default());
                        break; // 只选中一个棋子
                    }
                }
            }
        }
    }
}

/// 更新闪烁效果 - 仅使用缩放
pub fn update_piece_selection(
    time: Res<Time>,
    mut selected_pieces: Query<(&mut Selected, &mut Transform)>,
) {
    for (mut selected, mut transform) in selected_pieces.iter_mut() {
        // 更新闪烁计时器
        selected.timer.tick(time.delta());
        
        // 检查是否需要切换闪烁状态
        if selected.timer.just_finished() {
            selected.is_highlighted = !selected.is_highlighted;
            
            // 在正常大小和放大状态之间切换
            if selected.is_highlighted {
                transform.scale = Vec3::splat(1.2); // 放大20%
            } else {
                transform.scale = Vec3::ONE; // 恢复正常大小
            }
        }
    }
} 