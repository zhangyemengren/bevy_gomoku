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
       Mesh2d(meshes.add(Rectangle::new(BOARD_WIDTH, BOARD_HEIGHT))),
       MeshMaterial2d(materials.add(BACKGROUND_COLOR)),
       Transform::default(),
    ));
    
    // 绘制横线
    // 上半部分横线（0-4）
    for i in 0..5 {
        let y = BOARD_SIZE.y / 2.0 - i as f32 * (BOARD_SIZE.y / 9.0);
        commands.spawn((
            ChessBoard,
            Mesh2d(meshes.add(Rectangle::new(BOARD_WIDTH, LINE_WIDTH))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(0.0, y, 1.0),
        ));
    }
    
    // 下半部分横线（5-9）
    for i in 5..10 {
        let y = BOARD_SIZE.y / 2.0 - i as f32 * (BOARD_SIZE.y / 9.0);
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
        
        // 上半部分竖线
        commands.spawn((
            ChessBoard,
            Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, BOARD_HEIGHT / 2.0 - GRID_SIZE / 2.0))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(x, BOARD_SIZE.y / 4.0 + GRID_SIZE / 4.0, 1.0),
        ));
        
        // 下半部分竖线
        commands.spawn((
            ChessBoard,
            Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, BOARD_HEIGHT / 2.0 - GRID_SIZE / 2.0))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(x, -BOARD_SIZE.y / 4.0 - GRID_SIZE / 4.0, 1.0),
        ));
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

    // 绘制棋子放置点
    for (x, y) in POINT_POSITIONS.iter() {
        let pos_x = -BOARD_SIZE.x / 2.0 + *x as f32 * (BOARD_SIZE.x / 8.0);
        let pos_y = BOARD_SIZE.y / 2.0 - *y as f32 * (BOARD_SIZE.y / 9.0);

        // 绘制四个小线段组成的点
        for i in 0..4 {
            let (offset_x, offset_y) = match i {
                0 => (-POINT_SIZE, 0.0),  // 左
                1 => (POINT_SIZE, 0.0),   // 右
                2 => (0.0, -POINT_SIZE),  // 下
                3 => (0.0, POINT_SIZE),   // 上
                _ => (0.0, 0.0),
            };

            commands.spawn((
                PlacementPoint,
                ChessBoard,
                Mesh2d(meshes.add(Rectangle::new(POINT_SIZE, LINE_WIDTH))),
                MeshMaterial2d(materials.add(LINE_COLOR)),
                Transform::from_xyz(pos_x + offset_x/2.0, pos_y + offset_y/2.0, 1.0)
                    .with_rotation(Quat::from_rotation_z(
                        if i < 2 { 0.0 } else { std::f32::consts::PI / 2.0 }
                    )),
            ));
        }
    }

    // 添加楚河汉界文字
    commands.spawn((
        ChessBoard,
        Text2d::new("楚河"),
        TextFont {
            font: fonts.noto_sans_sc.clone(),
            font_size: TEXT_SIZE,
            ..default()
        },
        TextColor(LINE_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(-BOARD_SIZE.x/6.0, 0.0, 1.0),
    ));

    commands.spawn((
        ChessBoard,
        Text2d::new("汉界"),
        TextFont {
            font: fonts.noto_sans_sc.clone(),
            font_size: TEXT_SIZE,
            ..default()
        },
        TextColor(LINE_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(BOARD_SIZE.x/6.0, 0.0, 1.0),
    ));
    
    // 绘制河界区域的边框线
    // 上边界线
    commands.spawn((
        ChessBoard,
        Mesh2d(meshes.add(Rectangle::new(BOARD_WIDTH, LINE_WIDTH))),
        MeshMaterial2d(materials.add(LINE_COLOR)),
        Transform::from_xyz(0.0, GRID_SIZE / 2.0, 1.0),
    ));
    
    // 下边界线
    commands.spawn((
        ChessBoard,
        Mesh2d(meshes.add(Rectangle::new(BOARD_WIDTH, LINE_WIDTH))),
        MeshMaterial2d(materials.add(LINE_COLOR)),
        Transform::from_xyz(0.0, -GRID_SIZE / 2.0, 1.0),
    ));
} 