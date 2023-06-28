use macroquad::prelude::*;
use atlas_lookup::*;

// Spritesheet for the tiles
struct TileAtlas(Texture2D);

pub struct AssetHandle {
    tile_atlas: TileAtlas,
}

impl AssetHandle {
    pub fn new() -> Self {
        let embedded_tile_atlas = Self::load_embedded_asset(include_bytes!("assets\\tiles\\tile_atlas_padded.png"));
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

fn handle_camera_controls(camera: &mut Camera2D) {
    let camera_speed = 50.0;
    if is_key_down(KeyCode::W) {
        camera.target.y += camera_speed * get_frame_time();
    }
    if is_key_down(KeyCode::S) {
        camera.target.y -= camera_speed * get_frame_time();
    }
    if is_key_down(KeyCode::A) {
        camera.target.x -= camera_speed * get_frame_time();
    }
    if is_key_down(KeyCode::D) {
        camera.target.x += camera_speed * get_frame_time();
    }
}

#[macroquad::main("Rendering tests")]
async fn main() {
    let asset_handle = AssetHandle::new();

    let mut camera = Camera2D {
        zoom: vec2(1.0 / screen_width() * TILE_SIZE, 1.0 / screen_height() * TILE_SIZE),
        target: vec2(0.0, 0.0),
        render_target: None,
        offset: vec2(0.0, 0.0),
        rotation: 0.0,
        viewport: None,
    };

    loop {
        /* Update */
        camera.zoom = vec2(1.0 / screen_width() * TILE_SIZE, 1.0 / screen_height() * TILE_SIZE);
        camera.zoom *= 5.5;
        handle_camera_controls(&mut camera);
        set_camera(&camera);

        /* Draw */

        // Tiles
        draw_texture_ex(    
            asset_handle.tile_atlas.0, 
            {0 as f32 * TILE_SIZE}.round(),
            {-0 as f32 * TILE_SIZE}.round(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                source: Some(*TILE_WATER),
                flip_y: true,
                ..Default::default()
            }
        );
        draw_texture_ex(    
            asset_handle.tile_atlas.0,
            {1 as f32 * TILE_SIZE}.round(),
            {-0 as f32 * TILE_SIZE}.round(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                source: Some(*TILE_WATER),
                flip_y: true,
                ..Default::default()
            }
        );
        draw_texture_ex(    
            asset_handle.tile_atlas.0, 
            {0 as f32 * TILE_SIZE}.round(),
            {-1 as f32 * TILE_SIZE}.round(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                source: Some(*TILE_WATER),
                flip_y: true,
                ..Default::default()
            }
        );

        // UI
        set_default_camera();
        draw_text(&get_fps().to_string(), 50.0, 50.0, 50.0, YELLOW);
        next_frame().await;
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
    pub const TILE_SIZE: f32 = 8.0;

    pub static TILE_GRASS: Lazy<Rect> = Lazy::new(|| define_pos_in_atlas(0, 0));
    pub static TILE_WATER: Lazy<Rect> = Lazy::new(|| define_pos_in_atlas(1, 0));
    pub static TILE_SAND:  Lazy<Rect> = Lazy::new(|| define_pos_in_atlas(2, 0));
    pub static TILE_STONE: Lazy<Rect> = Lazy::new(|| define_pos_in_atlas(3, 0));
}


