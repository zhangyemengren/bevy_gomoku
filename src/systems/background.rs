use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use crate::components::*;
use crate::resources::constants::*;
use crate::resources::GameFonts;

pub fn setup_background(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    fonts: Res<GameFonts>) {

    // 绘制棋盘背景
    commands.spawn((
       ChessBoard,
       Mesh2d(meshes.add(Rectangle::new(BG_WIDTH, BG_HEIGHT))),
       MeshMaterial2d(materials.add(BACKGROUND_COLOR)),
       Transform::default(),
    ));
    
    // 外层边框 - 与内层边框线宽相同，但距离为4倍线宽
    let gap = LINE_WIDTH * 4.0; // 内外层边框之间的间距
    let outer_width = BOARD_WIDTH + LINE_WIDTH * 2.0 + gap * 2.0; // 外层边框宽度
    let outer_height = BOARD_HEIGHT + LINE_WIDTH * 2.0 + gap * 2.0; // 外层边框高度
    
    commands.spawn((
        ChessBoard,
        Mesh2d(meshes.add(Rectangle::new(outer_width, outer_height))),
        MeshMaterial2d(materials.add(Color::NONE)),
        Transform::default(),
    )).with_children(|parent| {
        // 上边框
        parent.spawn((
            ChessBoard,
            Mesh2d(meshes.add(Rectangle::new(outer_width, LINE_WIDTH))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(0.0, outer_height / 2.0 - LINE_WIDTH / 2.0, 1.0),
        ));
        
        // 下边框
        parent.spawn((
            ChessBoard,
            Mesh2d(meshes.add(Rectangle::new(outer_width, LINE_WIDTH))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(0.0, -outer_height / 2.0 + LINE_WIDTH / 2.0, 1.0),
        ));
        
        // 左边框
        parent.spawn((
            ChessBoard,
            Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, outer_height))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(-outer_width / 2.0 + LINE_WIDTH / 2.0, 0.0, 1.0),
        ));
        
        // 右边框
        parent.spawn((
            ChessBoard,
            Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, outer_height))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(outer_width / 2.0 - LINE_WIDTH / 2.0, 0.0, 1.0),
        ));
    });
    
    // 绘制横线
    // 上半部分横线（0-4）
    for i in 0..5 {
        // 计算横线位置，考虑线宽
        let y = BOARD_SIZE.y / 2.0 - i as f32 * (BOARD_SIZE.y / 9.0);
        // 确保线条不会超出边界，线宽居中
        commands.spawn((
            ChessBoard,
            Mesh2d(meshes.add(Rectangle::new(BOARD_WIDTH, LINE_WIDTH))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(0.0, y, 1.0),
        ));
    }
    
    // 下半部分横线（5-9）
    for i in 5..10 {
        // 计算横线位置，考虑线宽
        let y = BOARD_SIZE.y / 2.0 - i as f32 * (BOARD_SIZE.y / 9.0);
        // 确保线条不会超出边界，线宽居中
        commands.spawn((
            ChessBoard,
            Mesh2d(meshes.add(Rectangle::new(BOARD_WIDTH, LINE_WIDTH))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(0.0, y, 1.0),
        ));
    }
    
    // 绘制竖线
    for i in 0..9 {
        let x = -BOARD_SIZE.x / 2.0 + i as f32 * (BOARD_SIZE.x / 8.0);
        
        // 计算上半部分竖线的长度和位置，考虑线宽
        let upper_line_length = BOARD_HEIGHT / 2.0 - GRID_SIZE / 2.0;
        // 位置需要考虑线宽，确保与楚河汉界区域精确对齐
        let upper_line_y_pos = GRID_SIZE / 2.0 + upper_line_length / 2.0;
        
        // 上半部分竖线
        commands.spawn((
            ChessBoard,
            Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, upper_line_length))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(x, upper_line_y_pos, 1.0),
        ));
        
        // 计算下半部分竖线的长度和位置，考虑线宽
        let lower_line_length = BOARD_HEIGHT / 2.0 - GRID_SIZE / 2.0;
        // 位置需要考虑线宽，确保与楚河汉界区域精确对齐
        let lower_line_y_pos = -GRID_SIZE / 2.0 - lower_line_length / 2.0;
        
        // 下半部分竖线
        commands.spawn((
            ChessBoard,
            Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, lower_line_length))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(x, lower_line_y_pos, 1.0),
        ));
        
        // 只在两边添加连接线
        if i == 0 || i == 8 {
            commands.spawn((
                ChessBoard,
                Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, GRID_SIZE))),
                MeshMaterial2d(materials.add(LINE_COLOR)),
                Transform::from_xyz(x, 0.0, 1.0),
            ));
        }
    }

    // 绘制米字格（九宫）
    for positions in PALACE_POSITIONS.iter() {
        let (start_x, start_y) = positions[0];
        let (end_x, end_y) = positions[1];
        
        let start_pos = Vec2::new(
            -BOARD_SIZE.x / 2.0 + start_x as f32 * (BOARD_SIZE.x / 8.0),
            BOARD_SIZE.y / 2.0 - start_y as f32 * (BOARD_SIZE.y / 9.0),
        );
        let end_pos = Vec2::new(
            -BOARD_SIZE.x / 2.0 + end_x as f32 * (BOARD_SIZE.x / 8.0),
            BOARD_SIZE.y / 2.0 - end_y as f32 * (BOARD_SIZE.y / 9.0),
        );
        
        // 使用LineList绘制斜线
        let vertices = vec![[start_pos.x, start_pos.y, 0.0], [end_pos.x, end_pos.y, 0.0]];
        let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        
        commands.spawn((
            ChessBoard,
            Mesh2d(meshes.add(mesh)),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(0.0, 0.0, 1.0),
        ));
    }

    // 绘制棋子放置点（四个角落的直角线）
    for (x, y) in POINT_POSITIONS.iter() {
        let pos_x = -BOARD_SIZE.x / 2.0 + *x as f32 * (BOARD_SIZE.x / 8.0);
        let pos_y = BOARD_SIZE.y / 2.0 - *y as f32 * (BOARD_SIZE.y / 9.0);
        
        // 在四个角落创建直角线
        let corner_offset = GRID_SIZE / 6.0; // 减小距离，使直角线更靠近中心点
        let line_length = POINT_SIZE * 0.7; // 增加线的长度
        
        // 四个角落的位置和直角线方向
        let corners = [
            // 左上角 (横线向左，竖线向上)
            (Vec2::new(-1.0, -1.0), Vec2::new(-1.0, 0.0), Vec2::new(0.0, -1.0)),
            // 右上角 (横线向右，竖线向上)
            (Vec2::new(1.0, -1.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, -1.0)),
            // 左下角 (横线向左，竖线向下)
            (Vec2::new(-1.0, 1.0), Vec2::new(-1.0, 0.0), Vec2::new(0.0, 1.0)),
            // 右下角 (横线向右，竖线向下)
            (Vec2::new(1.0, 1.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)),
        ];
        
        // 检查是否在棋盘边缘，如果是则跳过某些方向的绘制
        let is_left_edge = *x == 0;
        let is_right_edge = *x == 8;
        let is_top_edge = *y == 0;
        let is_bottom_edge = *y == 9;
        
        for (i, (corner_dir, h_dir, v_dir)) in corners.iter().enumerate() {
            // 跳过棋盘边缘外的绘制
            if (is_left_edge && (i == 0 || i == 2)) || // 左边缘跳过左上和左下角
               (is_right_edge && (i == 1 || i == 3)) || // 右边缘跳过右上和右下角
               (is_top_edge && (i == 0 || i == 1)) || // 上边缘跳过左上和右上角
               (is_bottom_edge && (i == 2 || i == 3)) { // 下边缘跳过左下和右下角
                continue;
            }
            
            let corner_x = pos_x + corner_dir.x * corner_offset;
            let corner_y = pos_y + corner_dir.y * corner_offset;
            
            // 判断是否需要绘制横线（与边框不重合）
            let skip_horizontal = 
                (is_left_edge && i == 0) || // 左上角的左边线
                (is_left_edge && i == 2) || // 左下角的左边线
                (is_right_edge && i == 1) || // 右上角的右边线
                (is_right_edge && i == 3);   // 右下角的右边线
                
            if !skip_horizontal {
                // 横线
                commands.spawn((
                    PlacementPoint,
                    ChessBoard,
                    Mesh2d(meshes.add(Rectangle::new(line_length, LINE_WIDTH))),
                    MeshMaterial2d(materials.add(POINT_COLOR)),
                    Transform::from_xyz(
                        corner_x + h_dir.x * line_length / 2.0, 
                        corner_y + h_dir.y * line_length / 2.0, 
                        2.0
                    ),
                ));
            }
            
            // 判断是否需要绘制竖线（与边框不重合）
            let skip_vertical = 
                (is_top_edge && i == 0) || // 左上角的上边线
                (is_top_edge && i == 1) || // 右上角的上边线
                (is_bottom_edge && i == 2) || // 左下角的下边线
                (is_bottom_edge && i == 3);   // 右下角的下边线
                
            if !skip_vertical {
                // 竖线
                commands.spawn((
                    PlacementPoint,
                    ChessBoard,
                    Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, line_length))),
                    MeshMaterial2d(materials.add(POINT_COLOR)),
                    Transform::from_xyz(
                        corner_x + v_dir.x * line_length / 2.0, 
                        corner_y + v_dir.y * line_length / 2.0, 
                        2.0
                    ),
                ));
            }
        }
    }

    // 添加楚河汉界文字（每个字单独渲染并旋转）
    // 楚河 - 分别渲染每个字，顺时针旋转90度
    let chu_pos = Vec2::new(-BOARD_SIZE.x/6.0 - TEXT_SIZE/2.0, 0.0);
    let he_pos = Vec2::new(-BOARD_SIZE.x/6.0 + TEXT_SIZE/2.0, 0.0);
    
    // 楚
    commands.spawn((
        ChessBoard,
        Text2d::new("楚"),
        TextFont {
            font: fonts.noto_sans_sc.clone(),
            font_size: TEXT_SIZE,
            ..default()
        },
        TextColor(LINE_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(chu_pos.x, chu_pos.y, 1.0)
            .with_rotation(Quat::from_rotation_z(90.0 * std::f32::consts::PI / 180.0)),
    ));
    
    // 河
    commands.spawn((
        ChessBoard,
        Text2d::new("河"),
        TextFont {
            font: fonts.noto_sans_sc.clone(),
            font_size: TEXT_SIZE,
            ..default()
        },
        TextColor(LINE_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(he_pos.x, he_pos.y, 1.0)
            .with_rotation(Quat::from_rotation_z(90.0 * std::f32::consts::PI / 180.0)),
    ));
    
    // 汉界 - 分别渲染每个字，逆时针旋转90度
    let han_pos = Vec2::new(BOARD_SIZE.x/6.0 - TEXT_SIZE/2.0, 0.0);
    let jie_pos = Vec2::new(BOARD_SIZE.x/6.0 + TEXT_SIZE/2.0, 0.0);
    
    // 汉
    commands.spawn((
        ChessBoard,
        Text2d::new("汉"),
        TextFont {
            font: fonts.noto_sans_sc.clone(),
            font_size: TEXT_SIZE,
            ..default()
        },
        TextColor(LINE_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(han_pos.x, han_pos.y, 1.0)
            .with_rotation(Quat::from_rotation_z(-90.0 * std::f32::consts::PI / 180.0)),
    ));
    
    // 界
    commands.spawn((
        ChessBoard,
        Text2d::new("界"),
        TextFont {
            font: fonts.noto_sans_sc.clone(),
            font_size: TEXT_SIZE,
            ..default()
        },
        TextColor(LINE_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(jie_pos.x, jie_pos.y, 1.0)
            .with_rotation(Quat::from_rotation_z(-90.0 * std::f32::consts::PI / 180.0)),
    ));
    
} 