use macroquad::prelude::*;
use atlas_lookup::*;
use assets::*;
use world::*;
use utils::*;

mod assets;
mod world;
mod utils;

#[macroquad::main("Rendering tests")]
async fn main() {
    // Initilizing game
    let asset_handle: AssetHandle = AssetHandle::new();

    let mut camera = make_camera();

    let world = World::new().generate_world(
        WorldGenerationType::SimpleTerrain,  
        WorldGenerationSize::Small,
        0
    );

    let (x, y) = get_tile_pos_in_chunk(17);
    dbg!(x, y);

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



