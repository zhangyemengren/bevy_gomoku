use bevy::prelude::*;
use crate::components::{Stone, StoneColor};
use crate::game::{GameState, check_win};
use crate::resources::constants::*;
use crate::utils::{grid_to_world, world_to_grid};

/// Initialize game state
pub fn setup_game(mut commands: Commands) {
    commands.insert_resource(GameState::default());
}

/// Handle mouse clicks to place stones
pub fn handle_mouse_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    stones: Query<&Stone>,
) {
    // Don't process clicks if game is over
    if game_state.game_over {
        return;
    }

    // Check for left mouse click
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    // Get cursor position
    let Ok(window) = windows.single() else { return };
    let Some(cursor_position) = window.cursor_position() else { return };

    // Get camera for coordinate transformation
    let Ok((camera, camera_transform)) = cameras.single() else { return };

    // Convert cursor position to world coordinates
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return };

    // Calculate intersection with board plane (z=0)
    let plane_normal = Vec3::Z;
    let denominator = ray.direction.dot(plane_normal);
    
    if denominator.abs() < 0.0001 {
        return; // Ray is parallel to plane
    }

    let t = (Vec3::ZERO - ray.origin).dot(plane_normal) / denominator;
    if t < 0.0 {
        return; // Intersection is behind camera
    }

    let world_position = ray.origin + ray.direction * t;

    // Convert to grid coordinates
    let Some((grid_x, grid_y)) = world_to_grid(world_position.xy()) else { return };

    // Check if position is already occupied
    if stones.iter().any(|stone| stone.x == grid_x && stone.y == grid_y) {
        return;
    }

    // Update board state
    game_state.board[grid_y as usize][grid_x as usize] = Some(game_state.current_player);

    // Create stone entity
    create_stone(
        &mut commands,
        &mut meshes,
        &mut materials,
        game_state.current_player,
        grid_x,
        grid_y,
    );

    // Check for win
    if check_win(&game_state.board, grid_x, grid_y, game_state.current_player) {
        info!("Player {:?} wins!", game_state.current_player);
        game_state.game_over = true;
        game_state.winner = Some(game_state.current_player);
    } else {
        // Switch player
        game_state.current_player = game_state.current_player.opposite();
    }
}

/// Create a stone entity at the given position
fn create_stone(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: StoneColor,
    x: i32,
    y: i32,
) {
    let world_pos = grid_to_world(x, y);
    
    commands.spawn((
        Stone { x, y },
        color,
        Mesh2d(meshes.add(Circle::new(PIECE_SIZE / 2.0))),
        MeshMaterial2d(materials.add(color.color())),
        Transform::from_xyz(world_pos.x, world_pos.y, 2.0),
    ));
}