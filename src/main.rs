use macroquad::prelude::*;
use assets::*;
use world::*;
use utils::*;

mod assets;
mod world;
mod utils;
mod world_generation;

#[macroquad::main("Rendering tests")]
async fn main() {
    // Initilizing game
    let asset_handle: AssetHandle = AssetHandle::new();

    let mut camera = make_camera();

    let mut world = World::new().generate_world(
        WorldGenerationType::TileMess,  
        WorldGenerationSize::Medium,
        1797390122,
    );

    let mut camera_zoom_offset = 8.0;


    // Main Game loop
    loop {
        /* Update */
        camera.zoom = vec2(1.0 / screen_width(), 1.0 / screen_height());
        camera.zoom *= camera_zoom_offset;
        handle_camera_controls(&mut camera, &mut camera_zoom_offset);
        handle_camera_tile_edits(&camera, &mut world);

        /* Render */
        set_camera(&camera);
        world.render_visible_tiles(&camera, &asset_handle);

        // UI
        set_default_camera(); // Sets camera to defualt camera, used for ui rendering.
        draw_text(format!("FPS: {}", get_fps()).as_str(), 50.0, 50.0, 50.0, WHITE); // Draws fps
        next_frame().await;
    }
}
