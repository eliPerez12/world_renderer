#![allow(dead_code)]

use std::collections::HashMap;
use macroquad::prelude::*;
use crate::{assets::{atlas_lookup::TILE_SIZE, AssetHandle}, utils::get_atlas_rect};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Clone)]
pub enum Tile {
    Grass,
    Stone,
    Sand,
    Water,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub tiles: Vec<Tile>
}

pub struct World {
    pub chunks: HashMap<ChunkPos, Chunk>
}

#[allow(dead_code)]
pub enum WorldGenerationType {
    WaterWorld,
    ChunkMess,
    TileMess,
}

pub enum WorldGenerationSize {
    Tiny,
    Small,
    Medium,
    Large,
}

#[derive(Debug, PartialEq)]
pub struct GlobalTilePos(pub i32, pub i32);

// Standered functions for creating and modifying world
impl World {

    // Returns an empty world
    pub fn new() -> Self {
        return World { chunks: HashMap::new() };
    }

    pub fn contains_tile(&self, global_pos: &GlobalTilePos) -> bool {
        match self.get_tile(global_pos) {
            Some(_) => true,
            None => false,
        }
    }

    // Populates a world with tiles, with diffrent world types able to be generated
    pub fn generate_world(self, generation_type: WorldGenerationType, size: WorldGenerationSize, seed: u32) -> Self {
        let chunks = match generation_type {
            WorldGenerationType::WaterWorld => Self::generate_water_world(size, seed),
            WorldGenerationType::ChunkMess => Self::generate_chunk_mess_world(size, seed),
            WorldGenerationType::TileMess => Self::generate_tile_mess_world(size, seed),
        };

        return World { chunks };
    }

    // Gets immutable referance to tile from global tile position
    pub fn get_tile(&self, pos: &GlobalTilePos) -> Option<&Tile> {
        let chunk_x = if pos.0 < 0 { (pos.0 - 15) / 16 } else { pos.0 / 16 };
        let chunk_y = if pos.1 < 0 { (pos.1 - 15) / 16 } else { pos.1 / 16 };
        
        let tile_x = pos.0 % 16;
        let tile_y = pos.1 % 16;
        if let Some(chunk) = self.chunks.get(&ChunkPos { x: chunk_x, y: chunk_y }) {
            if let Some(tile) = chunk.tiles.get({tile_x + tile_y * 16} as usize) {
                return Some(tile)
            }
        }
        None
    }

    // Gets mutable referance to tile from global tile position
    pub fn get_tile_mut(&mut self, pos: &GlobalTilePos) -> Option<&mut Tile> {
        let chunk_x = if pos.0 < 0 { (pos.0 - 15) / 16 } else { pos.0 / 16 };
        let chunk_y = if pos.1 < 0 { (pos.1 - 15) / 16 } else { pos.1 / 16 };
        
        let tile_x = pos.0 % 16;
        let tile_y = pos.1 % 16;
        if let Some(chunk) = self.chunks.get_mut(&ChunkPos { x: chunk_x, y: chunk_y }) { // If tile exists in world
            if let Some(tile) = chunk.tiles.get_mut({tile_x + tile_y * 16} as usize) {
                return Some(tile)
            }
        }
        None
    }

    // Gets mutable referance to tile from mouse position
    pub fn get_tile_mut_mouse(&mut self, camera: &Camera2D) -> Option<&mut Tile> {
        let grid_pos = camera.screen_to_world(mouse_position().into()) / TILE_SIZE;
        let tile = self.get_tile_mut(&GlobalTilePos(grid_pos.x as i32, -grid_pos.y as i32));
        match tile {
            Some(tile) => return Some(tile),
            None => (),
        };
        None
    }

    fn get_visible_tiles(&self, camera: &Camera2D) -> Vec<GlobalTilePos> {
        let top_left_world_pos = camera.screen_to_world(Vec2 {x: 0.0, y: 0.0});
        let bottom_right_world_pos = camera.screen_to_world(
            Vec2 {x: screen_width(), y: screen_height()}
        );
        let top_left_grid_pos = top_left_world_pos / 8.0;
        let bottom_right_grid_pos = bottom_right_world_pos / 8.0;
        let mut visible_tiles: Vec<GlobalTilePos> = Vec::new();
        for y in -top_left_grid_pos.y as i32 ..=-bottom_right_grid_pos.y as i32 + 2 {
            for x in top_left_grid_pos.x as i32 ..=bottom_right_grid_pos.x as i32 + 2 {
                visible_tiles.push(GlobalTilePos(x, y));
            }
        }
        visible_tiles
    }
}



// Rendering for tiles
impl World {

    // Renders tiles that are visble to the camera
    pub fn render_visible_tiles(&self, camera: &Camera2D, asset_handle: &AssetHandle) {
        let visible_tiles = self.get_visible_tiles(&camera);
        for global_pos in &visible_tiles {
            if self.contains_tile(global_pos) {
                self.render_tile(&asset_handle, global_pos);
            }
        }
    }

    // Renders a tile given a global_position
    fn render_tile(&self, asset_handle: &AssetHandle, global_pos: &GlobalTilePos) {
        let (x, y) = (global_pos.0, global_pos.1);
        // If tile exists, draw it
        if let Some(tile) = self.get_tile(global_pos) {
            draw_texture_ex(    
                asset_handle.tile_atlas.0, 
                {x as f32 * TILE_SIZE}.round(),
                {-y as f32 * TILE_SIZE}.round() - TILE_SIZE, 
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    source: Some(get_atlas_rect(tile)),
                    flip_y: true,
                    ..Default::default()
                }
            );
        } else {

        }
    }
}