use std::collections::HashMap;
use macroquad::rand::RandomRange;


#[derive(PartialEq, Eq, Hash, Debug)]
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

impl Tile {
}

#[derive(Debug)]
pub struct Chunk {
    pub tiles: Vec<Tile>
}

pub struct World {
    pub chunks: HashMap<ChunkPos, Chunk>
}

pub enum WorldGenerationType {
    WaterWorld,
    ChunkMess,
    TileMess,
}

pub enum WorldGenerationSize {
    Tiny,
}

impl World {

    // Returns an empty world
    pub fn new() -> Self {
        return World { chunks: HashMap::new() };
    }

    // Helper function for getting size of world
    fn get_size_from_type(size: WorldGenerationSize) -> i32 {
        match size {
            WorldGenerationSize::Tiny => 3,
        }
    }

    // Populates a world with tiles, with diffrent world types able to be generated
    pub fn generate_world(self, generation_type: WorldGenerationType, size: WorldGenerationSize) -> Self {
        let chunks = match generation_type {
            WorldGenerationType::WaterWorld => Self::generate_water_world(size),
            WorldGenerationType::ChunkMess => Self::generate_chunk_mess_world(size),
            WorldGenerationType::TileMess => Self::generate_tile_mess_world(size),
        };

        return World { chunks };
    }
    
}

// Wolrd generation methods
impl World{

    // Generates world of just water tiles
    fn generate_water_world(size: WorldGenerationSize) -> HashMap<ChunkPos, Chunk> {
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

    // Generates world of randomized chunks filled with one type of tile
    fn generate_chunk_mess_world(size: WorldGenerationSize) -> HashMap<ChunkPos, Chunk> {
        let world_size = Self::get_size_from_type(size);
        let mut chunks = HashMap::new();
        
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                // Randomize tile used for chunk
                let current_chunk_tile = match RandomRange::gen_range(1, 5) {
                    1 => Tile::Water,
                    2 => Tile::Grass,
                    3 => Tile::Stone,
                    4 => Tile::Sand,
                    _ =>  unreachable!()
                };
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
    fn generate_tile_mess_world(size: WorldGenerationSize) -> HashMap<ChunkPos, Chunk> {
        let world_size = Self::get_size_from_type(size);
        let mut chunks = HashMap::new();
        
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                let mut current_chunk = Vec::new();
                for _ in 0..16*16 {
                    let current_chunk_tile = match RandomRange::gen_range(1, 5) {
                        1 => Tile::Water,
                        2 => Tile::Grass,
                        3 => Tile::Stone,
                        4 => Tile::Sand,
                        _ =>  unreachable!()
                    };
                    current_chunk.push(current_chunk_tile.clone());
                }
                // Insert chunk into chunks hashmap
                chunks.insert(ChunkPos {x: chunk_x, y: chunk_y}, Chunk { tiles: current_chunk });
            }
        }
        return chunks;
    }

}