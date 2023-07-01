use std::collections::HashMap;
use macroquad::prelude::Camera2D;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

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

// Impls

impl Chunk {
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y * 16 + x)
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(y * 16 + x)
    }
}

impl World {

    // Returns an empty world
    pub fn new() -> Self {
        return World { chunks: HashMap::new() };
    }

    // Helper function for getting size of world
    fn get_size_from_type(size: WorldGenerationSize) -> i32 {
        match size {
            WorldGenerationSize::Tiny => 2,
            WorldGenerationSize::Small => 4,
            WorldGenerationSize::Medium => 8,
            WorldGenerationSize::Large => 20,
        }
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

    fn get_tile_from_chunk(&self, chunk_pos: ChunkPos, tile_x: usize, tile_y: usize) -> Option<&Tile> {
        self.chunks.get(&chunk_pos)?.get_tile(tile_x, tile_y)
    }

    fn get_tile_from_chunk_mut(&mut self, chunk_pos: ChunkPos, tile_x: usize, tile_y: usize) -> Option<&mut Tile> {
        self.chunks.get_mut(&chunk_pos)?.get_tile_mut(tile_x, tile_y)
    }
    
}

// Wolrd generation methods
impl World{

    // Generates world of just water tiles
    fn generate_water_world(size: WorldGenerationSize, _seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = Self::get_size_from_type(size);
        let mut chunks = HashMap::new();
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                let mut current_chunk = Vec::new();
                for _ in 0..16*16 {
                    current_chunk.push(Tile::Water);
                }
                // Insert chunk into chunks hashmap
                chunks.insert(ChunkPos {x: chunk_x, y: chunk_y}, Chunk { tiles: current_chunk });
            }
        }
        return chunks;
    }

    fn random_tile(rng: &mut StdRng) -> Tile{
        match rng.gen_range(1..5) {
            1 => Tile::Water,
            2 => Tile::Grass,
            3 => Tile::Stone,
            4 => Tile::Sand,
            _ =>  unreachable!()
        }
    }

    // Generates either grass or water tile
    fn random_tile_water_grass(rng: &mut StdRng) -> Tile {
        match rng.gen_range(1..=2) {
            1 => Tile::Water,
            2 => Tile::Grass,
            _ => unreachable!(),
        }
    }

    // Generates world of randomized chunks filled with one type of tile
    fn generate_chunk_mess_world(size: WorldGenerationSize, seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = Self::get_size_from_type(size);
        let mut chunks = HashMap::new();

        let seed = seed_to_byte_array(seed);
        let mut rng = StdRng::from_seed(seed);
        
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                // Randomize tile used for chunk
                let current_chunk_tile = Self::random_tile(&mut rng);

                let mut current_chunk = Vec::new();
                for _ in 0..16*16 {
                    current_chunk.push(current_chunk_tile.clone());
                }
                // Insert chunk into chunks hashmap
                chunks.insert(ChunkPos {x: chunk_x, y: chunk_y}, Chunk { tiles: current_chunk });
            }
        }
        return chunks;
    }

    // Generates world of randomized tiles
    fn generate_tile_mess_world(size: WorldGenerationSize, seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = Self::get_size_from_type(size);
        let mut chunks = HashMap::new();

        let seed = seed_to_byte_array(seed);
        let mut rng = StdRng::from_seed(seed);
        
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                let mut current_chunk = Vec::new();
                for _ in 0..16*16 {
                    let current_chunk_tile = Self::random_tile(&mut rng);
                    current_chunk.push(current_chunk_tile.clone());
                }
                // Insert chunk into chunks hashmap
                chunks.insert(ChunkPos {x: chunk_x, y: chunk_y}, Chunk { tiles: current_chunk });
            }
        }
        return chunks;
    }

    // Generates world of just grass and water, using cellular automata.
    fn generate_terrain_grass(size: WorldGenerationSize, seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = Self::get_size_from_type(size);
        let mut chunks = HashMap::new();

        let seed = seed_to_byte_array(seed);
        let mut rng = StdRng::from_seed(seed);
        
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                let mut current_chunk = Vec::new();
                for _ in 0..16*16 {
                    let current_chunk_tile = Self::random_tile_water_grass(&mut rng);
                    current_chunk.push(current_chunk_tile.clone());
                }
                // Insert chunk into chunks hashmap
                chunks.insert(ChunkPos {x: chunk_x, y: chunk_y}, Chunk { tiles: current_chunk });
            }
        }

        // Normilize tiles by checking neighbors
        let chunks_clone = chunks.clone();

        for (chunkpos, chunk) in chunks {
            for (index, tile) in chunk.tiles.iter().enumerate() {
                match tile {
                    &Tile::Grass => (),
                    _ => continue,
                };
                let mut neighbors = 0;
            }
        }
        

        return chunks_clone;
    }
}


// Helper funtion for generating seed byte array 
fn seed_to_byte_array(seed: u32) -> [u8; 32] {
    let bytes = seed.to_be_bytes();
    let mut byte_array: [u8; 32] = [0; 32];
    for (index, _) in bytes.iter().enumerate() {
    byte_array[index] = bytes[index];
    }
    return byte_array;
}
