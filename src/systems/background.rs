use bevy::prelude::*;
use crate::components::*;
use crate::resources::constants::*;
use crate::resources::GameFonts;

pub fn setup_background(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    fonts: Res<GameFonts>) {

    // 绘制棋盘背景
    commands.spawn((
       GoBoard,
       Mesh2d(meshes.add(Rectangle::new(BG_WIDTH, BG_HEIGHT))),
       MeshMaterial2d(materials.add(BACKGROUND_COLOR)),
       Transform::default(),
    ));

    // 外层边框 - 与内层边框线宽相同，但距离为4倍线宽
    let gap = LINE_WIDTH * 4.0; // 内外层边框之间的间距
    let outer_width = BOARD_WIDTH + LINE_WIDTH * 2.0 + gap * 2.0; // 外层边框宽度
    let outer_height = BOARD_HEIGHT + LINE_WIDTH * 2.0 + gap * 2.0; // 外层边框高度

    commands.spawn((
        GoBoard,
        Mesh2d(meshes.add(Rectangle::new(outer_width, outer_height))),
        MeshMaterial2d(materials.add(Color::NONE)),
        Transform::default(),
    )).with_children(|parent| {
        // 上边框
        parent.spawn((
            GoBoard,
            Mesh2d(meshes.add(Rectangle::new(outer_width, LINE_WIDTH))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(0.0, outer_height / 2.0 - LINE_WIDTH / 2.0, 1.0),
        ));

        // 下边框
        parent.spawn((
            GoBoard,
            Mesh2d(meshes.add(Rectangle::new(outer_width, LINE_WIDTH))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(0.0, -outer_height / 2.0 + LINE_WIDTH / 2.0, 1.0),
        ));

        // 左边框
        parent.spawn((
            GoBoard,
            Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, outer_height))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(-outer_width / 2.0 + LINE_WIDTH / 2.0, 0.0, 1.0),
        ));

        // 右边框
        parent.spawn((
            GoBoard,
            Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, outer_height))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(outer_width / 2.0 - LINE_WIDTH / 2.0, 0.0, 1.0),
        ));
    });

    // 绘制横线 - 五子棋有15条横线
    for i in 0..15 {
        // 计算横线位置
        let y = BOARD_SIZE.y / 2.0 - i as f32 * GRID_SIZE;
        commands.spawn((
            GoBoard,
            Mesh2d(meshes.add(Rectangle::new(BOARD_WIDTH, LINE_WIDTH))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(0.0, y, 1.0),
        ));
    }

    // 绘制竖线 - 五子棋有15条竖线
    for i in 0..15 {
        // 计算竖线位置
        let x = -BOARD_SIZE.x / 2.0 + i as f32 * GRID_SIZE;

        commands.spawn((
            GoBoard,
            Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, BOARD_HEIGHT))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(x, 0.0, 1.0),
        ));
    }

    // 绘制五个星位点
    for (x, y) in STAR_POSITIONS.iter() {
        let pos_x = -BOARD_SIZE.x / 2.0 + *x as f32 * GRID_SIZE;
        let pos_y = BOARD_SIZE.y / 2.0 - *y as f32 * GRID_SIZE;

        // 创建星位点（小圆点）
        commands.spawn((
            StarPoint,
            GoBoard,
            Mesh2d(meshes.add(Circle::new(POINT_SIZE / 2.0))),
            MeshMaterial2d(materials.add(POINT_COLOR)),
            Transform::from_xyz(pos_x, pos_y, 1.0),
        ));
    }

    // 添加标题文字
    commands.spawn((
        GoBoard,
        Text2d::new("五子棋"),
        TextFont {
            font: fonts.noto_sans_sc.clone(),
            font_size: TEXT_SIZE,
            ..default()
        },
        TextColor(LINE_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(0.0, -BOARD_SIZE.y / 2.0 - TEXT_SIZE, 1.0),
    ));
}
