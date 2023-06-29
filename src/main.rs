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
        WorldGenerationType::TileMess,
        WorldGenerationSize::Small,
    );


    // Main Game loop
    loop {
        /* Update */
        camera.zoom = vec2(1.0 / screen_width() * TILE_SIZE, 1.0 / screen_height() * TILE_SIZE);
        camera.zoom *= 0.2;
        handle_camera_controls(&mut camera);


        /* Render */
        set_camera(&camera);
        render_entire_world(&world, &asset_handle);

        // UI
        set_default_camera(); // Sets camera to defualt camera, used for ui rendering.
        draw_text(&get_fps().to_string(), 50.0, 50.0, 50.0, YELLOW);
        next_frame().await;
    }
}



fn handle_camera_controls(camera: &mut Camera2D) {
    let camera_speed = 100.0;
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

    if mouse_wheel().1 > 0.0 {

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
