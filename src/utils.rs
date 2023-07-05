use crate::World;
use crate::assets::AssetHandle;
use crate::assets::atlas_lookup::{TILE_SIZE, self};
use crate::world::{Tile, WorldGenerationSize};
use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::rngs::StdRng;


// Helper funtion for generating seed byte array 
pub fn seed_to_byte_array(seed: u32) -> [u8; 32] {
    let bytes = seed.to_be_bytes();
    let mut byte_array: [u8; 32] = [0; 32];
    for (index, _) in bytes.iter().enumerate() {
    byte_array[index] = bytes[index];
    }
    return byte_array;
}

pub fn random_tile(rng: &mut StdRng) -> Tile{
    match rng.gen_range(1..5) {
        1 => Tile::Water,
        2 => Tile::Grass,
        3 => Tile::Stone,
        4 => Tile::Sand,
        _ =>  unreachable!()
    }
}

// Generates either grass or water tile
pub fn random_tile_water_grass(rng: &mut StdRng) -> Tile {
    match rng.gen_range(1..=2) {
        1 => Tile::Water,
        2 => Tile::Grass,
        _ => unreachable!(),
    }
}

pub fn handle_camera_controls(camera: &mut Camera2D, zoom_offset: &mut f32) {
    let camera_speed = 100.0;
    let zoom_speed: f32 = 0.01;
    let mut max_camera_zoom = 1.0; // Max as in zoomed in, smaller number means wider view
    let mut min_camera_zoom = 16.0; // These are not actually mutable, they are like that so they can interact with the zoom offset better
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

pub fn handle_camera_tile_edits(camera: &Camera2D, world: &mut World) {
    if is_key_down(KeyCode::Key1) {
        match world.get_tile_mut_mouse(&camera) {
            Some(tile) => *tile = Tile::Water,
            None => ()
        }
    }
    if is_key_down(KeyCode::Key2) {
        match world.get_tile_mut_mouse(&camera) {
            Some(tile) => *tile = Tile::Grass,
            None => ()
        }
    }
    if is_key_down(KeyCode::Key3) {
        match world.get_tile_mut_mouse(&camera) {
            Some(tile) => *tile = Tile::Sand,
            None => ()
        }
    }
    if is_key_down(KeyCode::Key4) {
        match world.get_tile_mut_mouse(&camera) {
            Some(tile) => *tile = Tile::Stone,
            None => ()
        }
    }
}

pub fn _render_entire_world(world: &World, asset_handle: &AssetHandle) {
    for (chunk_pos, chunk) in world.chunks.iter() {
        for y in 0..16 {
            for x in 0..16 {
                draw_texture_ex(    
                    asset_handle.tile_atlas.0, 
                    {(chunk_pos.x as f32 * TILE_SIZE * 16.0) + x as f32 * TILE_SIZE}.round(),
                    {(-chunk_pos.y as f32 * TILE_SIZE * 16.0) - y as f32 * TILE_SIZE}.round() - TILE_SIZE, 
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

pub fn make_camera() -> Camera2D {
    Camera2D {
        zoom: vec2(1.0 / screen_width() * TILE_SIZE, 1.0 / screen_height() * TILE_SIZE),
        target: vec2(0.0, 0.0),
        render_target: None,
        offset: vec2(0.0, 0.0),
        rotation: 0.0,
        viewport: None,
    }
}

pub fn get_atlas_rect(tile: &Tile) -> Rect{
    match tile {
        Tile::Grass => *atlas_lookup::TILE_GRASS,
        Tile::Water => *atlas_lookup::TILE_WATER,
        Tile::Stone => *atlas_lookup::TILE_STONE,
        Tile::Sand  => *atlas_lookup::TILE_SAND,
    }
}

// Lookup for getting size of the world when generating
pub fn get_size_from_type(size: WorldGenerationSize) -> i32 {
    match size {
        WorldGenerationSize::Tiny => 1,
        WorldGenerationSize::Small => 4,
        WorldGenerationSize::Medium => 8,
        WorldGenerationSize::Large => 20,
    }
}