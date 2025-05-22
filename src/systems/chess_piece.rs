use bevy::prelude::*;
use crate::components::{
    Stone, 
    StoneColor,
    BoardPosition, 
    StoneBackground,
    Selected,
    get_stone_color_value
};
use crate::resources::constants::*;

/// 游戏状态资源 - 用于跟踪当前玩家和游戏状态
#[derive(Resource)]
pub struct GameState {
    /// 当前玩家
    pub current_player: StoneColor,
    /// 游戏是否结束
    pub game_over: bool,
    /// 获胜者
    pub winner: Option<StoneColor>,
    /// 棋盘状态 - 存储每个位置的棋子颜色，None表示空位
    pub board: [[Option<StoneColor>; 15]; 15],
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_player: StoneColor::Black, // 黑棋先行
            game_over: false,
            winner: None,
            board: [[None; 15]; 15],
        }
    }
}

/// 初始化棋子系统
pub fn setup_chess_pieces(
    mut commands: Commands,
) {
    // 初始化游戏状态
    commands.insert_resource(GameState::default());
}

/// 创建棋子实体
fn create_stone(
    commands: &mut Commands,
    stone_color: StoneColor,
    position: (i32, i32),
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    // 计算初始位置
    let x = -BOARD_SIZE.x / 2.0 + position.0 as f32 * GRID_SIZE;
    let y = BOARD_SIZE.y / 2.0 - position.1 as f32 * GRID_SIZE;

    // 获取棋子颜色
    let display_color = get_stone_color_value(stone_color);

    // 创建棋子实体
    let entity = commands.spawn((
        Stone,
        stone_color,
        BoardPosition { position },
        Transform::from_xyz(x, y, 2.0), // 初始变换
        Visibility::Visible,
    )).id();

    // 直接添加视觉表现
    commands.entity(entity).with_children(|parent| {
        // 棋子圆形
        parent.spawn((
            StoneBackground,
            Mesh2d(meshes.add(Circle::new(PIECE_SIZE / 2.0))),
            MeshMaterial2d(materials.add(display_color)),
            Transform::from_xyz(0.0, 0.0, 2.0),
            Visibility::Visible,
        ));
    });

    entity
}

/// 根据棋子位置更新变换组件
pub fn update_piece_transforms(
    mut query: Query<(&BoardPosition, &mut Transform), With<Stone>>,
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

/// 处理棋盘点击事件 - 放置棋子
pub fn handle_chess_piece_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    stones: Query<&BoardPosition, With<Stone>>,
) {
    // 如果游戏已结束，不处理点击
    if game_state.game_over {
        return;
    }

    // 当鼠标左键被点击时
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // 获取主窗口和鼠标位置
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_position) = window.cursor_position() {
                // 获取相机
                if let Ok((camera, camera_transform)) = cameras.get_single() {
                    // 将鼠标位置转换为世界坐标
                    if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                        // 计算与棋盘平面的交点 - 使用自定义计算
                        let plane_normal = Vec3::Z;   // 棋盘朝上的法线
                        let plane_point = Vec3::ZERO; // 棋盘中心点

                        // 手动计算射线与平面的交点
                        let denominator = ray.direction.dot(plane_normal);

                        // 确保射线不与平面平行
                        if denominator.abs() > 0.0001 {
                            let t = (plane_point - ray.origin).dot(plane_normal) / denominator;

                            // 确保交点在射线前方
                            if t >= 0.0 {
                                let world_position = ray.origin + ray.direction * t;

                                // 将世界坐标转换为棋盘格子坐标
                                let grid_x = ((world_position.x + BOARD_SIZE.x / 2.0) / GRID_SIZE).round() as i32;
                                let grid_y = ((BOARD_SIZE.y / 2.0 - world_position.y) / GRID_SIZE).round() as i32;

                                // 检查是否在棋盘范围内
                                if grid_x >= 0 && grid_x < 15 && grid_y >= 0 && grid_y < 15 {
                                    // 检查该位置是否已有棋子
                                    let position_occupied = stones.iter().any(|pos| pos.position == (grid_x, grid_y));

                                    if !position_occupied {
                                        // 在棋盘状态中记录这个位置
                                        game_state.board[grid_y as usize][grid_x as usize] = Some(game_state.current_player);

                                        // 创建新棋子
                                        create_stone(
                                            &mut commands,
                                            game_state.current_player,
                                            (grid_x, grid_y),
                                            &mut meshes,
                                            &mut materials,
                                        );

                                        // 检查是否获胜
                                        if check_win(&game_state.board, grid_x, grid_y, game_state.current_player) {
                                            info!("玩家 {:?} 获胜!", game_state.current_player);
                                            game_state.game_over = true;
                                            game_state.winner = Some(game_state.current_player);
                                        } else {
                                            // 切换玩家
                                            game_state.current_player = match game_state.current_player {
                                                StoneColor::Black => StoneColor::White,
                                                StoneColor::White => StoneColor::Black,
                                            };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 检查是否获胜 - 五子连珠
fn check_win(board: &[[Option<StoneColor>; 15]; 15], x: i32, y: i32, color: StoneColor) -> bool {
    let directions = [
        (1, 0),   // 水平
        (0, 1),   // 垂直
        (1, 1),   // 对角线
        (1, -1),  // 反对角线
    ];

    for (dx, dy) in directions.iter() {
        let mut count = 1; // 当前位置已经有一个棋子

        // 正向检查
        let mut nx = x + dx;
        let mut ny = y + dy;
        while nx >= 0 && nx < 15 && ny >= 0 && ny < 15 {
            if board[ny as usize][nx as usize] == Some(color) {
                count += 1;
                nx += dx;
                ny += dy;
            } else {
                break;
            }
        }

        // 反向检查
        nx = x - dx;
        ny = y - dy;
        while nx >= 0 && nx < 15 && ny >= 0 && ny < 15 {
            if board[ny as usize][nx as usize] == Some(color) {
                count += 1;
                nx -= dx;
                ny -= dy;
            } else {
                break;
            }
        }

        // 如果有5个或更多连续棋子，则获胜
        if count >= 5 {
            return true;
        }
    }

    false
}

/// 更新选中棋子的闪烁效果
pub fn update_piece_selection(
    time: Res<Time>,
    mut selected_pieces: Query<(&mut Selected, &Children)>,
    mut backgrounds: Query<&mut Visibility, With<StoneBackground>>,
) {
    for (mut selected, children) in selected_pieces.iter_mut() {
        // 更新闪烁计时器
        selected.timer.tick(time.delta());

        // 闪烁动画进度 (0.0 - 1.0)
        let progress = selected.timer.elapsed_secs() / selected.timer.duration().as_secs_f32();

        // 使用正弦波函数计算可见性 - 调整为在中间部分闪烁，确保可见时间更长
        // 当sin值小于-0.6时隐藏，其余时间显示（约25%时间隐藏，75%时间显示）
        let visibility_value = (progress * std::f32::consts::PI * 2.0).sin();
        let is_visible = visibility_value > -0.6;

        // 应用可见性更改到棋子组件
        for child in children.iter() {
            // 处理背景圆形
            if let Ok(mut bg_visibility) = backgrounds.get_mut(child) {
                *bg_visibility = if is_visible { Visibility::Visible } else { Visibility::Hidden };
            }
        }

        // 检查是否需要切换闪烁状态（用于其他逻辑）
        if selected.timer.just_finished() {
            selected.is_highlighted = !selected.is_highlighted;
        }
    }
}
