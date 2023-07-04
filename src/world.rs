#![allow(dead_code)]

use std::{collections::HashMap, ops::Neg};
use macroquad::prelude::*;

use crate::assets::atlas_lookup::TILE_SIZE;

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
    SimpleTerrain
}

pub enum WorldGenerationSize {
    Tiny,
    Small,
    Medium,
    Large,
}

pub struct GlobalTilePos(pub i32, pub i32);

// Standered functions for creating and modifying world
impl World {

    // Returns an empty world
    pub fn new() -> Self {
        return World { chunks: HashMap::new() };
    }

    // Gets mutable referance to tile from global tile position
    pub fn get_tile_mut(&mut self, pos: GlobalTilePos) -> Option<&mut Tile> {
        let chunk_x = pos.0/16;
        let chunk_y = pos.1/16;
        let tile_x = pos.0 % 16;
        let tile_y = pos.1 % 16;
        if let Some(chunk) = self.chunks.get_mut(&ChunkPos { x: chunk_x, y: chunk_y }) {
            if let Some(tile) = chunk.tiles.get_mut({tile_x + tile_y * 16} as usize) {
                return Some(tile)
            }
        }
        None
    }

    // Gets mutable referance to tile from mouse_position
    pub fn get_tile_mut_mouse(&mut self, camera: &Camera2D) -> Option<&mut Tile> {
        let grid_pos = camera.screen_to_world(mouse_position().into()) / TILE_SIZE;
        let tile = self.get_tile_mut(GlobalTilePos(grid_pos.x as i32, -grid_pos.y as i32));
        match tile {
            Some(tile) => return Some(tile),
            None => (),
        };
        None
    }


    // Populates a world with tiles, with diffrent world types able to be generated
    pub fn generate_world(self, generation_type: WorldGenerationType, size: WorldGenerationSize, seed: u32) -> Self {
        let chunks = match generation_type {
            WorldGenerationType::WaterWorld => Self::generate_water_world(size, seed),
            WorldGenerationType::ChunkMess => Self::generate_chunk_mess_world(size, seed),
            WorldGenerationType::TileMess => Self::generate_tile_mess_world(size, seed),
            WorldGenerationType::SimpleTerrain => Self::generate_terrain_grass(size, seed),
        };

        return World { chunks };
    }
}