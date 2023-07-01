use macroquad::prelude::*;
use atlas_lookup::*;
use assets::*;
use world::*;

mod assets;
mod world;

#[macroquad::main("Rendering tests")]
async fn main() {
    // Initilizing game
    let asset_handle: AssetHandle = AssetHandle::new();

    let mut camera = make_camera();

    let world = World::new().generate_world(
        WorldGenerationType::SimpleTerrain,  
        WorldGenerationSize::Small,
        1000
    );

    let mut camera_zoom_offset = 1.0;

    // Main Game loop
    loop {
        /* Update */
        camera.zoom = vec2(1.0 / screen_width() * TILE_SIZE, 1.0 / screen_height() * TILE_SIZE);
        camera.zoom *= camera_zoom_offset;
        handle_camera_controls(&mut camera, &mut camera_zoom_offset);

        /* Render */
        set_camera(&camera);
        render_entire_world(&world, &asset_handle);

        // UI
        set_default_camera(); // Sets camera to defualt camera, used for ui rendering.
        draw_text(format!("FPS: {}", get_fps()).as_str(), 50.0, 50.0, 50.0, WHITE); // Draws fps
        next_frame().await;
    }
}



fn handle_camera_controls(camera: &mut Camera2D, zoom_offset: &mut f32) {
    let camera_speed = 100.0;
    let zoom_speed: f32 = 0.01;
    let mut max_camera_zoom = 0.3; // Max as in zoomed in, smaller number means wider view
    let mut min_camera_zoom = 4.0; // These are not actually mutable, they are like that so they can interact with the zoom offset better
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

    let mouse_wheel_delta = mouse_wheel().1;
    if mouse_wheel().1 > 0.0 {
        *zoom_offset *= zoom_speed * mouse_wheel_delta.abs();
    }
    if mouse_wheel().1 < 0.0 {
        *zoom_offset /= zoom_speed * mouse_wheel_delta.abs();
    }

    // Normilize zoom
    if zoom_offset > &mut min_camera_zoom {
        *zoom_offset = min_camera_zoom;
    }
    if zoom_offset < &mut max_camera_zoom {
        *zoom_offset = max_camera_zoom;
    }
}

fn render_entire_world(world: &World, asset_handle: &AssetHandle) {
    for (chunk_pos, chunk) in world.chunks.iter() {
        for y in 0..16 {
            for x in 0..16 {
                draw_texture_ex(    
                    asset_handle.tile_atlas.0, 
                    {(chunk_pos.x as f32 * TILE_SIZE * 16.0) + x as f32 * TILE_SIZE}.round(),
                    {(-chunk_pos.y as f32 * TILE_SIZE * 16.0) - y as f32 * TILE_SIZE}.round(),
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        source: Some(get_atlas_rect(chunk.tiles.get(x + y * 16).unwrap())),
                        flip_y: true,
                        ..Default::default()
                    }
                );
            }
        }
    }
}

fn make_camera() -> Camera2D {
    Camera2D {
        zoom: vec2(1.0 / screen_width() * TILE_SIZE, 1.0 / screen_height() * TILE_SIZE),
        target: vec2(0.0, 0.0),
        render_target: None,
        offset: vec2(0.0, 0.0),
        rotation: 0.0,
        viewport: None,
    }
}

fn get_atlas_rect(tile: &Tile) -> Rect{
    match tile {
        Tile::Grass => *atlas_lookup::TILE_GRASS,
        Tile::Water => *atlas_lookup::TILE_WATER,
        Tile::Stone => *atlas_lookup::TILE_STONE,
        Tile::Sand  => *atlas_lookup::TILE_SAND,
    }
}