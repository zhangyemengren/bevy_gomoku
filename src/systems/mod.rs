mod background;
mod camera;
mod chess_piece;

pub use background::setup_background;
pub use camera::setup_camera;
pub use chess_piece::{
    setup_chess_pieces, 
    add_piece_appearance, 
    update_piece_transforms, 
    spawn_piece_visuals
};
