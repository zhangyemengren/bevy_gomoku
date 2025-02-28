use bevy::prelude::*;

#[derive(Resource)]
pub struct GameFonts {
    pub noto_sans_sc: Handle<Font>,
}

impl GameFonts {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            noto_sans_sc: asset_server.load("fonts/noto_sans_sc.ttf"),
        }
    }
}

pub fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fonts = GameFonts::new(&asset_server);
    commands.insert_resource(fonts);
} 