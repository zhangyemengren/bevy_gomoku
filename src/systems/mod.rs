mod background;
mod camera;
mod game;

pub use background::setup_background;
pub use camera::setup_camera;
pub use game::{setup_game, handle_mouse_click};