use bevy::prelude::*;
use crate::resources::constants::*;
use crate::resources::GameFonts;
use crate::utils::grid_to_world;

pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    fonts: Res<GameFonts>
) {
    // Draw board background
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(BG_WIDTH, BG_HEIGHT))),
        MeshMaterial2d(materials.add(BACKGROUND_COLOR)),
        Transform::default(),
    ));

    // Draw grid lines - 15x15 board
    for i in 0..15 {
        let y = BOARD_SIZE.y / 2.0 - i as f32 * GRID_SIZE;
        let x = -BOARD_SIZE.x / 2.0 + i as f32 * GRID_SIZE;
        
        // Horizontal line
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(BOARD_WIDTH, LINE_WIDTH))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(0.0, y, 1.0),
        ));
        
        // Vertical line
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, BOARD_HEIGHT))),
            MeshMaterial2d(materials.add(LINE_COLOR)),
            Transform::from_xyz(x, 0.0, 1.0),
        ));
    }

    // Draw star points
    for &(x, y) in STAR_POSITIONS.iter() {
        let pos = grid_to_world(x, y);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(POINT_SIZE / 2.0))),
            MeshMaterial2d(materials.add(POINT_COLOR)),
            Transform::from_xyz(pos.x, pos.y, 1.0),
        ));
    }

    // Add title text
    commands.spawn((
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