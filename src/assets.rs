use macroquad::{
    prelude::ImageFormat,
    texture::{FilterMode, Texture2D},
};

// Spritesheet for the tiles
pub struct TileAtlas(pub Texture2D);

pub struct AssetHandle {
    pub tile_atlas: TileAtlas,
}

impl AssetHandle {
    pub fn new() -> Self {
        let embedded_tile_atlas =
            Self::load_embedded_asset(include_bytes!("assets\\tiles\\tile_atlas_padded.png"));
        AssetHandle {
            tile_atlas: TileAtlas(embedded_tile_atlas),
        }
    }
    pub fn load_embedded_asset(file_bytes: &[u8]) -> Texture2D {
        let texture = Texture2D::from_file_with_format(file_bytes, Some(ImageFormat::Png));
        texture.set_filter(FilterMode::Nearest);
        return texture;
    }
}

// Lookups for atlas / spritesheet.
pub mod atlas_lookup {
    use macroquad::prelude::*;
    use once_cell::sync::Lazy;

    // Calculates position in atlas for tile, with one pixel padding
    fn define_pos_in_atlas(x: i32, y: i32) -> Rect {
        let x = (x as f32 * (TILE_SIZE + 2.0)) + 1.0;
        let y = (y as f32 * (TILE_SIZE + 2.0)) + 1.0;
        Rect::new(x, y, TILE_SIZE, TILE_SIZE)
    }

    // Tiles
    pub const TILE_SIZE: f32 = 8.0; // Tile size in pixels

    pub const TILE_GRASS: Lazy<Rect> = Lazy::new(|| define_pos_in_atlas(0, 0));
    pub const TILE_WATER: Lazy<Rect> = Lazy::new(|| define_pos_in_atlas(1, 0));
    pub const TILE_SAND: Lazy<Rect> = Lazy::new(|| define_pos_in_atlas(2, 0));
    pub const TILE_STONE: Lazy<Rect> = Lazy::new(|| define_pos_in_atlas(3, 0));
    pub const TILE_SHALLOW_WATER: Lazy<Rect> = Lazy::new(|| define_pos_in_atlas(0, 1));
    pub const TILE_DEEP_WATER: Lazy<Rect> = Lazy::new(|| define_pos_in_atlas(1, 1));
    pub const TILE_DARK_STONE: Lazy<Rect> = Lazy::new(|| define_pos_in_atlas(2, 1));
    pub const TILE_SNOW: Lazy<Rect> = Lazy::new(|| define_pos_in_atlas(3, 1));
}
