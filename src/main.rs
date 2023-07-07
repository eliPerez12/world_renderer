use assets::*;
use macroquad::prelude::*;
use utils::*;
use world::*;

mod assets;
mod utils;
mod world;
mod world_generation;

#[macroquad::main("Rendering tests")]
async fn main() {
    // Initilizing game
    let asset_handle: AssetHandle = AssetHandle::new();
    let mut camera = make_camera();
    let mut world = World::new().generate_world(
        WorldGenerationType::PerlinTerrain,
        WorldGenerationSize::Large,
        WorldIslandSize::Large,
        25,
    );

    set_fullscreen(true);

    let mut camera_zoom_offset = 8.0;

    // Main Game loop
    loop {
        // Update game
        camera.zoom = vec2(1.0 / screen_width(), 1.0 / screen_height());
        camera.zoom *= camera_zoom_offset;
        handle_camera_controls(&mut camera, &mut camera_zoom_offset);
        handle_camera_tile_edits(&camera, &mut world);

        // Render in world space
        set_camera(&camera);
        world.render_visible_tiles(&camera, &asset_handle);

        // Render in ui space
        set_default_camera(); // Sets camera to default camera, used for ui rendering.
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            50.0,
            50.0,
            50.0,
            WHITE,
        ); // Draws fps
        next_frame().await;
    }
}
