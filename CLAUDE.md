# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Build and Run
```bash
# Run the game in development mode
cargo run

# Build the game in release mode (optimized)
cargo build --release

# Run in release mode
cargo run --release
```

### Code Quality
```bash
# Format all Rust code
cargo fmt

# Check for common mistakes and lint issues
cargo clippy

# Check if the project compiles without building
cargo check
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output displayed
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

## Architecture Overview

This is a Gomoku (Five in a Row) game built with the Bevy game engine using the Entity Component System (ECS) pattern.

### Project Structure

```
src/
├── main.rs           # Application entry point
├── components/       # ECS components
│   └── chess_piece.rs   # Stone and StoneColor components
├── systems/          # Game systems
│   ├── background.rs    # Board rendering
│   ├── camera.rs        # Camera setup
│   └── game.rs          # Game logic and input handling
├── resources/        # Game resources
│   ├── constants.rs     # Visual and gameplay constants
│   └── fonts.rs         # Font loading
├── game/            # Game state management
│   └── mod.rs          # GameState and win checking
└── utils/           # Utility functions
    └── mod.rs          # Coordinate transformations
```

### Core Systems

1. **Startup Systems** (executed once):
   - `load_fonts`: Loads the Noto Sans SC font
   - `setup_camera`: Initializes 2D camera
   - `setup_background`: Renders the 15x15 board with grid lines and star points
   - `setup_game`: Initializes GameState resource

2. **Update Systems** (executed every frame):
   - `handle_mouse_click`: Processes mouse input to place stones and check win conditions

### Key Components

- **Stone**: Marks an entity as a stone with x,y grid coordinates
- **StoneColor**: Enum for Black/White with helper methods for color and opposite

### Resources

- **GameState**: Tracks current player, board state (15x15 array), and win status
- **GameFonts**: Handles font assets

### Coordinate System

- Board uses 15x15 grid (0-14 indices)
- Utility functions in `utils/mod.rs`:
  - `grid_to_world(x, y)`: Convert grid to world coordinates
  - `world_to_grid(pos)`: Convert world to grid coordinates
- Board center (7,7) maps to world origin (0,0)

### Game Rules

1. Black plays first
2. Players alternate placing stones on empty intersections
3. Win by getting 5 stones in a row (horizontal, vertical, or diagonal)
4. Game prevents further moves after a win

### Visual Constants

- Grid size: 40px per cell
- Board: 560x560px (14 cells × 40px)
- Background: 600x600px (with margin)
- Stone size: 36px diameter
- Five star points at traditional Gomoku positions