use bevy::prelude::*;
use crate::components::*;
use crate::resources::constants::*;

pub fn setup_background(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,) {

    // 绘制棋盘背景
    commands.spawn((
       Mesh2d(meshes.add(Rectangle::new(BOARD_WIDTH, BOARD_HEIGHT))),
        MeshMaterial2d(materials.add(BACKGROUND_COLOR)),
        Transform::default(),
    ));
    

    // 绘制横线
    for i in 0..10 {
        let y = BOARD_SIZE.y / 2.0 - i as f32 * (BOARD_SIZE.y / 9.0);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(BOARD_WIDTH, LINE_WIDTH))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(0.0, y, 1.0),
        ));
    }
    // for i in 0..10 {
    //     let y = BOARD_SIZE.y / 2.0 - i as f32 * (BOARD_SIZE.y / 9.0);
    //     commands.spawn((
    //         Sprite {
    //             color: LINE_COLOR,
    //             custom_size: Some(Vec2::new(BOARD_SIZE.x, 2.0)),
    //             ..default()
    //         },
    //         Transform::from_xyz(0.0, y, 1.0),
    //     ));
    // }

    // 绘制竖线
    // for i in 0..9 {
    //     let x = -BOARD_SIZE.x / 2.0 + i as f32 * (BOARD_SIZE.x / 8.0);
    //     commands.spawn((
    //         Sprite {
    //             color: LINE_COLOR,
    //             custom_size: Some(Vec2::new(2.0, BOARD_SIZE.y)),
    //             ..default()
    //         },
    //         Transform::from_xyz(x, 0.0, 1.0),
    //     ));
    // }

    // 绘制米字格（九宫）
    // for positions in PALACE_POSITIONS.iter() {
    //     let (start_x, start_y) = positions[0];
    //     let (end_x, end_y) = positions[1];
        
    //     let start_pos = Vec2::new(
    //         -BOARD_SIZE.x / 2.0 + start_x as f32 * (BOARD_SIZE.x / 8.0),
    //         BOARD_SIZE.y / 2.0 - start_y as f32 * (BOARD_SIZE.y / 9.0),
    //     );
    //     let end_pos = Vec2::new(
    //         -BOARD_SIZE.x / 2.0 + end_x as f32 * (BOARD_SIZE.x / 8.0),
    //         BOARD_SIZE.y / 2.0 - end_y as f32 * (BOARD_SIZE.y / 9.0),
    //     );

    //     let angle = (end_pos - start_pos).angle_to(Vec2::X);
    //     let length = (end_pos - start_pos).length();

    //     commands.spawn((
    //         Sprite {
    //             color: LINE_COLOR,
    //             custom_size: Some(Vec2::new(length, 2.0)),
    //             ..default()
    //         },
    //         Transform::from_xyz(
    //             (start_pos.x + end_pos.x) / 2.0,
    //             (start_pos.y + end_pos.y) / 2.0,
    //             1.0,
    //         )
    //         .with_rotation(Quat::from_rotation_z(angle)),
    //     ));
    // }

    // 绘制棋子放置点
    // for (x, y) in POINT_POSITIONS.iter() {
    //     let pos_x = -BOARD_SIZE.x / 2.0 + *x as f32 * (BOARD_SIZE.x / 8.0);
    //     let pos_y = BOARD_SIZE.y / 2.0 - *y as f32 * (BOARD_SIZE.y / 9.0);

    //     // 绘制四个小线段组成的点
    //     for i in 0..4 {
    //         let (offset_x, offset_y) = match i {
    //             0 => (-POINT_SIZE, 0.0),  // 左
    //             1 => (POINT_SIZE, 0.0),   // 右
    //             2 => (0.0, -POINT_SIZE),  // 下
    //             3 => (0.0, POINT_SIZE),   // 上
    //             _ => (0.0, 0.0),
    //         };

    //         commands.spawn((
    //             PlacementPoint,
    //             Sprite {
    //                 color: LINE_COLOR,
    //                 custom_size: Some(Vec2::new(POINT_SIZE, 2.0)),
    //                 ..default()
    //             },
    //             Transform::from_xyz(pos_x + offset_x/2.0, pos_y + offset_y/2.0, 1.0)
    //                 .with_rotation(Quat::from_rotation_z(
    //                     if i < 2 { 0.0 } else { std::f32::consts::PI / 2.0 }
    //                 )),
    //         ));
    //     }
    // }

    // 添加楚河汉界文字
    // commands.spawn((
    //     Text::new("楚河"),
    //     TextColor(LINE_COLOR),
    //     Transform::from_xyz(-BOARD_SIZE.x/6.0, 0.0, 1.0),
    // ));

    // commands.spawn((
    //     Text::new("汉界"),
    //     TextColor(LINE_COLOR),
    //     Transform::from_xyz(BOARD_SIZE.x/6.0, 0.0, 1.0),
    // ));
} 